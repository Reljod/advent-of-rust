pub mod day_3 {

    use std::collections::HashSet;
    use std::fs;

    pub fn run() {
        self::first_part::run();
        self::second_part::run();
    }

    fn get_supplies() -> Vec<String> {
        let raw = fs::read_to_string("input/day3.txt").unwrap();
        raw.lines().map(|x| x.to_string()).collect()
    }

    fn convert_letter_to_points(letter: &char) -> i32 {
        match letter {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => 0,
        }
    }

    mod first_part {
        use super::*;

        pub fn run() {
            let supplies = get_supplies();
            let priorities_sum = get_priorities_sum(&supplies);

            println!(
                "DAY3 (first question): Priorities Sum is {}",
                priorities_sum
            );
        }

        fn get_half_supply(supply: &str) -> (&str, &str) {
            let half = supply.len() / 2;
            (&supply[..half], &supply[half..])
        }

        fn get_first_common_letter(first_supply: &str, second_supply: &str) -> Option<char> {
            let first_supply_set: HashSet<char> = first_supply.chars().collect();
            for letter in second_supply.chars() {
                if first_supply_set.contains(&letter) {
                    return Some(letter);
                }
            }
            return None;
        }

        fn get_priorities_sum(supplies: &Vec<String>) -> i32 {
            let mut priorities_sum = 0;
            for supply in supplies {
                let (first, second) = get_half_supply(supply);
                if let Some(common_letter) = get_first_common_letter(first, second) {
                    let common_letter_points = convert_letter_to_points(&common_letter);
                    priorities_sum += common_letter_points;
                }
            }
            priorities_sum
        }
    }

    mod second_part {
        use super::*;

        pub fn run() {
            let supplies = get_supplies();
            let priorities_sum = get_priorities_sum_by_3(&supplies);
            println!(
                "DAY3 (second question): Priorities Sum is {}",
                priorities_sum
            );
        }

        fn get_common_of_3_letter(first: &str, second: &str, third: &str) -> Option<char> {
            let first_supply_set: HashSet<char> = first.chars().collect();
            let second_supply_set: HashSet<char> = second.chars().collect();
            let third_supply_set: HashSet<char> = third.chars().collect();

            for letter in first_supply_set {
                if second_supply_set.contains(&letter) && third_supply_set.contains(&letter) {
                    return Some(letter);
                }
            }
            return None;
        }

        fn get_priorities_sum_by_3(supplies: &Vec<String>) -> i32 {
            let mut priorities_sum = 0;
            let mut i = 0;
            while i < supplies.len() {
                let common_letter =
                    get_common_of_3_letter(&supplies[i], &supplies[i + 1], &supplies[i + 2]);
                if let Some(common_letter) = common_letter {
                    let common_letter_points = convert_letter_to_points(&common_letter);
                    priorities_sum += common_letter_points;
                }
                i += 3;
            }
            priorities_sum
        }
    }
}
