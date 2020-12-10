use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ExitCond {
    Loop,
    Exit,
}

pub struct Console {
    program: Vec<Opcode>,
    acc: isize,
    ip: usize,
}

impl Console {
    fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
    }

    fn run(&mut self) -> ExitCond {
        let mut visited = vec![false; self.program.len()];

        while self.ip < self.program.len() {
            if visited[self.ip] {
                return ExitCond::Loop;
            }

            visited[self.ip] = true;

            match self.program[self.ip] {
                Opcode::Acc(n) => self.acc += n,
                Opcode::Jmp(n) => {
                    self.ip = (self.ip as isize + n) as usize;
                    continue;
                }
                Opcode::Nop(_) => (),
            };

            self.ip += 1;
        }

        ExitCond::Exit
    }
}

impl FromStr for Console {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program = s
            .lines()
            .map(|line| {
                let mut words = line.split_ascii_whitespace();
                let opcode = words.next().unwrap();
                let value = words.next().unwrap().parse().unwrap();

                match opcode {
                    "acc" => Opcode::Acc(value),
                    "jmp" => Opcode::Jmp(value),
                    "nop" => Opcode::Nop(value),
                    _ => unreachable!(),
                }
            })
            .collect();

        Ok(Console {
            program,
            acc: 0,
            ip: 0,
        })
    }
}

pub fn part_1(console: &mut Console) -> isize {
    console.reset();
    console.run();
    console.acc
}

pub fn part_2(console: &mut Console) -> isize {
    for i in 0..console.program.len() {
        let replacement = match console.program[i] {
            Opcode::Jmp(n) => Opcode::Nop(n),
            Opcode::Nop(n) => Opcode::Jmp(n),
            _ => continue,
        };

        let original = console.program[i];
        console.program[i] = replacement;

        console.reset();
        if console.run() == ExitCond::Exit {
            return console.acc;
        }

        console.program[i] = original;
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(&mut Console::from_str(include_str!("../inputs/day08.txt")).unwrap()),
        1446
    },
    p2 => {
        part_2(&mut Console::from_str(include_str!("../inputs/day08.txt")).unwrap()),
        1403
    }
}
