use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let value = parts[0].parse::<u8>().unwrap();
        let color = parts[1];

        match color {
            "red" => Ok(Color::Red(value)),
            "green" => Ok(Color::Green(value)),
            "blue" => Ok(Color::Blue(value)),
            _ => Err(()),
        }
    }
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let compatible = lines.into_iter().filter(|line| {
        let games = line
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split(';')
            .collect::<Vec<&str>>();

        games.iter().all(|game| {
            let colors = game
                .split(", ")
                .map(|v| v.parse::<Color>().unwrap())
                .collect::<Vec<Color>>();

            !colors.iter().any(|color| match color {
                Color::Red(val) => *val > 12,
                Color::Green(val) => *val > 13,
                Color::Blue(val) => *val > 14,
            })
        })
    });
    let result: u32 = compatible
        .map(|line| {
            let game_part = line.split(':').next().unwrap();
            let game_id = game_part
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            game_id
        })
        .sum();
    println!("{}", result);
}
