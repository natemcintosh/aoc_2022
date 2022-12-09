use ndarray::{Array2, ArrayView2};
use std::cmp::Ordering;

fn parse(input_str: &str) -> Array2<u8> {
    let v: Vec<Vec<u8>> = input_str
        .lines()
        .map(|line| line.as_bytes().iter().map(|n| n - 48).collect())
        .collect();

    let mut result: Array2<u8> = Array2::<u8>::zeros((v.len(), v[0].len()));

    for (ridx, row) in v.iter().enumerate() {
        for (cidx, item) in row.iter().enumerate() {
            result[(ridx, cidx)] = *item;
        }
    }
    result
}

/// True if all the items directly above the idx are less than it
fn visible_from_top(arr: ArrayView2<u8>, idx: (usize, usize)) -> bool {
    (0..idx.0)
        .into_iter()
        .map(|ridx| (ridx, idx.1))
        .all(|new_idx| arr[idx] > arr[new_idx])
}

/// True if all the items directly to the left of the idx are less than it
fn visible_from_left(arr: ArrayView2<u8>, idx: (usize, usize)) -> bool {
    (0..idx.1)
        .into_iter()
        .map(|cidx| (idx.0, cidx))
        .all(|new_idx| arr[idx] > arr[new_idx])
}

/// True if all the items directly to the right of the idx are less than it
fn visible_from_right(arr: ArrayView2<u8>, idx: (usize, usize)) -> bool {
    if idx.1 == (arr.ncols() - 1) {
        return true;
    }
    ((idx.1 + 1)..arr.ncols())
        .into_iter()
        .map(|cidx| (idx.0, cidx))
        .all(|new_idx| arr[idx] > arr[new_idx])
}

/// True if all the items directly below the idx are less than it
fn visible_from_bottom(arr: ArrayView2<u8>, idx: (usize, usize)) -> bool {
    if idx.0 == (arr.nrows() - 1) {
        return true;
    }

    ((idx.0 + 1)..arr.nrows())
        .into_iter()
        .map(|ridx| (ridx, idx.1))
        .all(|new_idx| arr[idx] > arr[new_idx])
}

fn part1(arr: ArrayView2<u8>) -> usize {
    arr.indexed_iter()
        // It gets through the filter if it is visible
        .filter(|(idx, _)| {
            visible_from_top(arr, *idx)
                || visible_from_bottom(arr, *idx)
                || visible_from_left(arr, *idx)
                || visible_from_right(arr, *idx)
        })
        .count()
}

/// What are the indices in the array that are directly above this `idx`
fn inds_above(idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..idx.0).rev().map(move |ridx| (ridx, idx.1))
}

/// What are the indices in the array that are directly to the left of this `idx`
fn inds_left(idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..idx.1).rev().map(move |cidx| (idx.0, cidx))
}

/// What are the indices in the array that are directly below this idx
fn inds_below(arr: ArrayView2<u8>, idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let last_idx = arr.nrows() - 1;
    ((idx.0 + 1)..=last_idx).map(move |ridx| (ridx, idx.1))
}

fn inds_right(arr: ArrayView2<u8>, idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let last_idx = arr.ncols() - 1;
    ((idx.1 + 1)..=last_idx).map(move |cidx| (idx.0, cidx))
}

fn count_visible_trees(
    arr: ArrayView2<u8>,
    viewer: (usize, usize),
    inds: &[(usize, usize)],
) -> usize {
    let mut score = 0;
    for idx in inds {
        match arr[*idx].cmp(&arr[viewer]) {
            Ordering::Less => {
                score += 1;
            }
            Ordering::Equal | Ordering::Greater => {
                score += 1;
                break;
            }
        }
    }
    score
}

/// A tree's scenic score is found by multiplying together its viewing distance in each
/// of the four directions.
fn get_scenic_score(arr: ArrayView2<u8>, idx: (usize, usize)) -> usize {
    // If any returned iterator is empty, return 0
    let left_inds: Vec<_> = inds_left(idx).collect();
    if left_inds.is_empty() {
        return 0;
    }
    let left_score = count_visible_trees(arr, idx, &left_inds);

    let right_inds: Vec<_> = inds_right(arr, idx).collect();
    if right_inds.is_empty() {
        return 0;
    }
    let right_score = count_visible_trees(arr, idx, &right_inds);

    let above_inds: Vec<_> = inds_above(idx).collect();
    if above_inds.is_empty() {
        return 0;
    }
    let above_score = count_visible_trees(arr, idx, &above_inds);

    let below_inds: Vec<_> = inds_below(arr, idx).collect();
    if below_inds.is_empty() {
        return 0;
    }
    let below_score = count_visible_trees(arr, idx, &below_inds);

    left_score * right_score * above_score * below_score
}

