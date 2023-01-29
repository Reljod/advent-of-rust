pub mod day_1 {
    use std::fs;

    pub fn run() {
        let content =
            fs::read_to_string("./input/day1.txt").expect("Something went wrong reading the file");
        let total_calories_per_elf: Vec<i32> = get_total_calories_per_elf(&content);
        println!(
            "DAY1 (first question): The elf with the most calories is carrying: {}",
            get_most_calories(&total_calories_per_elf)
        );

        println!(
            "DAY1 (second question): The top 3 elves with the most calories are carrying: {}",
            get_top3_most_calories(&total_calories_per_elf)
        );
    }

    fn get_total_calories_per_elf(content: &String) -> Vec<i32> {
        let mut total_calories_per_elf: Vec<i32> = Vec::new();
        for line in content.split("\n\n") {
            let mut total_calories: i32 = 0;
            for calories in line.lines() {
                total_calories += calories.parse::<i32>().unwrap();
            }
            total_calories_per_elf.push(total_calories);
        }
        total_calories_per_elf
    }

    fn get_most_calories(total_calories_per_elf: &Vec<i32>) -> i32 {
        *total_calories_per_elf.iter().max().unwrap()
    }

    fn get_top3_most_calories(total_calories_per_elf: &Vec<i32>) -> i32 {
        let mut sorted = total_calories_per_elf.clone();
        sorted.sort();
        sorted[sorted.len() - 3..].iter().sum()
    }
}
