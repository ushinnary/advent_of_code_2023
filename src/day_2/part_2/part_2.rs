use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Color {
    pub fn get_value(&self) -> u8 {
        match self {
            Color::Red(val) => *val,
            Color::Green(val) => *val,
            Color::Blue(val) => *val,
        }
    }

    pub fn max(&self, other: &Color) -> Color {
        match (self, other) {
            (Color::Red(a), Color::Red(b))
            | (Color::Green(a), Color::Green(b))
            | (Color::Blue(a), Color::Blue(b)) => {
                if b > a {
                    *other
                } else {
                    *self
                }
            }
            _ => *self,
        }
    }
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
    let lines = include_str!("../part_1/input.txt").lines();
    let result: u64 = lines
        .map(|line| {
            let games = line
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split(';')
                .collect::<Vec<&str>>();

            let results = games.iter().fold(
                vec![Color::Red(0), Color::Green(0), Color::Blue(0)],
                |mut acc, f| {
                    let colors = f
                        .split(", ")
                        .map(|v| v.parse::<Color>().unwrap())
                        .collect::<Vec<Color>>();

                    for a in acc.iter_mut() {
                        for b in colors.iter() {
                            *a = a.max(b);
                        }
                    }

                    acc
                },
            );

            results
                .iter()
                .map(|color| color.get_value() as u64)
                .filter(|val| *val != 0)
                .product::<u64>()
        })
        .sum();
    println!("result : {}", result);
}
