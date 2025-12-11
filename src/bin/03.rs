use std::f32::consts::E;

advent_of_code::solution!(3);

fn parse_line(line: &str) -> Vec<u8> {
    line
        .chars()
        .map(
            |c| c.to_digit(10).unwrap() as u8
        ).collect()
}

fn get_max_joltage_idx(jotages: &[u8], start: Option<usize>, end: Option<usize>) -> usize {
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(jotages.len());

    let &max_joltage = jotages[start..end]
        .iter()
        .max()
        .unwrap();

    jotages
        .iter()
        .enumerate()
        .skip(start)
        .take(end-start)
        .find_map(|(i, &j)| if j == max_joltage { Some(i) } else { None })
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    
    let mut sum = 0u64;

    for line in input.lines() {
        let joltages = parse_line(line);
        let first_battery_idx = get_max_joltage_idx(&joltages, None, Some(joltages.len()-1));
        let snd_battery_idx = get_max_joltage_idx(&joltages, Some(first_battery_idx+1), None);
        
        let first_battery = joltages[first_battery_idx] as u64;
        let snd_battery = joltages[snd_battery_idx] as u64;

        sum += first_battery * 10 + snd_battery;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    for line in input.lines() {
        let joltages = parse_line(line);
        let mut start_idx = 0;
        let mut line_result = 0u64;

        for i in 0..12 {
            let end_idx = Some( joltages.len() - 12 + i + 1 );
            let bat_idx = get_max_joltage_idx(&joltages, Some(start_idx), end_idx);
            line_result += joltages[bat_idx] as u64 * 10u64.pow(11 - i as u32);
            start_idx = bat_idx + 1;
        }

        sum += line_result;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_joltage_idx() {
        let joltages = vec![1, 3, 5, 2, 4];
        assert_eq!(get_max_joltage_idx(&joltages, None, None), 2);
        assert_eq!(get_max_joltage_idx(&joltages, Some(0), Some(3)), 2);
        assert_eq!(get_max_joltage_idx(&joltages, Some(3), None), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
