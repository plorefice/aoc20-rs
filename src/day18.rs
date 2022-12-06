use std::collections::VecDeque;

pub fn parse_input<S: AsRef<[u8]>>(input: S, p2: bool) -> Vec<VecDeque<u8>> {
    input
        .as_ref()
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut output = Vec::with_capacity(32);
            let mut operators = Vec::with_capacity(16);

            for &ch in line.iter().filter(|c| !c.is_ascii_whitespace()) {
                match ch {
                    b'+' | b'*' | b')' => {
                        while let Some(&op) = operators.last() {
                            if op == b'(' || (p2 && ch == b'+' && op == b'*') {
                                break;
                            } else {
                                operators.pop();
                                output.push(op);
                            }
                        }

                        if ch == b')' {
                            operators.pop();
                        } else {
                            operators.push(ch);
                        }
                    }
                    b'(' => operators.push(ch),
                    _ => output.push(ch),
                }
            }

            while let Some(op) = operators.pop() {
                output.push(op);
            }

            VecDeque::from(output)
        })
        .collect()
}

pub fn solve(mut exprs: Vec<VecDeque<u8>>) -> u64 {
    exprs.iter_mut().map(evaluate).sum()
}

fn evaluate(expr: &mut VecDeque<u8>) -> u64 {
    let mut stack = Vec::with_capacity(16);

    while let Some(tok) = expr.pop_front() {
        match tok {
            b'+' | b'*' => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(if tok == b'+' { a + b } else { a * b });
            }
            _ => stack.push((tok - b'0') as u64),
        }
    }

    stack.pop().unwrap()
}

crate::solutions! {
    p1 => {
        solve(parse_input(include_bytes!("../inputs/day18.txt"), false)),
        11076907812171
    },
    p2 => {
        solve(parse_input(include_bytes!("../inputs/day18.txt"), true)),
        283729053022731
    }
}
