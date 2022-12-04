fn parse(input_str: &str) -> Vec<(Set, Set)> {
    input_str
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once(',').expect("Could not split on ','");
            (Set::parse(s1), Set::parse(s2))
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Set(u8, u8);

impl Set {
    fn parse(input_str: &str) -> Set {
        let (lo_s, hi_s) = input_str.split_once('-').expect("Could not split on dash");
        let lo = lo_s.parse().expect("Could not parse low u8");
        let hi = hi_s.parse().expect("Could not parse high u8");
        Set(lo, hi)
    }

    /// If both the low and the high of one are equal or beyond the other
    fn either_is_proper_subset(self, other: Set) -> bool {
        if ((self.0 <= other.0) && (self.1 >= other.1))
            || ((other.0 <= self.0) && (other.1 >= self.1))
        {
            return true;
        }
        false
    }

    fn any_overlap(self, other: Set) -> bool {
        // If the high of one is below the low of the other no overlap, and vice versa
        !((self.1 < other.0) || (other.1 < self.0))
    }
}

fn part1(input: &[(Set, Set)]) -> usize {
    input
        .iter()
        .filter(|(s1, s2)| s1.either_is_proper_subset(*s2))
        .count()
}

fn part2(input: &[(Set, Set)]) -> usize {
    input.iter().filter(|(s1, s2)| s1.any_overlap(*s2)).count()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day04.txt").expect("Failed to read day 4 input file");

    // Parse the input into a vector of numbers
    let input = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input);
    println!("Part 2 took {:.6} ns", part2_time.elapsed().as_nanos());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input_str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let want = vec![
            (Set(2, 4), Set(6, 8)),
            (Set(2, 3), Set(4, 5)),
            (Set(5, 7), Set(7, 9)),
            (Set(2, 8), Set(3, 7)),
            (Set(6, 6), Set(4, 6)),
            (Set(2, 6), Set(4, 8)),
        ];
        let got = parse(input_str);
        assert_eq!(want, got);
    }

    #[test]
    fn test_either_is_proper_subset() {
        let input = vec![
            (Set(2, 4), Set(6, 8)),
            (Set(2, 3), Set(4, 5)),
            (Set(5, 7), Set(7, 9)),
            (Set(2, 8), Set(3, 7)),
            (Set(6, 6), Set(4, 6)),
            (Set(2, 6), Set(4, 8)),
        ];
        let want = [false, false, false, true, true, false];
        let got: Vec<bool> = input
            .iter()
            .map(|(s1, s2)| s1.either_is_proper_subset(*s2))
            .collect();
        for (w, g) in want.iter().zip(got.iter()) {
            assert_eq!(w, g);
        }
    }

    #[test]
    fn test_part1() {
        let input = vec![
            (Set(2, 4), Set(6, 8)),
            (Set(2, 3), Set(4, 5)),
            (Set(5, 7), Set(7, 9)),
            (Set(2, 8), Set(3, 7)),
            (Set(6, 6), Set(4, 6)),
            (Set(2, 6), Set(4, 8)),
        ];
        let want = 2;
        let got = part1(&input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            (Set(2, 4), Set(6, 8)),
            (Set(2, 3), Set(4, 5)),
            (Set(5, 7), Set(7, 9)),
            (Set(2, 8), Set(3, 7)),
            (Set(6, 6), Set(4, 6)),
            (Set(2, 6), Set(4, 8)),
        ];
        let want = 4;
        let got = part2(&input);
        assert_eq!(want, got);
    }
}
