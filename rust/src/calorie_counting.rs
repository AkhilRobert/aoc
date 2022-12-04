use crate::util::get_file_content;

pub fn part1() {
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

    println!("{}", calories.iter().max().unwrap());
}
