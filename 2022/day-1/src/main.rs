use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Couldn't read input file!");
    let top1_calories = calculate_calories(input.clone(), 1);
    let top3_calories = calculate_calories(input.clone(), 3);
    println!("Calories of elf with most calories: {:?}", top1_calories);
    println!("Calories of top 3 elves with most calories: {:?}", top3_calories);
}

fn calculate_calories(input: String, top: usize) -> u32 {
    let every_elf_total = input.split("\n\n").map(|x| {
        x.split("\n")
            .map(|y| y.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    let mut every_elf_total_vec = every_elf_total.collect::<Vec<u32>>();
    every_elf_total_vec.sort();
    every_elf_total_vec
        .iter()
        .copied()
        .rev()
        .collect::<Vec<u32>>()[0..top]
        .iter()
        .sum::<u32>()
}

#[test]
fn test_calculate_calories() {
    let input = fs::read_to_string("input/test_input.txt").expect("Can't read test file!");
    let top1_calories = calculate_calories(input.clone(), 1);
    let top3_calories = calculate_calories(input.clone(), 3);

    assert_eq!(top1_calories, 24000);
    assert_eq!(top3_calories, 45000);
}
