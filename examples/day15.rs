use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// The Manhattan distance between two points
    fn man_dist(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    }

    let mut result: [Point; 2] = [Point::default(); 2];
    for (idx, cap) in RE.captures_iter(line).take(2).enumerate() {
        let x: i64 = cap[1].parse().expect("Could not parse to i64");
        let y: i64 = cap[2].parse().expect("Could not parse to i64");
        result[idx] = Point { x, y };
    }
    (result[0], result[1])
}

/// How many spots in `row` cannot have a beacon in them?
fn part1(sensors: &[Point], beacons: &[Point], row: i64) -> usize {
    // Find any beacons in the row in question
    let beacons_in_row: HashSet<Point> = beacons.iter().filter(|&p| p.y == row).copied().collect();

    let mut empty_spots: HashSet<Point> = HashSet::new();

    // For each sensor, get its empty range, the range in which there is only one sensor,
    // right at the edge.
    let ranges = sensors
        .iter()
        .zip(beacons.iter())
        .map(|(sen, beac)| sen.man_dist(beac));

    // For each sensor and distance, check if `row` is in range.
    // If it is in range, add points to either side until they aren't in range.
    sensors
        .iter()
        .zip(ranges)
        .filter(|(sen, dist)| {
            // Get the point on the line
            let pt_on_line = Point { x: sen.x, y: row };
            sen.man_dist(&pt_on_line) <= *dist
        })
        .for_each(|(sen, dist)| {
            // Add the point directly above or below
            empty_spots.insert(Point { x: sen.x, y: row });

            // Add points with greater x value until no longer in range
            (sen.x..)
                .into_iter()
                // Create the point
                .map(|x| Point { x, y: row })
                // Take while the points are in range
                .take_while(|pt| sen.man_dist(pt) <= dist)
                // Add them to the empty spots
                .for_each(|pt| {
                    empty_spots.insert(pt);
                });

            // Add points with smaller x value until no longer in range
            (i64::MIN..sen.x)
                .rev()
                // Create the point
                .map(|x| Point { x, y: row })
                // Take while the points are in range
                .take_while(|pt| sen.man_dist(pt) <= dist)
                // Add them to the empty spots
                .for_each(|pt| {
                    empty_spots.insert(pt);
                });
        });

    // At the end, remove the `beacons_in_row` from the `empty_spots`
    empty_spots.difference(&beacons_in_row).count()
}

// fn gen_boundary(p: Point, radius: i64, lb: usize, ub: usize) -> impl Iterator<Item = Point> {

// }

/// Idea is to check the perimeter of each keep out zone.
fn part2(sensors: &[Point], beacons: &[Point], lb: usize, ub: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;

    // For each sensor, get its empty range, the range in which there is only one sensor,
    // right at the edge.
    let ranges: Vec<i64> = sensors
        .iter()
        .zip(beacons.iter())
        .map(|(sen, beac)| sen.man_dist(beac))
        .collect();

    // For each sensor, generate the square around it

    // The tuning frequency is 4_000_000*x + y
    (x * 4_000_000) + y
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day15.txt").expect("Failed to read day 15 input file");

    // Parse the input
    let pts: Vec<(Point, Point)> = input_str.lines().map(parse_line).collect();
    let sensors: Vec<Point> = pts.iter().map(|(pt, _)| pt).copied().collect();
    let beacons: Vec<Point> = pts.iter().map(|(_, pt)| pt).copied().collect();

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&sensors, &beacons, 2000000);
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    // let part2_time = std::time::Instant::now();
    // let part2_result = part2(&walls);
    // println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input_str = [
            "Sensor at x=2793338, y=1910659: closest beacon is at x=2504930, y=2301197",
            "Sensor at x=2887961, y=129550: closest beacon is at x=2745008, y=-872454",
        ];
        let want = [
            (
                Point {
                    x: 2793338,
                    y: 1910659,
                },
                Point {
                    x: 2504930,
                    y: 2301197,
                },
            ),
            (
                Point {
                    x: 2887961,
                    y: 129550,
                },
                Point {
                    x: 2745008,
                    y: -872454,
                },
            ),
        ];
        for (ins, w) in input_str.iter().zip(want.iter()) {
            let got = parse_line(ins);
            assert_eq!(*w, got);
        }
    }

    #[test]
    fn test_part1() {
        let input_str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let pts: Vec<(Point, Point)> = input_str.lines().map(parse_line).collect();
        let sensors: Vec<Point> = pts.iter().map(|(pt, _)| pt).copied().collect();
        let beacons: Vec<Point> = pts.iter().map(|(_, pt)| pt).copied().collect();

        let want = 26;
        let got = part1(&sensors, &beacons, 10);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input_str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let pts: Vec<(Point, Point)> = input_str.lines().map(parse_line).collect();
        let sensors: Vec<Point> = pts.iter().map(|(pt, _)| pt).copied().collect();
        let beacons: Vec<Point> = pts.iter().map(|(_, pt)| pt).copied().collect();

        let want = 5166077;
        let got = part2(&sensors, &beacons, 0, 4_000_000);
        assert_eq!(want, got);
    }
}
