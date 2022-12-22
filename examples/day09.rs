use itertools::Itertools;
use std::collections::HashSet;

fn parse(input_str: &str) -> Vec<Instruction> {
    input_str.lines().map(Instruction::parse).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn parse(input_str: &str) -> Dir {
        match input_str {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!("Found uknown direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    dir: Dir,
    count: u8,
}

impl Instruction {
    fn parse(input_str: &str) -> Instruction {
        let (dir_s, n_s) = input_str.split_once(' ').expect("No space to split on");
        let dir = Dir::parse(dir_s);
        let count: u8 = n_s.parse().expect("Could not parse to u8");
        Instruction { dir, count }
    }
}

/// Item 0 is the x coordinate, item 1 is the y coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point(i16, i16);

impl Point {
    /// L2 norm
    fn dist(&self, other: &Point) -> f32 {
        let s: f32 = (((self.0 - other.0) * (self.0 - other.0))
            + ((self.1 - other.1) * (self.1 - other.1)))
            .into();
        s.sqrt()
    }

    /// Move the point in the desired direction by 1 unit
    fn step(&self, d: Dir) -> Point {
        match d {
            // y increments
            Dir::U => Point(self.0, self.1 + 1),
            // y decrements
            Dir::D => Point(self.0, self.1 - 1),
            // x decrements
            Dir::L => Point(self.0 - 1, self.1),
            // x increments
            Dir::R => Point(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    /// Calculate the angle from the tail to head, relative to the x-axis
    /// [-pi/4, pi/4) => right
    /// [pi/4, 3pi/4) => up
    /// [3pi/4, 5pi/4) => left
    /// [5pi/4, 7pi/4) => down
    fn relative_direction(&self) -> Dir {
        let y: f32 = (self.head.1 - self.tail.1).into();
        let x: f32 = (self.head.0 - self.tail.0).into();
        let mut angle = y.atan2(x);

        // Make everything positive to make checking ranges easier
        if angle < 0.0 {
            angle += 2.0 * std::f32::consts::PI;
        }

        if (((7.0 / 4.0) * std::f32::consts::PI)..(2.0 * std::f32::consts::PI)).contains(&angle)
            || (0.0..std::f32::consts::FRAC_PI_4).contains(&angle)
        {
            Dir::R
        } else if (1.0 * std::f32::consts::FRAC_PI_4..(3.0 * std::f32::consts::FRAC_PI_4))
            .contains(&angle)
        {
            Dir::U
        } else if (3.0 * std::f32::consts::FRAC_PI_4..(5.0 * std::f32::consts::FRAC_PI_4))
            .contains(&angle)
        {
            Dir::L
        } else {
            Dir::D
        }
    }

    /// Carry out one step in the given direction, and return the new rope position
    fn one_step(&self, d: Dir) -> Rope {
        // Move the head in the given direction
        let new_head = self.head.step(d);

        // If the tail is more than 1.5 units away, it moves behind the head, otherwise
        // does not move
        if new_head.dist(&self.tail) <= 1.5 {
            Rope {
                head: new_head,
                tail: self.tail,
            }
        } else {
            let new_tail = match d {
                // head moved up, tail is one y unit below it
                Dir::U => Point(new_head.0, new_head.1 - 1),
                // head moved down, tail is one y unit above it
                Dir::D => Point(new_head.0, new_head.1 + 1),
                // head moved left, tail is now one x unit above it
                Dir::L => Point(new_head.0 + 1, new_head.1),
                // head moved right, tail is now one x unit below it
                Dir::R => Point(new_head.0 - 1, new_head.1),
            };
            Rope {
                head: new_head,
                tail: new_tail,
            }
        }
    }

    /// Where you direct a coordinate for the head, and the tail will go to the correct place
    fn move_head_and_have_tail_follow(&self, new_head: Point) -> Rope {
        // If the tail is more than 1.5 units away, it moves behind the head, otherwise
        // does not move
        if new_head.dist(&self.tail) <= 1.5 {
            Rope {
                head: new_head,
                tail: self.tail,
            }
        } else {
            let relative_direction: Dir = self.relative_direction();
            let new_tail = match relative_direction {
                // head moved up, tail is one y unit below it
                Dir::U => Point(new_head.0, new_head.1 - 1),
                // head moved down, tail is one y unit above it
                Dir::D => Point(new_head.0, new_head.1 + 1),
                // head moved left, tail is now one x unit above it
                Dir::L => Point(new_head.0 + 1, new_head.1),
                // head moved right, tail is now one x unit below it
                Dir::R => Point(new_head.0 - 1, new_head.1),
            };
            Rope {
                head: new_head,
                tail: new_tail,
            }
        }
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    // The rope starts with had and tail at (0, 0)
    let mut rope = Rope {
        head: Point(0, 0),
        tail: Point(0, 0),
    };
    let mut visited: HashSet<Point> = HashSet::new();

    // Carry out the instructions
    for inst in instructions {
        for _ in 0..inst.count {
            rope = rope.one_step(inst.dir);
            visited.insert(rope.tail);
        }
    }
    visited.len()
}

#[derive(Debug, Default)]
struct LongRope {
    segments: [Rope; 10],
}

impl LongRope {
    fn one_step(&self, d: Dir) -> LongRope {
        let mut new_rope = LongRope::default();

        // Move the head in the given direction
        new_rope.segments[0] = self.segments[0].one_step(d);

        for (idx, (s1, s2)) in self.segments.iter().tuple_windows().enumerate() {
            /*
            In following behavior, the tail of s1 is the head of s2
            So we need to step s1, then update the head position of s2 to be
            the tail of s1, then update the tail of s2
            */
            let nh2 = s1.head;
            let ns2 = s2.move_head_and_have_tail_follow(nh2);
            new_rope.segments[idx + 1] = ns2;
        }

        new_rope
    }
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut rope = LongRope::default();
    let mut visited: HashSet<Point> = HashSet::new();

    for instr in instructions {
        for _ in 0..instr.count {
            rope = rope.one_step(instr.dir);
            visited.insert(rope.segments[9].tail);
        }
    }

    visited.len()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day09.txt").expect("Failed to read day 9 input file");

    // Parse the input into a vector of numbers
    let input = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

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
    fn test_parse() {
        let input_str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let want = vec![
            Instruction {
                dir: Dir::R,
                count: 4,
            },
            Instruction {
                dir: Dir::U,
                count: 4,
            },
            Instruction {
                dir: Dir::L,
                count: 3,
            },
            Instruction {
                dir: Dir::D,
                count: 1,
            },
            Instruction {
                dir: Dir::R,
                count: 4,
            },
            Instruction {
                dir: Dir::D,
                count: 1,
            },
            Instruction {
                dir: Dir::L,
                count: 5,
            },
            Instruction {
                dir: Dir::R,
                count: 2,
            },
        ];
        let got = parse(input_str);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part1() {
        let input_str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let instructions = parse(input_str);
        let want = 13;
        let got = part1(&instructions);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input_str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let instructions = parse(input_str);
        let want = 0;
        let got = part2(&instructions);
        assert_eq!(want, got);
    }
}
