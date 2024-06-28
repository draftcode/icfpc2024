use std::{iter::Peekable, str::Chars};

use anyhow::{anyhow, bail};

use crate::base94::{decode_base94, decode_str, encode_base94, encode_base94_int, encode_str};

#[derive(Debug)]
pub enum Token {
    Bool(bool),
    Int(i64),
    String(String),
    Un(UnOp),
    Bin(BinOp),
    If,
    Lambda(usize),
    Var(usize),
}

impl Token {
    pub fn encoded(&self) -> TokenEncoded {
        TokenEncoded(self)
    }
}

pub struct TokenEncoded<'a>(&'a Token);

impl std::fmt::Display for TokenEncoded<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Token::Bool(v) => write!(f, "{}", if *v { "T" } else { "F" }),
            Token::Int(n) => {
                if *n < 0 {
                    return Err(std::fmt::Error);
                }
                write!(f, "I")?;
                if *n == 0 {
                    write!(f, "{}", encode_base94(0).unwrap())?;
                } else {
                    let mut n = *n;
                    let mut chars: Vec<char> = Vec::new();
                    while n > 0 {
                        chars.push(encode_base94(n % 94).unwrap());
                        n /= 94;
                    }
                    for c in chars.into_iter().rev() {
                        write!(f, "{}", c)?;
                    }
                }
                Ok(())
            }
            Token::String(s) => write!(f, "S{}", encode_str(s).map_err(|_| std::fmt::Error)?),
            Token::Un(op) => write!(f, "U{}", op.encoded()),
            Token::Bin(op) => write!(f, "B{}", op.encoded()),
            Token::If => write!(f, "?"),
            Token::Lambda(v) => write!(
                f,
                "L{}",
                encode_base94_int(*v as i64).map_err(|_| std::fmt::Error)?
            ),
            Token::Var(v) => write!(
                f,
                "v{}",
                encode_base94_int(*v as i64).map_err(|_| std::fmt::Error)?
            ),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UnOp {
    Neg,
    Not,
    StrToInt,
    IntToStr,
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            UnOp::Neg => "-",
            UnOp::Not => "!",
            UnOp::StrToInt => "str-to-int",
            UnOp::IntToStr => "int-to-str",
        };
        write!(f, "{op}")
    }
}

impl UnOp {
    pub fn encoded(self) -> UnOpEncoded {
        UnOpEncoded(self)
    }
}

pub struct UnOpEncoded(UnOp);

impl std::fmt::Display for UnOpEncoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.0 {
            UnOp::Neg => "-",
            UnOp::Not => "!",
            UnOp::StrToInt => "#",
            UnOp::IntToStr => "$",
        };
        write!(f, "{op}")
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Gt,
    Eq,
    Or,
    And,
    Concat,
    Take,
    Drop,
    App,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Eq => "=",
            BinOp::Or => "|",
            BinOp::And => "&",
            BinOp::Concat => ".",
            BinOp::Take => "take",
            BinOp::Drop => "drop",
            BinOp::App => "$",
        };
        write!(f, "{op}")
    }
}

impl BinOp {
    pub fn encoded(self) -> BinOpEncoded {
        BinOpEncoded(self)
    }
}

pub struct BinOpEncoded(BinOp);

impl std::fmt::Display for BinOpEncoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.0 {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Eq => "=",
            BinOp::Or => "|",
            BinOp::And => "&",
            BinOp::Concat => ".",
            BinOp::Take => "T",
            BinOp::Drop => "D",
            BinOp::App => "$",
        };
        write!(f, "{op}")
    }
}

struct Cursor<'a>(Peekable<Chars<'a>>);

impl Token {
    pub fn from_str(s: &str) -> anyhow::Result<Token> {
        let mut cur = Cursor(s.chars().peekable());

        let ty = cur.0.next().ok_or_else(|| anyhow!("invalid token"))?;
        Ok(match ty {
            'T' => {
                if cur.0.next().is_some() {
                    bail!("invalid token: {s}")
                }
                Token::Bool(true)
            }
            'F' => {
                if cur.0.next().is_some() {
                    bail!("invalid token: {s}")
                }
                Token::Bool(false)
            }
            'I' => {
                let mut n = 0;
                while let Some(c) = cur.0.next() {
                    n = n * 94 + decode_base94(c)?;
                }
                Token::Int(n)
            }
            'S' => {
                let mut s = String::new();
                while let Some(c) = cur.0.next() {
                    s.push(c);
                }
                Token::String(decode_str(&s)?)
            }
            'U' => match cur.0.next() {
                Some('-') => Token::Un(UnOp::Neg),
                Some('!') => Token::Un(UnOp::Not),
                Some('#') => Token::Un(UnOp::StrToInt),
                Some('$') => Token::Un(UnOp::IntToStr),
                _ => bail!("invalid token: {s}"),
            },
            'B' => match cur.0.next() {
                Some('+') => Token::Bin(BinOp::Add),
                Some('-') => Token::Bin(BinOp::Sub),
                Some('*') => Token::Bin(BinOp::Mul),
                Some('/') => Token::Bin(BinOp::Div),
                Some('%') => Token::Bin(BinOp::Mod),
                Some('<') => Token::Bin(BinOp::Lt),
                Some('>') => Token::Bin(BinOp::Gt),
                Some('=') => Token::Bin(BinOp::Eq),
                Some('|') => Token::Bin(BinOp::Or),
                Some('&') => Token::Bin(BinOp::And),
                Some('.') => Token::Bin(BinOp::Concat),
                Some('T') => Token::Bin(BinOp::Take),
                Some('D') => Token::Bin(BinOp::Drop),
                Some('$') => Token::Bin(BinOp::App),
                _ => bail!("invalid token: {s}"),
            },
            '?' => Token::If,
            'L' => {
                let var =
                    decode_base94(cur.0.next().ok_or_else(|| anyhow!("invalid token: {s}"))?)?;
                Token::Lambda(var as usize)
            }
            'v' => {
                let var =
                    decode_base94(cur.0.next().ok_or_else(|| anyhow!("invalid token: {s}"))?)?;
                Token::Var(var as usize)
            }
            _ => bail!("invalid token: {s}"),
        })
    }
}

