use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum OpCode {
    NOOP,
    ADDX(i32),
}

impl OpCode {
    fn parse(line: &str) -> OpCode {
        if line == "noop" {
            return OpCode::NOOP;
        } else if line.starts_with("addx") {
            let arg = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            return OpCode::ADDX(arg);
        }

        panic!("invalid opcode: '{}'", line);
    }

    fn cycles(&self) -> u32 {
        match self {
            Self::NOOP => 1,
            Self::ADDX(_) => 2,
        }
    }
}

fn is_sample_point(cycle: u32) -> bool {
    cycle >= 20 && (cycle - 20) % 40 == 0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let data = fs::read_to_string(file_path).unwrap();

    let program = data.lines().map(OpCode::parse).collect::<Vec<_>>();

    // program counter and register (machine state)
    let mut pc: usize = 0; // program counter - instruction to be executed next
    let mut clock: u32 = 0; // cycle counter (monotonically increasing)
    let mut x_reg: i32 = 1; // Register value
    let mut ic = 0; // instruction counter

    let mut total_signal = 0;

    while let Some(op) = program.get(pc) {
        // calculate signal strength at sample points
        if is_sample_point(clock + 1) {
            let signal = (clock + 1) as i32 * x_reg;
            total_signal += signal;
        }

        // draw the screen
        let h_pos = clock as i32 % 40;
        if (x_reg - 1..=x_reg + 1).contains(&h_pos) {
            print!("#");
        } else {
            print!(".");
        }
        if (clock + 1) % 40 == 0 {
            println!();
        }

        // increment counters
        ic += 1;
        clock += 1;

        // at the end of an instruction
        // side-affects of op happen here
        if ic % op.cycles() == 0 {
            ic = 0;
            pc += 1;

            if let OpCode::ADDX(arg) = op {
                x_reg += arg;
            }
        }
    }

    dbg!(total_signal);
}
