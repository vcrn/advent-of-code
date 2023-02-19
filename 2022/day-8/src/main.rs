use std::ops::RangeInclusive;

fn main() {
    let path = "input/input_test1";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    println!("Number of visible trees are {visible_trees}, and the highest scenic score is {scenic_score_max}");
}

fn find_visible_trees(path: &str) -> (u32, u32) {
    let input = std::fs::read_to_string(path).expect("Couldn't read file");

    let trees_vecs = input
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut visible_trees = 0;
    let mut scenic_score_max = 0;

    let cols = trees_vecs[0].len() - 1;
    let rows = trees_vecs.len() - 1;

    //println!("Rows: {rows}, Cols: {cols}");
    for c in 0..=cols {
        for r in 0..=rows {
            //println!("r: {r}, c: {c}");
            let scenic_score = calc_scenic_score(&trees_vecs, r, rows, c, cols);
            if scenic_score > scenic_score_max {
                scenic_score_max = scenic_score;
            }
            if (c == 0)
                || (c == cols)
                || (r == 0)
                || (r == rows)
                || is_visible(&trees_vecs, r, rows, c, cols)
            {
                // Tree {r}x{c} is on the perimiter or further in and visible
                visible_trees += 1;
            }
        }
    }

    (visible_trees, scenic_score_max)
}

/// rows = trees_vecs.len() - 1
fn is_visible(trees: &[Vec<u32>], r: usize, rows: usize, c: usize, cols: usize) -> bool {
    let tree = trees[r][c];
    let down = is_visible_ver(trees, tree, c, (r + 1)..=rows);
    let up = is_visible_ver(trees, tree, c, 0..=(r - 1));
    let right = is_visible_hor(trees, tree, r, (c + 1)..=cols);
    let left = is_visible_hor(trees, tree, r, 0..=(c - 1));

    down || up || left || right
}

fn calc_scenic_score(trees: &[Vec<u32>], r: usize, rows: usize, c: usize, cols: usize) -> u32 {
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;

    let tree = trees[r][c];
    //println!("trees is {trees:?}");
    //println!("For {r}x{c}, height {tree}:");
    if r != rows {
        let trees_down = r + 1..=rows;
        //#[allow(clippy::needless_range_loop)]
        //trees.iter().take(rows + 1).skip(r + 1)
        //(1..=10).take_while(|&x| x <= 5).for_each(|x| dummy(x))
        //down = (r + 1..=rows)
        //    .take_while(|&y| tree <= trees[y][c])
        //    .for_each(|y| y)
        //    .count();
        //let it = trees.iter().take(rows + 1).skip(r + 1);
        //down = it.take_while(|&y| tree <= trees[*y][c]).count();
        down = trees_down
            .clone()
            .take_while(|y| tree > trees[*y][c])
            .count();
        if down != trees_down.count() {
            down += 1;
        }
    }
    //if r != rows {
    //    #[allow(clippy::needless_range_loop)]
    //    for y in r + 1..=rows {
    //        down += 1;
    //        //println!("y is {y}");
    //        if tree <= trees[y][c] {
    //            //println!("Blocked at {y}x{c}");
    //            break;
    //        }
    //    }
    //}
    //println!("down is {down}");

    if r != 0 {
        let trees_up = (0..=(r - 1)).rev();
        up = trees_up.clone().take_while(|y| tree > trees[*y][c]).count();
        //for y in (0..=(r - 1)).rev() {
        //    up += 1;
        //    //println!("y is {y}");
        //    if tree <= trees[y][c] {
        //        //println!("Blocked at {y}x{c}");
        //        break;
        //    }
        //}
        if up != trees_up.count() {
            up += 1;
        }
    }
    //if up != (0..=(r - 1)).count() {
    //    up += 1;
    //}

    if c != cols {
        let trees_right = c + 1..=cols;
        right = trees_right
            .clone()
            .take_while(|x| tree > trees[r][*x])
            .count();
        if right != trees_right.count() {
            right += 1;
        }
        //for x in (c + 1)..=cols {
        //    right += 1;
        //    //println!("x is {x}");
        //    if tree <= trees[r][x] {
        //        //println!("Blocked at {r}x{x}");
        //        break;
        //    }
        //}
    }

    if c != 0 {
        //for x in trees_left {
        //    left += 1;
        //    //println!("x is {x}");
        //    if tree <= trees[r][x] {
        //        //println!("Blocked at {r}x{x}");
        //        break;
        //    }
        //}
        let trees_left = (0..=(c - 1)).rev();
        left = trees_left
            .clone()
            .take_while(|x| tree > trees[r][*x])
            .count();
        if left != trees_left.count() {
            left += 1;
        }
    }
    //println!("Tree {r}x{c}: {up} {down} {right} {left}");

    up as u32 * down as u32 * right as u32 * left as u32
}

fn is_visible_ver(trees: &[Vec<u32>], tree: u32, c: usize, range: RangeInclusive<usize>) -> bool {
    for y in range {
        //println!("x is {x}");
        if tree <= trees[y][c] {
            //println!("Blocked at {r}x{x}");
            return false;
        }
    }
    true
}

fn is_visible_hor(trees: &[Vec<u32>], tree: u32, r: usize, range: RangeInclusive<usize>) -> bool {
    for x in range {
        //println!("x is {x}");
        if tree <= trees[r][x] {
            //println!("Blocked at {r}x{x}");
            return false;
        }
    }
    true
}

// TODO: Implement function below
fn _calc_scenic_hor<T: IntoIterator<Item = usize>>(
    trees: &[Vec<u32>],
    tree: u32,
    r: usize,
    col_range: T,
) -> usize {
    let mut score = 0;
    for x in col_range {
        score += 1;
        if tree <= trees[r][x] {
            break;
        }
    }
    score
}

// TODO: Implement function below
fn _calc_scenic_ver<T: IntoIterator<Item = usize>>(
    trees: &[Vec<u32>],
    tree: u32,
    c: usize,
    row_range: T,
) -> bool {
    for y in row_range {
        //println!("x is {x}");
        if tree <= trees[y][c] {
            //println!("Blocked at {r}x{x}");
            return false;
        } //else if x == 0 {
          //  return true;
          //}
    }
    true
}

#[test]
fn test_find_visible_trees_test_input() {
    let path = "input/input_test1";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    assert_eq!(21, visible_trees);
    assert_eq!(8, scenic_score_max);
    // Should be visible:
    // 1x1
    // 1x2
    // 2x1 missing
    // 2x3 missing
    // 3x2
}
#[test]
fn test_find_visible_trees_real_input() {
    let path = "input/input";
    let (visible_trees, scenic_score_max) = find_visible_trees(path);
    assert_eq!(1785, visible_trees);
    assert_eq!(345168, scenic_score_max);
    // Should be visible:
    // 1x1
    // 1x2
    // 2x1 missing
    // 2x3 missing
    // 3x2
}

// Part 1: 1785
// Part 2: 345168
