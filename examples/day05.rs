#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    number: u8,
    from: usize,
    to: usize,
}

impl Instruction {
    /// Average instruction might look like
    /// `move 11 from 9 to 3`
    fn parse(input_str: &str) -> Instruction {
        let mut iter = input_str.split_ascii_whitespace();
        // Skip `move`
        iter.next();

        // Gather the number
        let number: u8 = iter
            .next()
            .expect("Could not get the number")
            .parse()
            .expect("Could not parse number of crates to u8");

        // Skip `from`
        iter.next();

        // Gather the `from` number
        let from: usize = iter
            .next()
            .expect("Could not get 'from' number")
            .parse()
            .expect("Could not parse from crate number to usize");

        // Skip `to`
        iter.next();

        // Gather the `to` number
        let to: usize = iter
            .next()
            .expect("Could not get 'to' number")
            .parse()
            .expect("Could not parse to crate number to usize");

        Instruction {
            number,
            from: from - 1,
            to: to - 1,
        }
    }
}

/// Input string has general form
/// ```
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
/// ```
fn parse(input_str: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    // First try to split on the double newline
    let (raw_crates, raw_instructions) = input_str
        .split_once("\n\n")
        .expect("Could not find a double newline to split on");

    // Parse the instructions
    let instructions: Vec<Instruction> = raw_instructions
        .trim()
        .lines()
        .map(Instruction::parse)
        .collect();

    // Flip the lines of crates, and skip the first one, that is just the numbers
    let mut crate_lines = raw_crates.lines().rev();
    // In the first row, count how many stacks there are
    let n_stacks = crate_lines
        .next()
        .expect("Could not get the line of the number of stacks")
        .trim()
        .split_ascii_whitespace()
        .count();

    // Create the set of empty stacks
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];

    // Loop over the crate lines
    for level in crate_lines {
        let line_chars: Vec<char> = level.chars().collect();
        let mut crates = line_chars.chunks(4);

        // Loop over each stack
        for this_stack in stacks.iter_mut().take(n_stacks) {
            // Get the next four characters. If they're whitespace, do nothing
            // if they're '[', 'A', ']', ' ', then capture the character and
            // push it onto the `stack_num` vec
            let possible_crate = crates
                .next()
                .expect("Could not get anything for this chunk");
            if possible_crate[0].eq(&'[') {
                this_stack.push(possible_crate[1]);
            }
        }
    }

    (stacks, instructions)
}

/// Each instruction tells us to move some number of crates from one stack to another
fn part1(stacks: &[Vec<char>], instructions: &[Instruction]) -> String {
    let mut s = stacks.to_owned();
    for instr in instructions {
        for _ in 0..instr.number {
            let transfer_crate = s[instr.from]
                .pop()
                .expect("Nothing left to pop from this stack");
            s[instr.to].push(transfer_crate);
        }
    }

    s.iter()
        .map(|v| v.last().unwrap_or(&' '))
        .copied()
        .collect::<String>()
}

fn part2(stacks: &[Vec<char>], instructions: &[Instruction]) -> String {
    let mut s = stacks.to_owned();
    for instr in instructions {
        // Pop the instr.number crates from instr.from into a vec, reverse it, push it to instr.to
        let to_transfer: Vec<char> = (0..instr.number)
            .map(|_| {
                s[instr.from]
                    .pop()
                    .expect("Nothing left to pop from this stack")
            })
            .collect();
        s[instr.to].extend(to_transfer.iter().rev());
    }

    s.iter()
        .map(|v| v.last().unwrap_or(&' '))
        .copied()
        .collect::<String>()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day05.txt").expect("Failed to read day 5 input file");

    // Parse the input into a vector of numbers
    let (stacks, instructions) = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&stacks, &instructions);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&stacks, &instructions);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions_parse() {
        let input_str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let want = vec![
            Instruction {
                number: 1,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 3,
                from: 0,
                to: 2,
            },
            Instruction {
                number: 2,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 1,
                from: 0,
                to: 1,
            },
        ];
        let got: Vec<Instruction> = input_str.lines().map(Instruction::parse).collect();

        assert_eq!(want, got);
    }

    #[test]
    fn test_parse() {
        let input_str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let want = (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![
                Instruction {
                    number: 1,
                    from: 1,
                    to: 0,
                },
                Instruction {
                    number: 3,
                    from: 0,
                    to: 2,
                },
                Instruction {
                    number: 2,
                    from: 1,
                    to: 0,
                },
                Instruction {
                    number: 1,
                    from: 0,
                    to: 1,
                },
            ],
        );
        let got = parse(input_str);

        assert_eq!(want, got);
    }

    #[test]
    fn test_part1() {
        let stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let instructions = vec![
            Instruction {
                number: 1,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 3,
                from: 0,
                to: 2,
            },
            Instruction {
                number: 2,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 1,
                from: 0,
                to: 1,
            },
        ];
        let want = String::from("CMZ");
        let got = part1(&stack, &instructions);

        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let instructions = vec![
            Instruction {
                number: 1,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 3,
                from: 0,
                to: 2,
            },
            Instruction {
                number: 2,
                from: 1,
                to: 0,
            },
            Instruction {
                number: 1,
                from: 0,
                to: 1,
            },
        ];
        let want = String::from("MCD");
        let got = part2(&stack, &instructions);

        assert_eq!(want, got);
    }
}
