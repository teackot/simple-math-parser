use std::{iter::{Peekable, Rev}, slice::Iter};

macro_rules! errexit {
    ($reason:literal) => {
        println!("Error: {}", $reason);
        std::process::exit(-1);
    };
}

// Operators
#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
enum Token {
    Operator(Op),

    Constant(u32),

    ParenOpen,
    ParenClose,
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.peek() {
        // parse a digit
        if c.is_numeric() {
            let mut constant: u32 = 0;
            while let Some(digit) = iter.peek() {
                if !digit.is_numeric() {
                    break;
                }

                constant *= 10;
                constant += digit.to_digit(10).unwrap();

                iter.next();
            }

            tokens.push(Token::Constant(constant));
            continue;
        }

        // parse an operator / parenthesis
        if !c.is_whitespace() {
            tokens.push(match c {
                '+' => Token::Operator(Op::Add),
                '-' => Token::Operator(Op::Sub),
                '*' => Token::Operator(Op::Mul),
                '/' => Token::Operator(Op::Div),
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                _   => {
                    errexit!("Unknown operator!");
                }
            });
        }

        iter.next();
    }

    return tokens;
}

enum Expression {
    Operator(Op, Box<Expression>, Box<Expression>),
    Constant(i32)
}

impl Expression {
    fn parse_block(
        iter: &mut Peekable<Rev<Iter<Token>>>,
        is_paren_block: bool,
    ) -> Self {
        let mut expr: Option<Expression> = None; // self
        let mut operand: Option<Expression> = None;

        let mut paren_close_matched = false; // used if is_paren_block is true

        while let Some(&token) = iter.peek() {
            // ParenOpen is a special case
            if let Token::ParenOpen = token {
                    // consume the paren if it belongs to this block
                    if is_paren_block {
                        paren_close_matched = true;
                        iter.next();
                    }

                    break;
            }

            iter.next();

            match token {
                Token::Constant(n) => {
                    operand = Some(Expression::Constant(
                        (*n).try_into().unwrap_or_else(|_| { errexit!("Out of bounds!"); })
                    ));
                },

                Token::Operator(op) => {
                    expr = Some(Expression::Operator(
                        *op,
                        Box::new(Expression::parse_block(iter, false)),
                        Box::new(operand.unwrap_or_else(|| { errexit!("Expected an operand!"); }))
                    ));
                    operand = None;
                },

                Token::ParenClose => {
                    operand = Some(Expression::parse_block(iter, true));
                },

                Token::ParenOpen => unreachable!(),
            }
        }

        if is_paren_block && !paren_close_matched {
            errexit!("Unmatched parenthesis!");
        }

        if let Some(expr) = expr {
            expr
        } else {
            operand.unwrap_or_else(|| { errexit!("Expected an operand!"); })
        }
    }

    pub fn parse(tokens: &[Token]) -> Self {
        Expression::parse_block(
            &mut tokens.iter().rev().peekable(),
            false
        )
    }

    pub fn evaluate(&self) -> i32 {
        match self {
            Self::Operator(op, a, b) => {
                match op {
                    Op::Add => a.evaluate() + b.evaluate(),
                    Op::Sub => a.evaluate() - b.evaluate(),
                    Op::Mul => a.evaluate() * b.evaluate(),
                    Op::Div => a.evaluate() / b.evaluate(),
                }
            },
            Self::Constant(n) => *n,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: simple-math-parser <expression>");
        return;
    }

    println!("{}", Expression::parse(&tokenize(args[1].as_str())).evaluate()); // 10
}
