advent_of_code::solution!(6);

fn parse_line(line: &str) -> Vec<i64> {
    line
        .split_ascii_whitespace()
        .map(|s| s.parse().expect(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (acc_sum, acc_prod, ops) = input
        .lines()
        .fold(
            (vec![], vec![], vec![]), 
            |(mut acc_sum, mut acc_prod, mut ops), line| {
                

                if line.starts_with("*") || line.starts_with("+") {
                    ops = line.split_ascii_whitespace().collect::<Vec<&str>>();
                } else {
                    let numbers = parse_line(line);
                    if acc_sum.is_empty() {
                        acc_sum = numbers.clone();
                        acc_prod = numbers;
                    } else {
                        acc_sum = acc_sum
                            .iter()
                            .zip(numbers.iter())
                            .map(|(a, b)| a + b)
                            .collect();

                        acc_prod = acc_prod
                            .iter()
                            .zip(numbers.iter())
                            .map(|(a, b)| a * b)
                            .collect();
                    }
                }

                (acc_sum, acc_prod, ops)
            }
        );

    Some(
        ops.iter()
        .enumerate()
        .map(|(i, &op)|
            if op == "*" {
                acc_prod[i] 
            } else {
                acc_sum[i]
            } as u64
        )
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
