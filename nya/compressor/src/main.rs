use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use common::expr::{BinOp, Expr};
use itertools::Itertools;
use num_bigint::BigInt;

//////////////////////// BEGIN copipe from tanakh/solver/src/bin/lambdaman.rs

fn varid(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

trait ToExpr {
    fn to_expr(&self) -> Expr;
}

impl ToExpr for BigInt {
    fn to_expr(&self) -> Expr {
        Expr::Int(self.clone().into())
    }
}

impl ToExpr for String {
    fn to_expr(&self) -> Expr {
        Expr::String(self.clone().into())
    }
}

impl ToExpr for &str {
    fn to_expr(&self) -> Expr {
        Expr::String(self.to_string().into())
    }
}

impl ToExpr for i32 {
    fn to_expr(&self) -> Expr {
        Expr::Int(BigInt::from(*self).into())
    }
}

impl ToExpr for Expr {
    fn to_expr(&self) -> Expr {
        self.clone()
    }
}

macro_rules! icfp {
    (fix $($args:tt)+) => {
        icfp! {
            (fn f -> ((fn x -> (f (x x))) (fn x -> (f (x x))))) $($args)*
        }
    };
    (fn $($args:ident)+ -> $body:tt) => {{
        let mut e = icfp!{ $body };
        for arg in [$(stringify!($args)), *].iter().rev() {
            e = Expr::Lambda(varid(arg), Box::new(e));
        }
        e
    }};
    (let $var:ident = $val:tt in $($body:tt)+) => {
        Expr::Bin(BinOp::App,
            Box::new(Expr::Lambda(varid(stringify!($var)), Box::new(icfp!{ $($body)* }))),
            Box::new(icfp!{ $val }))
    };
    (if $cond:tt { $($th:tt)+ } else { $($el:tt)+ }) => {
        Expr::If(Box::new(icfp!{ $cond }), Box::new(icfp!{ $($th)* }), Box::new(icfp!{ $($el)* }))
    };
    (concat $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Concat, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (take $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Take, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (drop $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Drop, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (== $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Eq, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (+ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Add, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (- $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Sub, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (* $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mul, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (/ $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Div, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (% $a1:tt $a2:tt) => {
        Expr::Bin(BinOp::Mod, Box::new(icfp!{ $a1 }), Box::new(icfp!{ $a2 }))
    };
    (# $var:ident) => {
        ToExpr::to_expr(&$var)
    };
    ($f:tt $($args:tt)+) => {{
        let mut e = icfp!{ $f };
        $(
            e = Expr::Bin(BinOp::App, Box::new(e), Box::new(icfp!{ $args }));
        )+
        e
    }};
    ($var:ident) => {
        Expr::Var(varid(stringify!($var)))
    };
    ($val:literal) => {
        ToExpr::to_expr(&$val)
    };
    (($($tt:tt)+)) => {
        icfp! { $($tt)+ }
    };
}

//////////////////////// END copipe from tanakh/solver/src/bin/lambdaman.rs

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, required = true)]
    header: String,

    file: PathBuf,
}

fn push_fixint(out: &mut BigInt, next: &mut BigInt, n: usize, x: usize) {
    assert!(x < n);
    *out += &*next * x;
    *next *= n;
}

fn push_varint(out: &mut BigInt, next: &mut BigInt, x: usize) {
    let mut x = x;
    let mut bits: Vec<usize> = Vec::new();
    while x > 0 {
        bits.push(x & 1);
        x >>= 1;
    }
    for bit in bits.into_iter().rev() {
        push_fixint(out, next, 3, bit);
    }
    push_fixint(out, next, 3, 2);
}

fn pop_fixint(out: &mut BigInt, n: usize) -> usize {
    let x = (&*out % n).try_into().unwrap();
    *out /= n;
    x
}

fn pop_varint(out: &mut BigInt) -> usize {
    let mut x = 0;
    loop {
        let b = pop_fixint(out, 3);
        if b == 2 {
            break;
        }
        x = x << 1 | b;
    }
    x
}

fn find_best_copy(
    chars: &[char],
    current_pos: usize,
    index: &HashMap<Vec<char>, usize>,
    prev_chain: &[isize],
    max_offset: usize,
    max_len: usize,
) -> Option<(usize, usize)> {
    let prefix = &chars[current_pos..(current_pos + 4).min(chars.len() - 1)];
    let cand_pos = index.get(prefix)?;
    let mut cand_pos = *cand_pos as isize;

    // Find the longest match.
    let mut best_offset = current_pos - cand_pos as usize;
    if best_offset > max_offset {
        return None;
    }
    let mut best_len = 0;
    while best_len < max_len
        && current_pos + best_len < chars.len()
        && chars[cand_pos as usize + best_len] == chars[current_pos + best_len]
    {
        best_len += 1;
    }

    cand_pos = prev_chain[cand_pos as usize];
    while cand_pos >= 0 && current_pos - cand_pos as usize <= max_offset && best_len < max_len {
        let mut cand_len = 0;
        while cand_len < max_len
            && current_pos + cand_len < chars.len()
            && chars[cand_pos as usize + cand_len] == chars[current_pos + cand_len]
        {
            cand_len += 1;
        }
        if cand_len > best_len {
            best_len = cand_len;
            best_offset = current_pos - cand_pos as usize;
        }
        cand_pos = prev_chain[cand_pos as usize];
    }

    Some((best_offset, best_len))
}

pub enum CompressedString {
    Raw { text: String },
    Lz { code_to_char: Vec<char>, n: BigInt },
}

impl CompressedString {
    pub fn new(text: &str) -> Self {
        if text.len() <= 10 {
            return Self::Raw {
                text: text.to_string(),
            };
        }

        let chars: Vec<char> = text.chars().collect();
        let code_to_char: Vec<char> = chars.iter().copied().sorted().dedup().collect();
        let char_to_code: HashMap<char, u8> = code_to_char
            .iter()
            .copied()
            .enumerate()
            .map(|(i, c)| (c, i as u8))
            .collect();
        let alphabet = code_to_char.len();

        let max_offset = 256;
        let max_len = 256;

        let mut out = BigInt::ZERO;
        let mut next = BigInt::from(1);
        let mut index: HashMap<Vec<char>, usize> = HashMap::new();
        let mut prev_chain: Vec<isize> = Vec::new();
        let mut current_pos = 0;
        while current_pos < chars.len() {
            match find_best_copy(
                &chars,
                current_pos,
                &index,
                &prev_chain,
                max_offset,
                max_len,
            ) {
                Some((best_offset, best_len)) if best_len >= 8 => {
                    // Emit copy.
                    assert!(best_offset <= max_offset);
                    assert!(best_len <= max_len);
                    push_fixint(&mut out, &mut next, 2, 0);
                    push_varint(&mut out, &mut next, best_offset - 1);
                    push_varint(&mut out, &mut next, best_len - 1);
                    current_pos += best_len;
                    eprintln!("copy {} {}", best_offset, best_len);
                }
                _ => {
                    // Emit literal.
                    let c = chars[current_pos];
                    push_fixint(&mut out, &mut next, 2, 1);
                    push_fixint(&mut out, &mut next, alphabet, char_to_code[&c] as usize);
                    current_pos += 1;
                    eprintln!("literal {}", c);
                }
            }

            while prev_chain.len() < current_pos {
                let n = prev_chain.len();
                let prefix = &chars[n..(n + 4).min(chars.len() - 1)];
                let prev_pos = index.get(prefix).map(|pos| *pos as isize).unwrap_or(-1);
                prev_chain.push(prev_pos);
                index.insert(prefix.to_vec(), n);
            }
        }

        Self::Lz {
            code_to_char,
            n: out,
        }
    }

    pub fn decompress(&self) -> String {
        match self {
            Self::Raw { text } => text.clone(),
            Self::Lz { code_to_char, n } => {
                let mut n = n.clone();
                let mut out: Vec<char> = Vec::new();
                while n > BigInt::ZERO {
                    let b = pop_fixint(&mut n, 2);
                    if b == 0 {
                        // Copy
                        let offset = pop_varint(&mut n) + 1;
                        let len = pop_varint(&mut n) + 1;
                        for _ in 0..len {
                            out.push(out[out.len() - offset]);
                        }
                    } else {
                        // Literal
                        let c = code_to_char[pop_fixint(&mut n, code_to_char.len())];
                        out.push(c);
                    }
                }
                out.into_iter().collect()
            }
        }
    }

    pub fn expr(&self) -> Expr {
        match self {
            CompressedString::Raw { text } => Expr::String(text.clone().into()),
            CompressedString::Lz { code_to_char, n } => {
                let n = n.clone();
                let code_to_char: String = code_to_char.iter().copied().collect();
                let num_chars = code_to_char.len() as i32;
                icfp! {
                    (
                        (fn P L R ->
                            (
                                (fn X ->
                                    (
                                        (fn Y ->
                                            (
                                                // main (n: int, s: str) -> str
                                                fix (fn f n s ->
                                                    (if (== n 0) {
                                                        s
                                                    } else {
                                                        (
                                                            (fn p ->
                                                                (if (== (R p) 0) {
                                                                    // Copy
                                                                    (
                                                                        (fn p ->
                                                                            (
                                                                                (fn p o ->
                                                                                    (
                                                                                        (fn n l ->
                                                                                            999 // TODO
                                                                                        )
                                                                                        (L p)
                                                                                        (R p)
                                                                                    )
                                                                                )
                                                                                (Y (L p))
                                                                                (R p)
                                                                            )
                                                                        )
                                                                        (Y (L p))
                                                                    )
                                                                } else {
                                                                    // Literal
                                                                    (
                                                                        (fn p ->
                                                                            (f (L p) (concat s 999)) // TODO
                                                                        )
                                                                        (X (L p) (#num_chars))
                                                                    )
                                                                })
                                                            )
                                                            (X n 2)
                                                        )
                                                    })
                                                )
                                                (#n)
                                                ""
                                            )
                                        )
                                        // Y: pop_varint(n: int) -> (n': int, x: int)
                                        (
                                            fix (fn f n ->
                                                (
                                                    (fn r m ->
                                                        (
                                                            if (== r 2) {
                                                                (P m 0)
                                                            } else {
                                                                (fn p ->
                                                                    // TODO: Reverse bits
                                                                    (P (L p) (+ (* (R p) 2) r))
                                                                )
                                                                (f m)
                                                            }
                                                        )
                                                    )
                                                    (% n 3)
                                                    (/ n 3)
                                                )
                                            )
                                        )
                                    )
                                )
                                // X: pop_fixint(n: int, b: int) -> (n': int, x: int)
                                (fn n b -> (p (/ n b) (% n b)))
                            )
                        )
                        // P: pair
                        (fn l r f -> (f l r))
                        // L: left
                        (fn p -> (p (fn l r -> l)))
                        // R: right
                        (fn p -> (p (fn l r -> r)))
                    )
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let text = std::fs::read_to_string(args.file)?.trim().to_string();
    // let compressed = compress(&text);
    // println!("{}", compressed.expr(&args.header).encoded());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint() {
        let mut out = BigInt::ZERO;
        let mut next = BigInt::from(1);

        push_varint(&mut out, &mut next, 123);
        push_varint(&mut out, &mut next, 456);
        assert_eq!(pop_varint(&mut out), 123);
        assert_eq!(pop_varint(&mut out), 456);
    }

    #[test]
    fn test_compress_decompress() {
        let text = "bbbbbaaaaabbbbbaaaaaabbbbb";
        let compressed = CompressedString::new(text);
        assert_eq!(compressed.decompress(), text);
    }
}
