use std::collections::HashMap;

fn main() {
    let numbers: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let lines = include_str!("input.txt").lines();
    let sum = lines
        .into_iter()
        .map(|line| {
            let mut line = line.to_string();
            if numbers.iter().any(|(&key, _)| line.contains(key)) {
                let mut end_index = 0;
                while end_index < line.len() {
                    let word = &line[..=end_index];
                    let contained_word = numbers.iter().find(|(&key, _)| word.contains(key));
                    if contained_word.is_some() {
                        let mut key = *contained_word.unwrap().0;
                        key = &key[0..key.len() - 1];
                        line = line.replacen(key, &contained_word.unwrap().1.to_string(), 1);
                        end_index = 0;
                    } else {
                        end_index += 1;
                    }
                }
            }
            line
        })
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .map(|nums| {
            format!(
                "{}{}",
                *nums.first().unwrap_or(&'0'),
                *nums.last().unwrap_or(&'0')
            )
        })
        .map(|nums| nums.parse::<u32>().unwrap_or(0))
        .sum::<u32>();
    println!("Result: {sum}");
}
