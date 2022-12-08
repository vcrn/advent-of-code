fn main() {
    let path = "input/input".to_string();
    let start_packet = detect_start(path.clone(), 4);
    let start_message = detect_start(path, 14);
    println!("Packet starts after {}", start_packet);
    println!("Message starts after {}", start_message);
}

fn detect_start(path: String, length: usize) -> usize {
    let input = std::fs::read_to_string(path).expect("Couldn't read file");
    let vec_of_chars = input.chars().collect::<Vec<char>>();
    let windows = vec_of_chars.windows(length);
    let mut i = length;

    for window in windows {
        let mut map = HashMap::new();
        for c in window {
            map.insert(c, "v");
        }
        if map.keys().len() == length {
            return i;
        }
        i += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_message() {
        let path = "input/input_test1".to_string();
        let start = detect_start(path, 4);

        assert_eq!(5, start);
    }

    #[test]
    fn second_message() {
        let path = "input/input_test2".to_string();
        let start = detect_start(path, 4);

        assert_eq!(6, start);
    }

    #[test]
    fn third_message() {
        let path = "input/input_test3".to_string();
        let start = detect_start(path, 4);

        assert_eq!(10, start);
    }

    #[test]
    fn fourth_message() {
        let path = "input/input_test4".to_string();
        let start = detect_start(path, 4);

        assert_eq!(11, start);
    }

    #[test]
    fn fifth_message() {
        let path = "input/input_test5".to_string();
        let start = detect_start(path, 14);

        assert_eq!(19, start);
    }

    #[test]
    fn sixth_message() {
        let path = "input/input_test6".to_string();
        let start = detect_start(path, 14);

        assert_eq!(23, start);
    }

    #[test]
    fn real_message() {
        let path = "input/input".to_string();
        let start_packet = detect_start(path.clone(), 4);
        let start_message = detect_start(path, 14);

        assert_eq!(1909, start_packet);
        assert_eq!(3380, start_message);
    }
}
