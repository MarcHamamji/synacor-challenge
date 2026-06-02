use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

use crate::value::Value;

struct LifterLine {
    incoming_edges: u16,
    content: String,
}

impl Display for LifterLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{: >10} {}",
            if self.incoming_edges > 0 {
                format!("[{}] →", self.incoming_edges)
            } else {
                " ".to_string()
            },
            self.content,
        )
    }
}

pub struct SynacorLifter {
    data: Vec<u16>,
    size: usize,
    index: usize,
    lines: BTreeMap<u16, LifterLine>,
}

impl SynacorLifter {
    pub fn new(data: Vec<u16>) -> SynacorLifter {
        let size = data.len();

        SynacorLifter {
            data,
            size,
            index: 0,
            lines: BTreeMap::new(),
        }
    }

    pub fn lift(&mut self) {
        while self.index < self.size - 1 {
            self.collect_current_instruction();
        }

        for line in &self.lines {
            let address = line.0;
            let line = line.1;

            println!("[{address}] {line}");
        }
    }

    fn collect_current_instruction(&mut self) {
        let mut instruction = self.get_current_data();

        let line = self.index;

        let content = match instruction {
            // halt: 0
            //   stop execution and terminate the program
            0 => {
                self.index += 1;
                "halt".to_string()
            }

            // set: 1 a b
            //   set register <a> to the value of <b>
            1 => {
                let register = self.value_from_offset(1);
                let value = self.value_from_offset(2);

                self.index += 3;
                format!("set {register} {value}")
            }

            // push: 2 a
            //   push <a> onto the stack
            2 => {
                let value = self.value_from_offset(1);

                self.index += 2;
                format!("push {value}")
            }

            // pop: 3 a
            //   remove the top element from the stack and write it into <a>; empty stack = error
            3 => {
                let value = self.value_from_offset(1);

                self.index += 2;
                format!("pop {value}")
            }

            // eq: 4 a b c
            //   set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
            4 => {
                let register = self.value_from_offset(1);
                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("eq {register} {value_a} {value_b}")
            }

            // gt: 5 a b c
            //   set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
            5 => {
                let register = self.value_from_offset(1);
                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("gt {register} {value_a} {value_b}")
            }

            // jmp: 6 a
            //   jump to <a>
            6 => {
                let address = self.value_from_offset(1);

                self.add_incoming_edge(address.0);

                self.index += 2;
                format!("jump {address}")
            }

            // jt: 7 a b
            //   if <a> is nonzero, jump to <b>
            7 => {
                let condition = self.value_from_offset(1);
                let address = self.value_from_offset(2);

                self.add_incoming_edge(address.0);

                self.index += 3;
                format!("jt {condition} {address}")
            }

            // jf: 8 a b
            //   if <a> is zero, jump to <b>
            8 => {
                let condition = self.value_from_offset(1);
                let address = self.value_from_offset(2);

                self.add_incoming_edge(address.0);

                self.index += 3;
                format!("jf {condition} {address}")
            }

            // add: 9 a b c
            //   assign into <a> the sum of <b> and <c> (modulo 32768)
            9 => {
                let register = self.value_from_offset(1);

                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("add {register} {value_a} {value_b}")
            }

            // mult: 10 a b c
            //   store into <a> the product of <b> and <c> (modulo 32768)
            10 => {
                let register = self.value_from_offset(1);

                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("mult {register} {value_a} {value_b}")
            }

            // mod: 11 a b c
            //   store into <a> the remainder of <b> divided by <c>
            11 => {
                let register = self.value_from_offset(1);

                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("mod {register} {value_a} {value_b}")
            }

            // and: 12 a b c
            //   stores into <a> the bitwise and of <b> and <c>
            12 => {
                let register = self.value_from_offset(1);

                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("and {register} {value_a} {value_b}")
            }

            // or: 13 a b c
            //   stores into <a> the bitwise or of <b> and <c>
            13 => {
                let register = self.value_from_offset(1);

                let value_a = self.value_from_offset(2);
                let value_b = self.value_from_offset(3);

                self.index += 4;
                format!("or {register} {value_a} {value_b}")
            }

            // not: 14 a b
            //   stores 15-bit bitwise inverse of <b> in <a>
            14 => {
                let register = self.value_from_offset(1);

                let value = self.value_from_offset(2);

                self.index += 3;
                format!("not {register} {value}")
            }

            // rmem: 15 a b
            //   read memory at address <b> and write it to <a>
            15 => {
                let register = self.value_from_offset(1);

                let address = self.value_from_offset(2);

                self.index += 3;
                format!("rmem {register} {address}")
            }

            // wmem: 16 a b
            //   write the value from <b> into memory at address <a>
            16 => {
                let address = self.value_from_offset(1);

                let register = self.value_from_offset(2);

                self.index += 3;
                format!("wmem {address} {register}")
            }

            // call: 17 a
            //   write the address of the next instruction to the stack and jump to <a>
            17 => {
                let address = self.value_from_offset(1);

                self.add_incoming_edge(address.0);

                self.index += 2;
                format!("call {address}")
            }

            // ret: 18
            //   remove the top element from the stack and jump to it; empty stack = halt
            18 => {
                self.index += 1;
                "ret".to_string()
            }

            // out: 19 a
            //   write the character represented by ascii code <a> to the terminal
            19 => {
                let mut value = self.value_from_offset(1);

                if value.is_literal() {
                    let mut output = String::from("out \x1b[38;5;4m");
                    while instruction == 19 && value.is_literal() {
                        output.push_str(format!("{}", char::from(value.0 as u8)).as_str());

                        self.index += 2;
                        instruction = self.get_current_data();
                        value = self.value_from_offset(1);
                    }
                    output.push_str("\x1b[0m");

                    output
                } else {
                    self.index += 2;
                    format!("out {value}")
                }
            }

            // in: 20 a
            //   read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
            20 => {
                let register = self.value_from_offset(1);

                self.index += 2;
                format!("in {register}")
            }

            // noop: 21
            //   no operation
            21 => {
                let mut count = 0;

                while instruction == 21 {
                    count += 1;
                    self.index += 1;
                    instruction = self.get_current_data();
                }
                format!("noop ({count})")
            }

            // _ => unimplemented!("instruction code {instruction} at position {}", self.index),
            _ => {
                let value = self.value_from_offset(1);

                self.index += 1;
                format!("{}", value)
            }
        }
        .into();

        let existing_line = self.lines.get_mut(&(line as u16));
        if let Some(line) = existing_line {
            line.content = content;
        } else {
            self.lines.insert(
                line as u16,
                LifterLine {
                    incoming_edges: 0,
                    content,
                },
            );
        }
    }

    fn get_current_data(&self) -> u16 {
        self.get_data(self.index)
    }

    fn get_data(&self, position: usize) -> u16 {
        self.data
            .get(position)
            .expect("Unable to read data at position")
            .clone()
    }

    fn value_from_offset(&self, offset: usize) -> Value {
        Value(self.get_data(self.index + offset))
    }

    fn add_incoming_edge(&mut self, address: u16) {
        let line = self.lines.get_mut(&address);
        if let Some(line) = line {
            line.incoming_edges += 1;
        } else {
            self.lines.insert(
                address,
                LifterLine {
                    incoming_edges: 1,
                    content: "".to_string(),
                },
            );
        }
    }
}
