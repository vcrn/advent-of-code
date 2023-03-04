fn main() {
    let input_path = "input/input.txt";
    let (res_p1, res_p2) = compute(input_path);
    println!("Number of increases in Part 1 is {res_p1} and in Part 2 is {res_p2}");
}

fn compute(path: &str) -> (usize, usize) {
    // Read data
    let data_string = std::fs::read_to_string(path).expect("Could not read file");
    let data: Vec<usize> = data_string
        .trim() // Editors have a tendency to add trailing \n
        .split('\n')
        .map(|x| x.parse::<usize>().expect("Can't map"))
        .collect();

    // Part 1
    let increases_part1 = data
        .iter()
        .zip(data.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count();

    // Part 2
    let data_window = data.windows(3);
    let increases_part2 = data_window
        .clone()
        .zip(data_window.skip(1))
        .filter(|(x, y)| (x.iter().sum::<usize>()) < (y.iter().sum()))
        .count();
    (increases_part1, increases_part2)
}

#[test]
fn compute_example() {
    let input_path = "input/test_input.txt";
    let (res_p1, res_p2) = compute(input_path);
    assert_eq!(res_p1, 7);
    assert_eq!(res_p2, 5);
}

#[test]
fn compute_real_result() {
    let input_path = "input/input.txt";
    let (res_p1, res_p2) = compute(input_path);
    assert_eq!(res_p1, 1288);
    assert_eq!(res_p2, 1311);
}
