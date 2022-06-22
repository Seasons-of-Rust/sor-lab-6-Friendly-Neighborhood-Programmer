use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input2.txt").expect("input not found"))
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|number| number.parse().expect("could not parse number"))
        .collect();

    let noun = 12;
    let verb = 2;

    println!(
        "The final code for part 1 is {:?}",
        run_program(numbers.clone(), noun, verb)
    );

    println!(
        "The final code for part 2 is {:?}",
        find_answer(numbers, 19690720)
    );
}

struct IntcodeParser {
    ipointer: usize,
    memory: Vec<i32>,
}

impl IntcodeParser {
    fn get(&self, index: i32) -> i32 {
        self.memory[index as usize]
    }

    fn set(&mut self, index: i32, val: i32) {
        self.memory[index as usize] = val;
    }

    fn get_next_codes(&mut self) -> (i32, i32, i32, i32) {
        let op = self.get(self.ipointer as i32);
        match op {
            99 => (99, -1, -1, -1), // program finished
            _ => {
                let x = self.get((self.ipointer + 1) as i32);
                let y = self.get((self.ipointer + 2) as i32);
                let z = self.get((self.ipointer + 3) as i32);
                self.ipointer += 4;
                (op, x, y, z)
            }
        }
    }

    fn use_opcode(mut self, op: i32, x: i32, y: i32, z: i32) -> Self {
        match op {
            1 => self.set(z, self.get(x) + self.get(y)),
            2 => self.set(z, self.get(x) * self.get(y)),
            _ => println!("ERROORRR!!"), // this should never happen
        }
        self
    }
}

fn run_program(codes: Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut i_parser = IntcodeParser {
        ipointer: 0,
        memory: codes,
    };

    // restore it to previous state before it broke
    i_parser.set(1, noun);
    i_parser.set(2, verb);

    loop {
        let (op, x, y, z) = i_parser.get_next_codes();

        match op {
            99 => break,
            _ => i_parser = i_parser.use_opcode(op, x, y, z),
        }
    }

    i_parser.get(0)
}

fn find_answer(codes: Vec<i32>, needed_answer: i32) -> i32 {
    for i in 0..99 {
        for j in 0..99 {
            let answer = run_program(codes.clone(), i, j);
            if answer == needed_answer {
                return 100 * i + j;
            }
        }
    }
    -1
}
