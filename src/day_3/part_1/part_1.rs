fn resolve(text: &str) -> u64 {
    let lines = text.lines().map(|line| line.trim());
    let line_width = lines.clone().next().unwrap().len();

    let lines_and_ranges = lines
        .clone()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let num_indexes: Vec<_> = line
                .char_indices()
                .filter(|(_, c)| c.is_numeric())
                .map(|(i, _)| i)
                .collect();

            if num_indexes.is_empty() {
                None
            } else {
                let mut ranges = Vec::new();
                let mut current_range = num_indexes[0]..=num_indexes[0];
                for &index in &num_indexes[1..] {
                    if index == *current_range.end() + 1 {
                        current_range = *current_range.start()..=index;
                    } else {
                        ranges.push(current_range);
                        current_range = index..=index;
                    }
                }
                ranges.push(current_range);
                Some((line_idx, ranges))
            }
        })
        .collect::<Vec<_>>();
    let result: u64 = lines_and_ranges
        .iter()
        .map(|(line_idx, ranges)| {
            (
                line_idx,
                ranges
                    .iter()
                    .filter(|range| {
                        let y_lines = line_idx.checked_sub(1).unwrap_or(0)
                            ..=(line_idx + 1).min(line_width - 1);
                        let x_range = {
                            let start = range.start().checked_sub(1).unwrap_or(0);
                            let end = (range.end() + 1).min(line_width - 1);
                            start..=end
                        };
                        lines
                            .clone()
                            .enumerate()
                            .filter(|(i, _)| y_lines.contains(i))
                            .any(|(_, line)| {
                                line[x_range.clone()]
                                    .chars()
                                    .any(|c| c != '.' && !c.is_alphanumeric())
                            })
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(_, ranges)| !ranges.is_empty())
        .flat_map(|(line_idx, ranges)| {
            ranges
                .iter()
                .map(|&range| {
                    lines.clone().nth(*line_idx).unwrap()[range.clone()]
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();

    result
}

fn main() {
    let lines = include_str!("input.txt");
    let result = resolve(lines);
    println!("result : {}", result);
}

#[cfg(test)]
mod tests {
    use crate::resolve;

    #[test]
    fn resolve_given_test() {
        let lines = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#;
        assert_eq!(resolve(lines), 4361);
    }
}
