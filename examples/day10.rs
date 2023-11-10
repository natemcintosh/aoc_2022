#[derive(PartialEq, Debug, Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl From<&str> for Instruction {
    /// parses `addx V` or `noop` into an Instruction
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let instruction = parts.next();
        if instruction == Some("noop") {
            return Instruction::Noop;
        } else if instruction == Some("addx") {
            let number: i64 = parts
                .next()
                .expect("No number to parse after addx")
                .parse()
                .expect("Could not parse number after addx");
            return Instruction::Addx(number);
        }
        panic!("Could not parse line of input")
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.lines().map(Instruction::from).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct CPU {
    cycle: i64,
    register_during: i64,
    register_after: i64,
}

impl CPU {
    /// Create a freshly initialized CPU state
    fn new() -> CPU {
        CPU {
            cycle: 0,
            register_during: 1,
            register_after: 1,
        }
    }

    /// Step the CPU state based on the instruction
    fn step(&self, inst: Instruction) -> CPU {
        match inst {
            Instruction::Noop => CPU {
                cycle: self.cycle + 1,
                register_during: self.register_after,
                register_after: self.register_after,
            },
            Instruction::Addx(number) => CPU {
                cycle: self.cycle + 2,
                register_during: self.register_after,
                register_after: self.register_after + number,
            },
        }
    }
}

/// Sum the signal strength at cycles 20, 60, 100, 140, 180, 220.
/// Signal strength is the cycle number multiplied by the value of the register
fn part1(instructions: &[Instruction]) -> i64 {
    let mut desired_cycles = vec![20, 60, 100, 140, 180, 220];
    desired_cycles.reverse();
    let mut next_goal_cycle = desired_cycles.pop().expect("Nothing in desired_cycles");

    let mut cpu = CPU::new();

    let mut total_signal_strength = 0;

    for inst in instructions {
        // Check if we've reached the next_goal_cycle
        if cpu.cycle == next_goal_cycle {
            // What was the value DURING the cycle
            total_signal_strength += next_goal_cycle * cpu.register_during;

            // Get the next goal cycle
            match desired_cycles.pop() {
                Some(gc) => {
                    next_goal_cycle = gc;
                }
                None => {
                    // Finished. Return
                    return total_signal_strength;
                }
            }
        } else if cpu.cycle > next_goal_cycle {
            // We've gone past it, still use the value during??
            total_signal_strength += next_goal_cycle * cpu.register_during;

            // Get the next goal cycle
            match desired_cycles.pop() {
                Some(gc) => {
                    next_goal_cycle = gc;
                }
                None => {
                    // Finished. Return
                    return total_signal_strength;
                }
            }
        }

        // Process instruction
        cpu = cpu.step(*inst);
    }

    total_signal_strength
}

fn part2(instructions: &[Instruction]) -> Vec<String> {
    // The raw pixels. '.' by default
    let mut pixels = [['.'; 40]; 6];

    // Create the CPU
    let mut cpu = CPU::new();
    let mut instr_idx = 0;

    // Iterate over each row
    for (row_idx, row) in pixels.iter().enumerate() {
        // And each column
        for (col_idx, pixel) in row.iter().enumerate() {
            // If the sprite overlaps with (row_idx, col_idx), we set pixel to '#'

            // Check if we need to go to the next instruction
        }
    }

    // Convert to Strings and return
    pixels
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day10.txt").expect("Failed to read day 10 input file");

    // Parse the input into a vector of numbers
    let input = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = part2(input);
    // println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction1() {
        let s = "addx 3";
        let want = Instruction::Addx(3);
        let got = Instruction::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_instruction2() {
        let s = "addx -5";
        let want = Instruction::Addx(-5);
        let got = Instruction::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_instruction3() {
        let s = "noop";
        let want = Instruction::Noop;
        let got = Instruction::from(s);
        assert_eq!(want, got);
    }

    #[test]
    fn test_step_basic() {
        let start = CPU::new();
        let want = CPU {
            cycle: 1,
            register_during: 1,
            register_after: 1,
        };
        // Apply noop
        let got = start.step(Instruction::Noop);
        assert_eq!(want, got);

        // Apply addx 3
        let got = want.step(Instruction::Addx(3));
        let want = CPU {
            cycle: 3,
            register_during: 1,
            register_after: 4,
        };
        assert_eq!(want, got);

        // Apply addx -5
        let got = want.step(Instruction::Addx(-5));
        let want = CPU {
            cycle: 5,
            register_during: 4,
            register_after: -1,
        };
        assert_eq!(want, got);
    }

    const INPUT_STR: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        let input = parse(INPUT_STR);
        let want = 13140;
        let got = part1(&input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT_STR);
        let want: Vec<String> = vec![
            "##..##..##..##..##..##..##..##..##..##..".to_string(),
            "###...###...###...###...###...###...###.".to_string(),
            "####....####....####....####....####....".to_string(),
            "#####.....#####.....#####.....#####.....".to_string(),
            "######......######......######......####".to_string(),
            "#######.......#######.......#######.....".to_string(),
        ];
        let got = part2(&input);
        assert_eq!(want, got);
    }
}
