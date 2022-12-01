use itertools::Itertools;

fn parse(input_str: &str) -> Vec<usize> {
    let output: Vec<Vec<usize>> = input_str
        .split("\n\n")
        .map(|rations| {
            rations
                .split('\n')
                .map(|line| line.parse().expect("Could not parse number"))
                .collect()
        })
        .collect();

    output.iter().map(|rations| rations.iter().sum()).collect()
}

fn part1(input: &[usize]) -> usize {
    // For each inner vector, calculate its sum, then return the max value
    *input.iter().max().expect("Input was entirely empty")
}

fn part2(input: &[usize]) -> usize {
    // For each inner vector, calculate its sum, then sort them, and calculate the
    // sum of the largest three
    input
        .iter()
        .sorted_unstable_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum()
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day01.txt").expect("Failed to read day 1 input file");

    // Parse the input into a vector of numbers
    let input = parse(&input_str);
    dbg!(&input);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} ns", part1_time.elapsed().as_nanos());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&input);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input_str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let got = parse(input_str);
        let want = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(want, got);
    }

    #[test]
    fn test_part1() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        let want: usize = 24000;
        let got = part1(&input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        let want: usize = 45000;
        let got = part2(&input);
        assert_eq!(want, got);
    }
}
