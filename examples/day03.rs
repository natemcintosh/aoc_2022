use std::collections::HashSet;

fn parse_input(input_str: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input_str
        .lines()
        .map(|line| {
            let h1: HashSet<char> = line.chars().take(line.len() / 2).collect();
            let h2: HashSet<char> = line.chars().rev().take(line.len() / 2).collect();
            (h1, h2)
        })
        .collect()
}

fn part1(packs: &[(HashSet<char>, HashSet<char>)]) -> usize {
    packs.iter().map(|(p1, p2)| single_pack(p1, p2)).sum()
}

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn single_pack(p1: &HashSet<char>, p2: &HashSet<char>) -> usize {
    // Find the overlap of the two hash sets
    let overlap = *p1.intersection(p2).next().expect("No intersection");
    // Find it in the string.
    LETTERS.find(overlap).expect("Letter is not in alphabet") + 1
}

fn part2(packs: &[(HashSet<char>, HashSet<char>)]) -> usize {
    packs.chunks(3).map(group_id).sum()
}

fn group_id(chunk: &[(HashSet<char>, HashSet<char>)]) -> usize {
    // Union the two halves of each line, and then find the intersection of each
    // group of three, and find its priority
    let e1: HashSet<char> = chunk[0].0.union(&chunk[0].1).copied().collect();
    let e2: HashSet<char> = chunk[1].0.union(&chunk[1].1).copied().collect();
    let e3: HashSet<char> = chunk[2].0.union(&chunk[2].1).copied().collect();

    let temp: HashSet<char> = e1.intersection(&e2).copied().collect();
    let overlap = temp
        .intersection(&e3)
        .next()
        .expect("No intersection in this group");

    LETTERS.find(*overlap).expect("Letter is not in alphabet") + 1
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day03.txt").expect("Failed to read day 1 input file");

    // Parse the input into a vector of numbers
    let input = parse_input(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

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
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let want = vec![
            (
                HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']),
                HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']),
            ),
            (
                HashSet::from([
                    'j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L',
                ]),
                HashSet::from([
                    'r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L',
                ]),
            ),
            (
                HashSet::from(['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V']),
                HashSet::from(['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g']),
            ),
            (
                HashSet::from([
                    'w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H',
                ]),
                HashSet::from([
                    'j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n',
                ]),
            ),
            (
                HashSet::from(['t', 't', 'g', 'J', 't', 'R', 'G', 'J']),
                HashSet::from(['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T']),
            ),
            (
                HashSet::from(['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z']),
                HashSet::from(['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w']),
            ),
        ];

        let got = parse_input(input_str);
        assert_eq!(want, got);
    }

    #[test]
    fn test_single_pack() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let input = parse_input(input_str);
        let want: [usize; 6] = [16, 38, 42, 22, 20, 19];
        for (&want, got) in want.iter().zip(input.iter()) {
            assert_eq!(want, single_pack(&got.0, &got.1));
        }
    }

    #[test]
    fn test_part1() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let input = parse_input(input_str);
        let want = 157;
        let got = part1(&input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let input = parse_input(input_str);
        let want = 70;
        let got = part2(&input);
        assert_eq!(want, got);
    }
}
