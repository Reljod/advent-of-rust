pub mod day_4 {
    use std::fs;

    pub fn run() {
        first_part::run();
        second_part::run();
    }

    fn split_section_assignment(line: &str) -> (i32, i32, i32, i32) {
        let mut split = line.split(',');
        let before_comma = split.next().unwrap();
        let after_comma = split.next().unwrap();

        let mut first_split = before_comma.split('-');
        let first_start = first_split.next().unwrap().parse::<i32>().unwrap();
        let first_end = first_split.next().unwrap().parse::<i32>().unwrap();

        let mut second_split = after_comma.split('-');
        let second_start = second_split.next().unwrap().parse::<i32>().unwrap();
        let second_end = second_split.next().unwrap().parse::<i32>().unwrap();

        (first_start, first_end, second_start, second_end)
    }

    fn get_section_assignments() -> Vec<(i32, i32, i32, i32)> {
        let raw = fs::read_to_string("input/day4.txt").unwrap();
        raw.lines()
            .map(|line| split_section_assignment(line))
            .collect()
    }

    mod first_part {
        use super::*;

        pub fn run() {
            let section_assignments = get_section_assignments();
            let count = count_fully_contain(&section_assignments);
            println!("DAY4 (first question): Count of section assignments with one pair fully contains another pair is {}", count);
        }

        fn check_fully_contain(section: (i32, i32, i32, i32)) -> bool {
            let (first_start, first_end, second_start, second_end) = section;
            let is_fully_contained: bool = (first_start <= second_start && first_end >= second_end)
                || (second_start <= first_start && second_end >= first_end);
            is_fully_contained
        }

        fn count_fully_contain(section_assignments: &Vec<(i32, i32, i32, i32)>) -> i32 {
            let mut count = 0;
            for section in section_assignments {
                if check_fully_contain(*section) {
                    count += 1;
                }
            }
            count
        }
    }

    mod second_part {
        use super::*;

        pub fn run() {
            let section_assignments = get_section_assignments();
            let count = count_overlap(&section_assignments);
            println!("DAY4 (second question): Count of section assignments with one pair overlaps another pair is {}", count);
        }

        fn check_overlap(section: (i32, i32, i32, i32)) -> bool {
            let (first_start, first_end, second_start, second_end) = section;
            first_end >= second_start && second_end >= first_start
        }

        fn count_overlap(section_assignments: &Vec<(i32, i32, i32, i32)>) -> i32 {
            let mut count = 0;
            for section in section_assignments {
                if check_overlap(*section) {
                    count += 1;
                }
            }
            count
        }
    }
}
