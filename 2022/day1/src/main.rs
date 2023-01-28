use std::fs;

fn main() {
    let content = fs::read_to_string("./src/input/calories.txt")
        .expect("Something went wrong reading the file");
    let calories_per_elf: Vec<i32> = get_calories_per_elf(&content);
    println!(
        "The elf with the most calories is carrying: {}",
        calories_per_elf.iter().max().unwrap()
    );
}

fn get_calories_per_elf(content: &String) -> Vec<i32> {
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
