fn main() {
    let lines = include_str!("input.txt").lines();
    let sum = lines
        .into_iter()
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
    println!("{sum}");
}
