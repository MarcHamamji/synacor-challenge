use std::io::{Read, Write};

use tracing::trace;

const TELEPORTATION_REG_7: u16 = 25734;

pub struct SynacorVM {
    memory: Vec<u16>,
    size: usize,
    pc: usize,
    halted: bool,
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl SynacorVM {
    pub fn new(data: Vec<u16>) -> SynacorVM {
        let size = data.len();
        SynacorVM {
            memory: data,
            size,
            pc: 0,
            halted: false,
            registers: [0; 8],
            stack: vec![],
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            if self.pc > self.size {
                break;
            }

            self.execute_current_instruction()
        }
    }

    fn execute_current_instruction(&mut self) {
        let instruction = self.get_current_data();

        match instruction {
            // halt: 0
            //   stop execution and terminate the program
            0 => {
                trace!("halt at position {}", self.pc);
                self.halted = true;
            }

            // set: 1 a b
            //   set register <a> to the value of <b>
            1 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));
                let value = self.evaluate(self.get_data(self.pc + 2));

                self.registers[register_number] = value;

                self.pc += 3;
            }

            // push: 2 a
            //   push <a> onto the stack
            2 => {
                self.stack.push(self.evaluate(self.get_data(self.pc + 1)));
                self.pc += 2;
            }

            // pop: 3 a
            //   remove the top element from the stack and write it into <a>; empty stack = error
            3 => {
                let value = self.stack.pop().expect("Unable to pop empty stack");

                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                self.registers[register_number] = value;

                self.pc += 2;
            }

            // eq: 4 a b c
            //   set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
            4 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = if value_a == value_b { 1 } else { 0 };

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // gt: 5 a b c
            //   set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
            5 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = if value_a > value_b { 1 } else { 0 };

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // jmp: 6 a
            //   jump to <a>
            6 => {
                self.jump(self.evaluate(self.get_data(self.pc + 1)));
            }

            // jt: 7 a b
            //   if <a> is nonzero, jump to <b>
            7 => {
                let value = self.evaluate(self.get_data(self.pc + 1));

                if value != 0 {
                    let position = self.evaluate(self.get_data(self.pc + 2));
                    self.jump(position);
                } else {
                    self.pc += 3;
                }
            }

            // jf: 8 a b
            //   if <a> is zero, jump to <b>
            8 => {
                let value = self.evaluate(self.get_data(self.pc + 1));

                if value == 0 {
                    let position = self.evaluate(self.get_data(self.pc + 2));
                    self.jump(position);
                } else {
                    self.pc += 3;
                }
            }

            // add: 9 a b c
            //   assign into <a> the sum of <b> and <c> (modulo 32768)
            9 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = (value_a + value_b) % 32768;

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // mult: 10 a b c
            //   store into <a> the product of <b> and <c> (modulo 32768)
            10 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = (value_a.wrapping_mul(value_b)) % 32768;

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // mod: 11 a b c
            //   store into <a> the remainder of <b> divided by <c>
            11 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = value_a % value_b;

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // and: 12 a b c
            //   stores into <a> the bitwise and of <b> and <c>
            12 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = value_a & value_b;

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // or: 13 a b c
            //   stores into <a> the bitwise or of <b> and <c>
            13 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));
                let value_b = self.evaluate(self.get_data(self.pc + 3));

                let value = value_a | value_b;

                self.registers[register_number] = value;

                self.pc += 4;
            }

            // not: 14 a b
            //   stores 15-bit bitwise inverse of <b> in <a>
            14 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let value_a = self.evaluate(self.get_data(self.pc + 2));

                let value = !value_a & !(1 << 15);

                self.registers[register_number] = value;

                self.pc += 3;
            }

            // rmem: 15 a b
            //   read memory at address <b> and write it to <a>
            15 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let address = self.evaluate(self.get_data(self.pc + 2));

                self.registers[register_number] = self.get_data(address.into());

                self.pc += 3;
            }

            // wmem: 16 a b
            //   write the value from <b> into memory at address <a>
            16 => {
                let address = self.evaluate(self.get_data(self.pc + 1));

                let value = self.evaluate(self.get_data(self.pc + 2));

                self.memory[address as usize] = value;

                self.pc += 3;
            }

            // call: 17 a
            //   write the address of the next instruction to the stack and jump to <a>
            17 => {
                self.stack.push((self.pc + 2) as u16);

                let address = self.evaluate(self.get_data(self.pc + 1));
                trace!("calling function at {address}");

                self.jump(address);
            }

            // ret: 18
            //   remove the top element from the stack and jump to it; empty stack = halt
            18 => {
                let address = self.stack.pop().expect("Cannot return with an empty stack");
                self.jump(address);
            }

            // out: 19 a
            //   write the character represented by ascii code <a> to the terminal
            19 => {
                self.pc += 1;

                let ascii = self.evaluate(self.get_current_data()) as u8;

                std::io::stdout()
                    .write(&[ascii])
                    .expect("Unable to write character to stdout");

                self.pc += 1;
            }

            // in: 20 a
            //   read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
            20 => {
                let register_number = self.get_register_number(self.get_data(self.pc + 1));

                let mut char: [u8; 1] = [0; 1];
                let result = std::io::stdin().read_exact(&mut char);
                if result.is_err_and(|e| e.kind() == std::io::ErrorKind::UnexpectedEof) {
                    std::process::exit(0);
                }

                if char[0] == b'.' {
                    print!(
                        "\x1b[38;5;3mSET REGISTER $7 TO {}\x1b[0m",
                        TELEPORTATION_REG_7
                    );
                    self.registers[7] = TELEPORTATION_REG_7;

                    std::io::stdin()
                        .read_exact(&mut char)
                        .expect("Unable to read from stdin");
                } else if char[0] == b'/' {
                    const DUMP_LOCATION: &str = "/tmp/memdump.bin";
                    self.dump_memory(DUMP_LOCATION);
                    println!("Successfully wrote memory dump to {}", DUMP_LOCATION);

                    std::io::stdin()
                        .read_exact(&mut char)
                        .expect("Unable to read from stdin");
                }

                print!("\x1b[38;5;3m{}\x1b[0m", char[0] as char);

                self.registers[register_number] = char[0].into();

                self.pc += 2;
            }

            // noop: 21
            //   no operation
            21 => {
                self.pc += 1;
            }
            _ => unimplemented!("instruction code {instruction} at position {}", self.pc),
        }
    }

    fn get_register_number(&self, data: u16) -> usize {
        let register = data - 32768;
        assert!(register < 8);
        register.into()
    }

    fn evaluate(&self, data: u16) -> u16 {
        if data <= 32767 {
            data
        } else if data <= 32775 {
            let register = self.get_register_number(data);
            // if register == 7 {
            //     return 6;
            // }
            self.registers[register as usize]
        } else {
            panic!("Invalid data {data}")
        }
    }

    fn get_current_data(&self) -> u16 {
        self.get_data(self.pc)
    }

    fn get_data(&self, position: usize) -> u16 {
        self.memory
            .get(position)
            .expect("Unable to read data at position")
            .clone()
    }

    fn jump(&mut self, position: u16) {
        self.pc = position.try_into().expect("Invalid address to jump tp");
    }

    fn dump_memory(&self, path: &str) {
        std::fs::write(
            path,
            self.memory
                .iter()
                .flat_map(|a| [(a & 0b11111111) as u8, (a >> 8) as u8])
                .collect::<Vec<u8>>(),
        )
        .expect("Failed to write memory dump");
    }
}
