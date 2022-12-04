use crate::util::get_file_content;

fn get_calorie_sum() -> Vec<usize> {
    let mut calories: Vec<usize> = Vec::new();
    let mut sum = 0;
    let content = get_file_content("./input/1.txt");

    content.lines().for_each(|e| {
        if !e.is_empty() {
            sum += e.parse::<usize>().unwrap();
        } else {
            calories.push(sum);
            sum = 0;
        }
    });

    calories
}

pub fn part1() {
    println!(
        "Highest calorie is {}",
        get_calorie_sum().iter().max().unwrap()
    );
}

pub fn part2() {
    let mut calorie_sum = get_calorie_sum();
    calorie_sum.sort();

    let answer: usize = calorie_sum.iter().rev().take(3).sum();
    println!("Calorie of three elves {}", answer);
}