/// Find the maximum scenic score
fn part2(arr: ArrayView2<u8>) -> usize {
    arr.indexed_iter()
        .map(|(idx, _)| get_scenic_score(arr, idx))
        .max()
        .expect("No items in array")
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read the input file for day 1
    let input_str =
        std::fs::read_to_string("input/day08.txt").expect("Failed to read day 8 input file");

    // Parse the input into a vector of numbers
    let arr = parse(&input_str);

    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let part1_result = part1(arr.view());
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    let part2_result = part2(arr.view());
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_parse() {
        let input_str = "30373
25512
65332
33549
35390";

        let want = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let got = parse(input_str);
        assert_eq!(want, got);
    }

    #[test]
    fn test_visible_from_top() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        // Test an item at the top, hidden, behind an item at the same height
        let indices = [(0, 0), (1, 3), (2, 1)];
        let want = [true, false, false];

        for (idx, w) in indices.iter().zip(want.iter()) {
            if *w {
                assert!(visible_from_top(arr.view(), *idx))
            } else {
                assert!(!visible_from_top(arr.view(), *idx))
            }
        }
    }

    #[test]
    fn test_visible_from_left() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        // Test an item at the left, not hidden second col, hidden, behind an item at the same height
        let indices = [(0, 0), (1, 1), (2, 1), (3, 1)];
        let want = [true, true, false, false];

        for (idx, w) in indices.iter().zip(want.iter()) {
            if *w {
                assert!(visible_from_left(arr.view(), *idx))
            } else {
                assert!(!visible_from_left(arr.view(), *idx))
            }
        }
    }

    #[test]
    fn test_visible_from_right() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        // Test an item at the right, not hidden second col, hidden, hidden
        let indices = [(0, 4), (0, 3), (1, 3), (3, 3)];
        let want = [true, true, false, false];

        for (idx, w) in indices.iter().zip(want.iter()) {
            if *w {
                assert!(visible_from_right(arr.view(), *idx))
            } else {
                assert!(!visible_from_right(arr.view(), *idx))
            }
        }
    }

    #[test]
    fn test_visible_from_bottom() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        // Test an item below, not hidden second bottom row, hidden, hidden
        let indices = [(4, 1), (3, 2), (3, 3), (3, 0)];
        let want = [true, true, false, false];

        for (idx, w) in indices.iter().zip(want.iter()) {
            if *w {
                let got = visible_from_bottom(arr.view(), *idx);
                assert!(got);
            } else {
                let got = visible_from_bottom(arr.view(), *idx);
                assert!(!got);
            }
        }
    }

    #[test]
    fn test_part1() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let want = 21;
        let got = part1(arr.view());
        assert_eq!(want, got);
    }

    #[test]
    fn test_inds_above() {
        let to_test = [0, 1, 4];
        let want = [vec![], vec![(0, 0)], vec![(3, 0), (2, 0), (1, 0), (0, 0)]];
        for (t, w) in to_test.iter().zip(want.iter()) {
            let got: Vec<_> = inds_above((*t, 0)).collect();
            assert_eq!(*w, got);
        }
    }

    #[test]
    fn test_inds_left() {
        let to_test = [0, 1, 4];
        let want = [vec![], vec![(0, 0)], vec![(0, 3), (0, 2), (0, 1), (0, 0)]];
        for (t, w) in to_test.iter().zip(want.iter()) {
            let got: Vec<_> = inds_left((0, *t)).collect();
            assert_eq!(*w, got);
        }
    }

    #[test]
    fn test_inds_below() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let to_test = [4, 3, 1];
        let want = [vec![], vec![(4, 0)], vec![(2, 0), (3, 0), (4, 0)]];
        for (t, w) in to_test.iter().zip(want.iter()) {
            let got: Vec<_> = inds_below(arr.view(), (*t, 0)).collect();
            assert_eq!(*w, got);
        }
    }

    #[test]
    fn test_inds_right() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let to_test = [4, 3, 1];
        let want = [vec![], vec![(0, 4)], vec![(0, 2), (0, 3), (0, 4)]];
        for (t, w) in to_test.iter().zip(want.iter()) {
            let got: Vec<_> = inds_right(arr.view(), (0, *t)).collect();
            assert_eq!(*w, got);
        }
    }

    #[test]
    fn test_get_scenic_score() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let idx = (1, 2);
        let want = 4;
        let got = get_scenic_score(arr.view(), idx);
        assert_eq!(want, got);

        let idx = (3, 2);
        let want = 8;
        let got = get_scenic_score(arr.view(), idx);
        assert_eq!(want, got);
    }

    #[test]
    fn test_part2() {
        let arr = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let want = 8;
        let got = part2(arr.view());
        assert_eq!(want, got);
    }
}