pub fn tokenize(s: &str) -> anyhow::Result<Vec<Token>> {
    Ok(s.split_whitespace()
        .map(Token::from_str)
        .collect::<anyhow::Result<Vec<Token>>>()?)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Bool(bool),
    Int(i64),
    String(String),
    Var(usize),
    Un(UnOp, Box<Expr>),
    Bin(BinOp, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Lambda(usize, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(v) => write!(f, "{}", if *v { "T" } else { "F" }),
            Expr::Int(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "{s:?}"),
            Expr::Var(v) => write!(f, "v{v}"),
            Expr::Un(op, e) => write!(f, "({op} {e})"),
            Expr::Bin(op, l, r) => match op {
                BinOp::App => {
                    write!(f, "({l} {r})")
                }
                _ => {
                    write!(f, "({op} {l} {r})")
                }
            },
            Expr::If(cond, th, el) => write!(f, "if {cond} then {th} else {el}"),
            Expr::Lambda(v, e) => write!(f, "(λv{v}. {e})"),
        }
    }
}

struct TokenCursor<'a>(Peekable<std::slice::Iter<'a, Token>>);

impl Expr {
    pub fn is_nf(&self) -> bool {
        match self {
            Expr::Bool(_) | Expr::Int(_) | Expr::String(_) => true,
            _ => false,
        }
    }

    pub fn parse(tokens: &[Token]) -> anyhow::Result<Expr> {
        let mut cur = TokenCursor(tokens.iter().peekable());
        let ret = Self::parse_expr(&mut cur)?;
        if cur.0.next().is_some() {
            bail!("extra tokens")
        }
        Ok(ret)
    }

    fn parse_expr(cur: &mut TokenCursor<'_>) -> anyhow::Result<Expr> {
        Ok(match cur.0.next() {
            Some(Token::Bool(b)) => Expr::Bool(*b),
            Some(Token::Int(n)) => Expr::Int(*n),
            Some(Token::String(s)) => Expr::String(s.clone()),
            Some(Token::Var(v)) => Expr::Var(*v),
            Some(Token::Un(op)) => {
                let e = Box::new(Self::parse_expr(cur)?);
                Expr::Un(*op, e)
            }
            Some(Token::Bin(op)) => {
                let e1 = Box::new(Self::parse_expr(cur)?);
                let e2 = Box::new(Self::parse_expr(cur)?);
                Expr::Bin(*op, e1, e2)
            }
            Some(Token::If) => {
                let e1 = Box::new(Self::parse_expr(cur)?);
                let e2 = Box::new(Self::parse_expr(cur)?);
                let e3 = Box::new(Self::parse_expr(cur)?);
                Expr::If(e1, e2, e3)
            }
            Some(Token::Lambda(v)) => {
                let e = Box::new(Self::parse_expr(cur)?);
                Expr::Lambda(*v, e)
            }
            _ => bail!("invalid expr"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_encoded() {
        assert_eq!("T", Token::Bool(true).encoded().to_string());
        assert_eq!("F", Token::Bool(false).encoded().to_string());
        assert_eq!("I!", Token::Int(0).encoded().to_string());
        assert_eq!("I\"", Token::Int(1).encoded().to_string());
        assert_eq!("I/6", Token::Int(1337).encoded().to_string());
        assert_eq!(
            "SB%,,/}Q/2,$_",
            Token::String("Hello World!".into()).encoded().to_string()
        );
        assert_eq!("U-", Token::Un(UnOp::Neg).encoded().to_string());
        assert_eq!("U!", Token::Un(UnOp::Not).encoded().to_string());
        assert_eq!("U#", Token::Un(UnOp::StrToInt).encoded().to_string());
        assert_eq!("U$", Token::Un(UnOp::IntToStr).encoded().to_string());
        assert_eq!("B+", Token::Bin(BinOp::Add).encoded().to_string());
        assert_eq!("BT", Token::Bin(BinOp::Take).encoded().to_string());
        assert_eq!("BD", Token::Bin(BinOp::Drop).encoded().to_string());
        assert_eq!("B$", Token::Bin(BinOp::App).encoded().to_string());
        assert_eq!("?", Token::If.encoded().to_string());
        assert_eq!("L#", Token::Lambda(2).encoded().to_string());
        assert_eq!("v#", Token::Var(2).encoded().to_string());
    }
}
