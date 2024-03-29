use std::cmp::min;
use std::ops::RangeInclusive;

fn main() {
    let path = "input/input";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    println!("Number of visible trees are {visible_trees}, and the highest scenic score is {scenic_score_max}");
}

fn read_file_content(path: &str) -> Vec<Vec<u32>> {
    let input_as_string = std::fs::read_to_string(path).expect("Couldn't read file");

    input_as_string
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn find_visible_trees(path: &str) -> (u32, u32) {
    let trees_vecs = read_file_content(path);

    let mut visible_trees = 0;
    let mut scenic_score_max = 0;

    let cols = trees_vecs[0].len() - 1;
    let rows = trees_vecs.len() - 1;

    (0..=cols).for_each(|c| {
        (0..=rows).for_each(|r| {
            if (c == 0)
                || (c == cols)
                || (r == 0)
                || (r == rows)
                || is_visible(&trees_vecs, r, rows, c, cols)
            {
                visible_trees += 1;
            }

            let scenic_score = calc_scenic_score_total(&trees_vecs, r, rows, c, cols);
            if scenic_score > scenic_score_max {
                scenic_score_max = scenic_score;
            }
        })
    });
    (visible_trees, scenic_score_max)
}

/// Asses whether tree of position `r`x`c` is visible from edge.
/// rows = trees_vecs.len() - 1
fn is_visible(trees: &[Vec<u32>], r: usize, rows: usize, c: usize, cols: usize) -> bool {
    let tree = trees[r][c];
    let down = is_visible_ver(trees, tree, c, (r + 1)..=rows);
    let up = is_visible_ver(trees, tree, c, 0..=(r - 1));
    let right = is_visible_hor(trees, tree, r, (c + 1)..=cols);
    let left = is_visible_hor(trees, tree, r, 0..=(c - 1));

    down || up || left || right
}

fn is_visible_ver(trees: &[Vec<u32>], tree: u32, c: usize, range: RangeInclusive<usize>) -> bool {
    range.clone().take_while(|y| tree > trees[*y][c]).count() == range.count()
}

fn is_visible_hor(trees: &[Vec<u32>], tree: u32, r: usize, range: RangeInclusive<usize>) -> bool {
    range.clone().take_while(|x| tree > trees[r][*x]).count() == range.count()
}

fn calc_scenic_score_total(
    trees: &[Vec<u32>],
    r: usize,
    rows: usize,
    c: usize,
    cols: usize,
) -> u32 {
    let tree = trees[r][c];

    let down_score = match r != rows {
        true => {
            let trees_down = r + 1..=rows;
            calc_scenic_ver(trees, tree, c, trees_down)
        }
        false => 0,
    };

    let up_score = match r != 0 {
        true => {
            let trees_up = (0..=(r - 1)).rev();
            calc_scenic_ver(trees, tree, c, trees_up)
        }
        false => 0,
    };

    let right_score = match c != cols {
        true => {
            let trees_right = c + 1..=cols;
            calc_scenic_hor(trees, tree, r, trees_right)
        }
        false => 0,
    };

    let left_score = match c != 0 {
        true => {
            let trees_left = (0..=(c - 1)).rev();
            calc_scenic_hor(trees, tree, r, trees_left)
        }
        false => 0,
    };

    (up_score * down_score * right_score * left_score) as u32
}

fn calc_scenic_hor<T: Iterator<Item = usize> + Clone>(
    trees: &[Vec<u32>],
    tree: u32,
    r: usize,
    col_range: T,
) -> usize {
    let score = col_range
        .clone()
        .take_while(|x| tree > trees[r][*x])
        .count();
    // Makes sure to include the tree blocking the sigth, unless edge can be seen from tree
    min(score + 1, col_range.count())
}

fn calc_scenic_ver<T: Iterator<Item = usize> + Clone>(
    trees: &[Vec<u32>],
    tree: u32,
    c: usize,
    row_range: T,
) -> usize {
    let score = row_range
        .clone()
        .take_while(|y| tree > trees[*y][c])
        .count();
    // Makes sure to include the tree blocking the sigth, unless edge can be seen from tree
    min(score + 1, row_range.count())
}

#[test]
fn test_find_visible_trees_test_input() {
    let path = "input/input_test1";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    assert_eq!(21, visible_trees);
    assert_eq!(8, scenic_score_max);
}

#[test]
fn test_find_visible_trees_real_input() {
    let path = "input/input";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    assert_eq!(1785, visible_trees);
    assert_eq!(345168, scenic_score_max);
}
