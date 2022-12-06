use itertools::Itertools;

fn parse(input_str: &str) -> Vec<char> {
    input_str.trim().chars().collect()
}

fn solve(input: &[char], n_unique_chars: usize) -> usize {
    for (idx, window) in input.windows(n_unique_chars).enumerate() {
        if window.iter().unique().count() == n_unique_chars {
            return idx + n_unique_chars;
        }
    }
    0
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day06.txt").expect("Failed to read day 6 input file");

    // Parse the input into a vector of numbers
    let input = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = solve(&input, 4);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = solve(&input, 14);
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect_vec(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect_vec(),
            "nppdvjthqldpwncqszvftbrmjlhg".chars().collect_vec(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect_vec(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect_vec(),
        ];
        let want = vec![7, 5, 6, 10, 11];
        for (want, got) in want.iter().zip(inputs.iter()) {
            assert_eq!(*want, solve(got, 4));
        }
    }

    #[test]
    fn test_part2() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect_vec(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect_vec(),
            "nppdvjthqldpwncqszvftbrmjlhg".chars().collect_vec(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect_vec(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect_vec(),
        ];
        let want = vec![19, 23, 23, 29, 26];
        for (want, got) in want.iter().zip(inputs.iter()) {
            assert_eq!(*want, solve(got, 14));
        }
    }
}
