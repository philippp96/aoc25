use std::collections::btree_map::Values;

advent_of_code::solution!(1);

fn parse_line(line: &str) -> i64 {
    let mut char_iter = line.chars();

    let direction = char_iter.next().unwrap();
    let value = char_iter.as_str().parse::<i64>().unwrap();

    return match direction {
        'L' => -value,
        'R' => value,
        _ => panic!("Invalid direction"),
    }

}

pub fn part_one(input: &str) -> Option<u64> {
    let mut initval = 50i64;
    let mut resutl = 0;

    for line in input.lines() {
        let value = parse_line(line);

        initval += value;

        if initval % 100 == 0 {
            resutl += 1;
        }

    }
    Some(resutl)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut initval = 50i64;
    let mut resutl = 0u64;

    for line in input.lines() {
        let value = parse_line(line);
        
        let prevval = initval;
        initval += value;

        resutl += value.unsigned_abs() / 100;

        if value % 100 != 0 {
            if prevval % 100 != 0 && (prevval / 100 != initval / 100 ||  prevval.signum() != initval.signum()) {
                resutl += 1;
            }
        }

    }
    Some(resutl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
