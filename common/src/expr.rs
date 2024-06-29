use std::{
    iter::Peekable,
    rc::Rc,
    str::{Chars, FromStr},
};

use anyhow::{anyhow, bail};
use num_bigint::BigInt;

use crate::base94::{decode_base94, decode_str, encode_base94, encode_base94_int, encode_str};

#[derive(Debug)]
pub enum Token {
    Bool(bool),
    Int(BigInt),
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
                if *n < BigInt::ZERO {
                    return Err(std::fmt::Error);
                }
                write!(f, "I")?;
                if *n == BigInt::ZERO {
                    write!(f, "{}", encode_base94(0).unwrap())?;
                } else {
                    let mut n = n.clone();
                    let mut chars: Vec<char> = Vec::new();
                    while n > BigInt::ZERO {
                        let n2 = &n / 94;
                        let r: BigInt = &n - &n2 * 94;
                        chars.push(encode_base94(r.try_into().unwrap()).unwrap());
                        n = n2;
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
    AppL,
    AppV,
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
            BinOp::AppL => "~",
            BinOp::AppV => "!",
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
            BinOp::AppL => "~",
            BinOp::AppV => "!",
        };
        write!(f, "{op}")
    }
}

struct Cursor<'a>(Peekable<Chars<'a>>);

fn decode_base94_cur(cur: &mut Cursor<'_>) -> anyhow::Result<BigInt> {
    let mut n = BigInt::from(0);
    while let Some(c) = cur.0.next() {
        n = n * 94 + decode_base94(c)?;
    }
    Ok(n)
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Token> {
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
            'I' => Token::Int(decode_base94_cur(&mut cur)?),
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
                Some('~') => Token::Bin(BinOp::AppL),
                Some('!') => Token::Bin(BinOp::AppV),
                _ => bail!("invalid token: {s}"),
            },
            '?' => Token::If,
            'L' => {
                let var = decode_base94_cur(&mut cur)?;
                Token::Lambda(var.try_into().unwrap())
            }
            'v' => {
                let var = decode_base94_cur(&mut cur)?;
                Token::Var(var.try_into().unwrap())
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
    Int(Rc<BigInt>),
    String(Rc<String>),
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

    pub fn parse_tokens(tokens: &[Token]) -> anyhow::Result<Expr> {
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
            Some(Token::Int(n)) => Expr::Int(n.clone().into()),
            Some(Token::String(s)) => Expr::String(s.clone().into()),
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
            e => bail!("invalid expr: {e:?}"),
        })
    }

    pub fn to_tokens(&self) -> Vec<Token> {
        match self {
            Expr::Bool(b) => vec![Token::Bool(*b)],
            Expr::Int(n) => vec![Token::Int(n.as_ref().clone())],
            Expr::String(s) => vec![Token::String(s.as_ref().clone())],
            Expr::Var(v) => vec![Token::Var(*v)],
            Expr::Un(op, e) => {
                let mut ret = vec![Token::Un(*op)];
                ret.extend(e.to_tokens());
                ret
            }
            Expr::Bin(op, l, r) => {
                let mut ret = vec![Token::Bin(*op)];
                ret.extend(l.to_tokens());
                ret.extend(r.to_tokens());
                ret
            }
            Expr::If(cond, th, el) => {
                let mut ret = vec![Token::If];
                ret.extend(cond.to_tokens());
                ret.extend(th.to_tokens());
                ret.extend(el.to_tokens());
                ret
            }
            Expr::Lambda(v, e) => {
                let mut ret = vec![Token::Lambda(*v)];
                ret.extend(e.to_tokens());
                ret
            }
        }
    }

    pub fn encoded(&self) -> ExprEncoded {
        ExprEncoded(self)
    }
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Expr> {
        let tokens = tokenize(s)?;
        log::info!("tokenize: {tokens:?}");
        Expr::parse_tokens(&tokens)
    }
}

pub struct ExprEncoded<'a>(&'a Expr);

impl std::fmt::Display for ExprEncoded<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, token) in self.0.to_tokens().into_iter().enumerate() {
            if i == 0 {
                write!(f, "{}", token.encoded())?;
            } else {
                write!(f, " {}", token.encoded())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_encoded() {
        assert_eq!("T", Token::Bool(true).encoded().to_string());
        assert_eq!("F", Token::Bool(false).encoded().to_string());
        assert_eq!("I!", Token::Int(0.into()).encoded().to_string());
        assert_eq!("I\"", Token::Int(1.into()).encoded().to_string());
        assert_eq!("I/6", Token::Int(1337.into()).encoded().to_string());
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
        assert_eq!("B~", Token::Bin(BinOp::AppL).encoded().to_string());
        assert_eq!("B!", Token::Bin(BinOp::AppV).encoded().to_string());
    }

    #[test]
    fn expr_encoded() {
        let eff12 = r#"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L% B$ B$ L" L# ? B< v" v# v" v# v% B+ I" ? B> v% I# B$ B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L& L' L( ? B= v' v% v( B$ B$ v& B+ v' I" ? B> B$ v$ v' B- v' I" ? B= B% v% v' I! B* B/ v( B$ v$ v' B- B$ v$ v' I" v( v( I# v% v% I"Ndb"#;
        let expr: Expr = eff12.parse().unwrap();
        assert_eq!(eff12, expr.encoded().to_string());
    }
}
