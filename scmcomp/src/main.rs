use anyhow::Result;
use std::{io::Read, str::Chars};

struct Program {
    defines: Vec<Expr>,
}

enum Expr {
    Proc { name: String, args: Vec<Expr> },
    Str(String),
    Num(i32),
    Var(String),
}

#[argopt::subcmd]
fn compile() -> anyhow::Result<()> {
    eprint!("> ");

    let mut input = "".to_string();
    std::io::stdin().read_to_string(&mut input)?;

    let program = parse(input.chars())?;

    Ok(())
}

fn parse<'a>(cs: Chars<'a>) -> anyhow::Result<Program> {
    let sc = Tokenizer::new(cs);

    sc.into_iter().for_each(|x| println!("{:?}", x));
    // for(;;) {

    // }
    // for c in cs.next() {}

    todo!();
}

struct Tokenizer<'a> {
    cs: Chars<'a>,
    c: Option<char>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Tokenizer<'a> {
    fn new(cs: Chars<'a>) -> Self {
        Tokenizer { cs, c: None }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.c {
            self.c = None;
            return Some(c);
        }
        self.cs.next()
    }

    fn push_back(&mut self, c: char) {
        self.c = c.into();
    }

    fn next_token(&mut self) -> Option<Token> {
        let fst = self.cs.next()?;
        if fst.is_whitespace() {
            return self.next_token();
        }

        match fst {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            '0'..='9' => {
                let mut num = (fst as i32) - ('0' as i32);
                while let Some(c) = self.next_char() {
                    match c {
                        '0'..='9' => num = num * 10 + (c as i32) - ('0' as i32),
                        _ => {
                            self.push_back(c);
                            return Some(Token::Num(num));
                        }
                    }
                }
                Some(Token::Num(num))
            }
            '"' => {
                let mut s = String::new();
                while let Some(c) = self.next_char() {
                    match c {
                        '"' => return Some(Token::Str(s)),
                        _ => s.push(c),
                    }
                }
                Some(Token::Str(s))
            }
            _ => {
                let mut s = String::new();
                s.push(fst);
                while let Some(c) = self.next_char() {
                    match c {
                        _ if c.is_whitespace() || c == '(' || c == ')' || c == '"' => {
                            self.push_back(c);
                            break;
                        }
                        _ => s.push(c),
                    }
                }
                Some(Token::Var(s))
            }
        }
    }
}

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    Str(String),
    Num(i32),
    Var(String),
}

#[argopt::cmd_group(commands = [compile])]
fn main() -> anyhow::Result<()> {}
