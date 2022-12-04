#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn parse(s: &str) -> RPS {
        match s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Input not A, B, C, X, Y, or Z"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Strat {
    Lost,
    Tie,
    Win,
}

impl Strat {
    fn parse(s: &str) -> Strat {
        match s {
            "X" => Strat::Lost,
            "Y" => Strat::Tie,
            "Z" => Strat::Win,
            _ => panic!("Input not X, Y, or Z"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RPSPair {
    them: RPS,
    us: RPS,
}

impl RPSPair {
    fn play_round(&self) -> usize {
        let score_of_move = match self.us {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let round_winner = match (self.us, self.them) {
            // Tie gives 3 points
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
            (RPS::Rock, RPS::Paper) => 0,
            (RPS::Rock, RPS::Scissors) => 6,
            (RPS::Paper, RPS::Rock) => 6,
            (RPS::Paper, RPS::Scissors) => 0,
            (RPS::Scissors, RPS::Rock) => 0,
            (RPS::Scissors, RPS::Paper) => 6,
        };

        score_of_move + round_winner
    }
}

#[derive(Debug, Clone, Copy)]
struct RPSStratPair {
    them: RPS,
    strat: Strat,
}

impl RPSStratPair {
    fn play_round(&self) -> usize {
        let round_winner = match self.strat {
            Strat::Lost => 0,
            Strat::Tie => 3,
            Strat::Win => 6,
        };

        let my_move = match (self.them, self.strat) {
            (RPS::Rock, Strat::Lost) => 3,
            (RPS::Rock, Strat::Tie) => 1,
            (RPS::Rock, Strat::Win) => 2,
            (RPS::Paper, Strat::Lost) => 1,
            (RPS::Paper, Strat::Tie) => 2,
            (RPS::Paper, Strat::Win) => 3,
            (RPS::Scissors, Strat::Lost) => 2,
            (RPS::Scissors, Strat::Tie) => 3,
            (RPS::Scissors, Strat::Win) => 1,
        };

        round_winner + my_move
    }
}

fn parse_input_part1(input_str: &str) -> Vec<RPSPair> {
    input_str
        .trim()
        .lines()
        .map(|line| {
            let parts = line
                .split_once(' ')
                .expect("Could not split once around space");
            RPSPair {
                them: RPS::parse(parts.0),
                us: RPS::parse(parts.1),
            }
        })
        .collect()
}

fn part1(plays: &[RPSPair]) -> usize {
    plays.iter().map(|p| p.play_round()).sum()
}

fn parse_input_part2(input_str: &str) -> Vec<RPSStratPair> {
    input_str
        .trim()
        .lines()
        .map(|line| {
            let parts = line
                .split_once(' ')
                .expect("Could not split once around space");
            RPSStratPair {
                them: RPS::parse(parts.0),
                strat: Strat::parse(parts.1),
            }
        })
        .collect()
}

fn part2(plays: &[RPSStratPair]) -> usize {
    plays.iter().map(|p| p.play_round()).sum()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day02.txt").expect("Failed to read day 2 input file");

    // Parse the input into a vector of numbers
    let input1 = parse_input_part1(&input_str);
    let input2 = parse_input_part2(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input1);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input2);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() {
        let input_str = "A Y
B X
C Z";
        let want = vec![
            RPSPair {
                them: RPS::Rock,
                us: RPS::Paper,
            },
            RPSPair {
                them: RPS::Paper,
                us: RPS::Rock,
            },
            RPSPair {
                them: RPS::Scissors,
                us: RPS::Scissors,
            },
        ];
        let got = parse_input_part1(input_str);
        assert_eq!(want, got);
    }

    #[test]
    fn test_round1_winner() {
        let p = RPSPair {
            them: RPS::Rock,
            us: RPS::Paper,
        };
        assert_eq!(8, p.play_round());

        let p = RPSPair {
            them: RPS::Paper,
            us: RPS::Rock,
        };
        assert_eq!(1, p.play_round());

        let p = RPSPair {
            them: RPS::Scissors,
            us: RPS::Scissors,
        };
        assert_eq!(6, p.play_round());
    }

    #[test]
    fn test_part1() {
        let plays = vec![
            RPSPair {
                them: RPS::Rock,
                us: RPS::Paper,
            },
            RPSPair {
                them: RPS::Paper,
                us: RPS::Rock,
            },
            RPSPair {
                them: RPS::Scissors,
                us: RPS::Scissors,
            },
        ];
        let want = 15;
        let got = part1(&plays);
        assert_eq!(want, got);
    }

    #[test]
    fn test_round2_winner() {
        let p = RPSStratPair {
            them: RPS::Rock,
            strat: Strat::Tie,
        };
        assert_eq!(4, p.play_round());

        let p = RPSStratPair {
            them: RPS::Paper,
            strat: Strat::Lost,
        };
        assert_eq!(1, p.play_round());

        let p = RPSStratPair {
            them: RPS::Scissors,
            strat: Strat::Win,
        };
        assert_eq!(7, p.play_round());
    }
}
