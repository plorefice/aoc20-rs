use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Add,
    Mul,
    Open,
    Close,
    Num(u64),
}

pub fn parse_input<S: AsRef<str>>(input: S) -> Vec<VecDeque<Token>> {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .fold(VecDeque::with_capacity(128), |mut tokens, word| {
                    for ch in word.bytes() {
                        match ch {
                            b'+' => tokens.push_back(Token::Add),
                            b'*' => tokens.push_back(Token::Mul),
                            b'(' => tokens.push_back(Token::Open),
                            b')' => tokens.push_back(Token::Close),
                            _ => tokens.push_back(Token::Num((ch - b'0') as u64)),
                        }
                    }
                    tokens
                })
        })
        .collect()
}

pub fn part_1(mut exprs: Vec<VecDeque<Token>>) -> u64 {
    exprs.iter_mut().map(|expr| evaluate(expr, false)).sum()
}

pub fn part_2(mut exprs: Vec<VecDeque<Token>>) -> u64 {
    exprs.iter_mut().map(|expr| evaluate(expr, true)).sum()
}

fn evaluate(expr: &mut VecDeque<Token>, p2: bool) -> u64 {
    let mut stack = VecDeque::with_capacity(expr.len());
    let mut prod_stack = VecDeque::with_capacity(expr.len());

    while let Some(tok) = expr.pop_front() {
        match tok {
            Token::Num(_) | Token::Add | Token::Mul => stack.push_back(tok),
            Token::Open => stack.push_back(Token::Num(evaluate(expr, p2))),
            Token::Close => break,
        }
    }

    while stack.len() > 1 {
        match (stack.pop_front(), stack.pop_front(), stack.pop_front()) {
            (Some(Token::Num(b)), Some(Token::Add), Some(Token::Num(a))) => {
                stack.push_front(Token::Num(a + b))
            }
            (Some(Token::Num(b)), Some(Token::Mul), Some(Token::Num(a))) => {
                if !p2 {
                    stack.push_front(Token::Num(a * b))
                } else {
                    prod_stack.push_back(Token::Num(b));
                    prod_stack.push_back(Token::Mul);
                    stack.push_front(Token::Num(a));
                }
            }
            _ => {}
        }
    }

    if p2 {
        prod_stack.push_back(stack.pop_front().unwrap());

        while prod_stack.len() > 1 {
            match (
                prod_stack.pop_front(),
                prod_stack.pop_front(),
                prod_stack.pop_front(),
            ) {
                (Some(Token::Num(b)), Some(Token::Mul), Some(Token::Num(a))) => {
                    prod_stack.push_front(Token::Num(a * b))
                }
                v => unreachable!("{:?} {:?} {:?}", v, stack, prod_stack),
            }
        }

        stack.push_back(prod_stack.pop_front().unwrap());
    }

    match stack.pop_front().unwrap() {
        Token::Num(n) => n,
        tok => unreachable!("unexpected token in stack: {:?}", tok),
    }
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day18.txt"))),
        11076907812171
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day18.txt"))),
        283729053022731
    }
}
