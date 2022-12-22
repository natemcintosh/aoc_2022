use std::collections::HashSet;

use itertools::Itertools;

fn parse_line(line: &str) -> impl Iterator<Item = Point> + '_ {
    line.split(" -> ")
        .map(Point::parse)
        .tuple_windows()
        .flat_map(|(p1, p2)| p1.gen_points_between(p2))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    /// Assumes `s_pt` looks like "504,23"
    fn parse(s_pt: &str) -> Point {
        let (x, y) = s_pt.split_once(',').expect("No comma to split point on");
        Point {
            x: x.parse().expect("could not parse to u16"),
            y: y.parse().expect("could not parse to u16"),
        }
    }

    /// Take two points, and draw a straight line between them. Assumes points will
    /// always either be above each eachother (vertical line) or side to side
    /// (horizontal line)
    fn gen_points_between(&self, o: Point) -> Vec<Point> {
        // If xs are the same, then vertical
        if self.x == o.x {
            let (ysmall, ybig) = if self.y < o.y {
                (self.y, o.y)
            } else {
                (o.y, self.y)
            };
            return (ysmall..=ybig)
                .into_iter()
                .map(|y| Point { x: self.x, y })
                .collect();
        } else if self.y == o.y {
            // the ys are the same. Move horizontally
            let (xsmall, xbig) = if self.x < o.x {
                (self.x, o.x)
            } else {
                (o.x, self.x)
            };
            return (xsmall..=xbig)
                .into_iter()
                .map(|x| Point { x, y: self.y })
                .collect();
        }
        unreachable!("Points are not vertical or horizontal")
    }
}

/// Find the next place a unit of sand will fall to.
/// First attempts to go directly down, then down and left, then down and right
/// If those three spots exist in `walls`, then return None
fn next_point(walls: &HashSet<Point>, p: Point) -> Option<Point> {
    // Check down
    let d = Point { x: p.x, y: p.y + 1 };
    if !walls.contains(&d) {
        return Some(d);
    }

    let l = Point {
        x: p.x - 1,
        y: p.y + 1,
    };
    if !walls.contains(&l) {
        return Some(l);
    }

    let r = Point {
        x: p.x + 1,
        y: p.y + 1,
    };
    if !walls.contains(&r) {
        return Some(r);
    }

    None
}

enum SandState {
    AtRest,
    Abyss,
}

/// Call new_point continuously until can't go any further, or fall below bottom
fn fall_forever(walls: &HashSet<Point>, point: Point, bottom: u16) -> (SandState, Point) {
    let mut pt = point;

    // Propagate the point. If it is None, then return AtRest and current point
    // If it is a point, check if it is below the bottom
    // If it is below the bottom, return Abyss, and the current state
    // Else keep propagating
    loop {
        match next_point(walls, pt) {
            None => {
                return (SandState::AtRest, pt);
            }
            Some(p) => {
                if p.y >= bottom {
                    return (SandState::Abyss, p);
                } else {
                    pt = p;
                }
            }
        }
    }
}

/// The source of sand is (500, 0). Count how many units of sand fall before all further
/// sand falls into the abyss
fn part1(walls: &HashSet<Point>) -> usize {
    let source = Point { x: 500, y: 0 };
    let lowest_wall = walls.iter().map(|p| p.y).max().expect("No wall points");
    let mut blockers = walls.clone();

    let mut ctr = 0;
    loop {
        // Keep dropping sand until we get an abyss
        // When we receive an AtRest, add that point to the blockers
        match fall_forever(&blockers, source, lowest_wall) {
            (SandState::Abyss, _) => {
                return ctr;
            }
            (SandState::AtRest, pt) => {
                blockers.insert(pt);
            }
        }
        ctr += 1;
    }
}

/// Find the next place a unit of sand will fall to.
/// First attempts to go directly down, then down and left, then down and right
/// If those three spots exist in `walls`, then return None
fn next_point_w_floor(walls: &HashSet<Point>, p: Point, floor_height: u16) -> Option<Point> {
    // Check down
    let d = Point { x: p.x, y: p.y + 1 };
    if !walls.contains(&d) && (d.y < floor_height) {
        return Some(d);
    }

    // Check left
    let l = Point {
        x: p.x - 1,
        y: p.y + 1,
    };
    if !walls.contains(&l) && (l.y < floor_height) {
        return Some(l);
    }

    // Check right
    let r = Point {
        x: p.x + 1,
        y: p.y + 1,
    };
    if !walls.contains(&r) && (r.y < floor_height) {
        return Some(r);
    }

    // No valid point, at rest
    None
}

fn fall_to_floor(walls: &HashSet<Point>, point: Point, floor_height: u16) -> Point {
    let mut pt = point;

    // Propagate the point. If it is None, return the point
    loop {
        match next_point_w_floor(walls, pt, floor_height) {
            Some(p) => {
                pt = p;
            }
            None => {
                return pt;
            }
        }
    }
}

/// There is a floor 2 units below (at higher y index) the lowest wall that extends
/// infinitely in either direction
fn part2(walls: &HashSet<Point>) -> usize {
    let source = Point { x: 500, y: 0 };
    let lowest_wall = walls.iter().map(|p| p.y).max().expect("No wall points");
    let mut blockers = walls.clone();
    let floor_height = lowest_wall + 2;

    let mut ctr = 0;
    loop {
        ctr += 1;
        // Fall until the returned point is the source
        let new_pt = fall_to_floor(&blockers, source, floor_height);
        blockers.insert(new_pt);
        if new_pt == source {
            return ctr;
        }
    }
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day14.txt").expect("Failed to read day 14 input file");

    // Parse the input into a vector of numbers
    let walls: HashSet<Point> = input_str.lines().flat_map(parse_line).collect();

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(&walls);
    println!("Part 1 took {:.6} ms", part1_time.elapsed().as_millis());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(&walls);
    println!("Part 2 took {:.6} ms", part2_time.elapsed().as_millis());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let walls: HashSet<Point> = input_str.lines().flat_map(parse_line).collect();

        let want = 24;
        let got = part1(&walls);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let input_str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let walls: HashSet<Point> = input_str.lines().flat_map(parse_line).collect();

        let want = 93;
        let got = part2(&walls);
        assert_eq!(want, got);
    }
}
