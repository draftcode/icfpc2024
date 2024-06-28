use std::io;
use std::{iter::Peekable, str::Chars};
use anyhow::{anyhow, bail, Result};


#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum UnOp {
    Neg,
    Not,
    StrToInt,
    IntToStr,
}

#[derive(Clone, Copy, Debug)]
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

struct Cursor<'a>(Peekable<Chars<'a>>);

fn base94(c: char) -> anyhow::Result<i64> {
    if ('!'..='~').contains(&c) {
        let n = c as i64 - '!' as i64;
        Ok(n)
    } else {
        bail!("invalid base94 char")
    }
}

fn base94_char(c: char) -> anyhow::Result<char> {
    const TBL: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";
    Ok(TBL[base94(c)? as usize] as char)
}

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
                    n = n * 94 + base94(c)?;
                }
                Token::Int(n)
            }
            'S' => {
                let mut s = String::new();
                while let Some(c) = cur.0.next() {
                    s.push(base94_char(c)?);
                }
                Token::String(s)
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
                let var = base94(cur.0.next().ok_or_else(|| anyhow!("invalid token: {s}"))?)?;
                Token::Lambda(var as usize)
            }
            'v' => {
                let var = base94(cur.0.next().ok_or_else(|| anyhow!("invalid token: {s}"))?)?;
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

#[derive(Debug)]
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

struct TokenCursor<'a>(Peekable<std::slice::Iter<'a, Token>>);

impl Expr {
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

    pub fn pretty_print(&self) -> String {
        match &self {
            Expr::Bool(b) => { if *b { "true".to_string() } else { "false".to_string() }},
            Expr::Int(i) => { i.to_string() },
            Expr::String(s) => { format!("\"{}\"", s) },
            Expr::Var(idx) => { format!("v{}", idx) },
            Expr::Un(op, expr) => {
                let operand = expr.as_ref().pretty_print();
                match op {
                    UnOp::Neg => { format!("-{}", operand) },
                    UnOp::Not => { format!("!{}", operand) },
                    UnOp::StrToInt => { format!("stoi({})", operand) },
                    UnOp::IntToStr => { format!("itos({})", operand) },
                }
            },
            Expr::Bin(op, lhs, rhs) => {
                let l = lhs.as_ref().pretty_print();
                let r = rhs.as_ref().pretty_print();
                match op {
                    BinOp::Add => { format!("{}+{}", l, r) },
                    BinOp::Sub => { format!("{}-{}", l, r) },
                    BinOp::Mul => { format!("{}*{}", l, r) },
                    BinOp::Div => { format!("{}/{}", l, r) },
                    BinOp::Mod => { format!("{}%{}", l, r) },
                    BinOp::Lt => { format!("{}<{}", l, r) },
                    BinOp::Gt => { format!("{}>{}", l, r) },
                    BinOp::Eq => { format!("{}=={}", l, r) },
                    BinOp::Or => { format!("{}|{}", l, r) },
                    BinOp::And => { format!("{}&{}", l, r) },
                    BinOp::Concat => { format!("concat({},{})", l, r)},
                    BinOp::Take => { format!("take({},{})", l, r)},
                    BinOp::Drop => { format!("drop({},{})", l, r)},
                    BinOp::App => { format!("({} {})", l, r) },
                }
            },
            Expr::If(cond, th, el) => {
                let c = cond.as_ref().pretty_print();
                let t = th.as_ref().pretty_print();
                let e = el.as_ref().pretty_print();
                format!("if ({}) then {{{}}} else {{{}}})", c, t, e)
            },
            Expr::Lambda(idx, body) => {
                let b = body.as_ref().pretty_print();
                format!("(\\v{}. {})", idx, b)
            }
        }
    }

    pub fn pretty_print_lisp(&self) -> String {
        match &self {
            Expr::Bool(b) => { if *b { "#t".to_string() } else { "#f".to_string() }},
            Expr::Int(i) => { i.to_string() },
            Expr::String(s) => { format!("\"{}\"", s.replace("\n", "\\n")) },
            Expr::Var(idx) => { format!("v{}", idx) },
            Expr::Un(op, expr) => {
                let operand = expr.as_ref().pretty_print_lisp();
                match op {
                    UnOp::Neg => { format!("-{}", operand) },
                    UnOp::Not => { format!("!{}", operand) },
                    UnOp::StrToInt => { format!("(stoi {})", operand) },
                    UnOp::IntToStr => { format!("(itos {})", operand) },
                }
            },
            Expr::Bin(op, lhs, rhs) => {
                let l = lhs.as_ref().pretty_print_lisp();
                let r = rhs.as_ref().pretty_print_lisp();
                match op {
                    BinOp::Add => { format!("(+ {} {})", l, r) },
                    BinOp::Sub => { format!("(- {} {})", l, r) },
                    BinOp::Mul => { format!("(* {} {})", l, r) },
                    BinOp::Div => { format!("(/ {} {})", l, r) },
                    BinOp::Mod => { format!("(% {} {})", l, r) },
                    BinOp::Lt => { format!("(< {} {})", l, r) },
                    BinOp::Gt => { format!("(> {} {})", l, r) },
                    BinOp::Eq => { format!("(equal? {} {})", l, r) },
                    BinOp::Or => { format!("(and {} {})", l, r) },
                    BinOp::And => { format!("(or {} {})", l, r) },
                    BinOp::Concat => { format!("(string-append {} {})", l, r)},
                    BinOp::Take => { format!("(take {} {})", l, r)},
                    BinOp::Drop => { format!("(drop {} {})", l, r)},
                    BinOp::App => { format!("({} {})", l, r) },
                }
            },
            Expr::If(cond, th, el) => {
                let c = cond.as_ref().pretty_print_lisp();
                let t = th.as_ref().pretty_print_lisp();
                let e = el.as_ref().pretty_print_lisp();
                format!("(if {} {} {})", c, t, e)
            },
            Expr::Lambda(idx, body) => {
                let b = body.as_ref().pretty_print_lisp();
                format!("(lambda (v{}) {})", idx, b)
            }
        }
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let tokens = tokenize(&buffer)?;
    let exp = Expr::parse(&tokens)?;
    println!("{}", exp.pretty_print_lisp());
    Ok(())
}
