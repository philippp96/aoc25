advent_of_code::solution!(2);

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_line(line: &str) -> Vec<Range> {
    line.split(",")
        .map(|s| s.split("-")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
        )
        .map(|v| Range { start: v[0], end: v[1] })
        .collect::<Vec<Range>>()
}

fn get_invalid_ids_for_digit_count(num_digits: u32, lower_bound: Option<u64>, upperbound: Option<u64>) -> Vec<u64> {

    let mut ret = vec![];

    if num_digits % 2 != 0 || num_digits == 0 {
        return ret;
    }

    let mut result = 0;

    let half_digits = num_digits / 2;

    let lower_bound = lower_bound.unwrap_or(10u64.pow(num_digits - 1));
    let upperbound = upperbound.unwrap_or(10u64.pow(num_digits) - 1);

    let lower_bound_msh = lower_bound / 10u64.pow(half_digits);
    let lower_bound_msh_concatenated = lower_bound_msh * 10u64.pow(half_digits) + lower_bound_msh;

    let upper_bound_msh = upperbound / 10u64.pow(half_digits);
    let upper_bound_msh_concatenated = upper_bound_msh * 10u64.pow(half_digits) + upper_bound_msh;

    if lower_bound_msh_concatenated >= lower_bound && lower_bound_msh_concatenated <= upperbound {
        ret.push(lower_bound_msh_concatenated);
    }


    for val in (lower_bound_msh+1)..upper_bound_msh {
        ret.push(val * 10u64.pow(half_digits) + val);
    }

    if upper_bound_msh != lower_bound_msh && upper_bound_msh_concatenated <= upperbound && upper_bound_msh_concatenated >= lower_bound {
        ret.push(upper_bound_msh_concatenated);
    }

    ret

}

fn get_invalid_ids(range: &Range) -> Vec<u64> {
    let mut num_digits_start = (range.start as f64).log10().ceil() as u32;

    if range.start == 10u64.pow(num_digits_start) {
        num_digits_start += 1;
    }

    let mut num_digits_end = (range.end as f64).log10().ceil() as u32;

    if range.start == 10u64.pow(num_digits_end) {
        num_digits_end += 1;
    }

    
    let mut ret = vec![];

    for n_digits in (num_digits_start..=num_digits_end) {
        let lower_bound = if n_digits == num_digits_start { Some(range.start) } else { None };
        let upper_bound = if n_digits == num_digits_end { Some(range.end) } else { None };

        let invalid_ids = get_invalid_ids_for_digit_count(
            n_digits, lower_bound, upper_bound
        );
        
        ret.extend(invalid_ids);
    }

    println!("{:?} -> {:?}", range, ret);
    ret
}

pub fn part_one(input: &str) -> Option<u64> {
    let line = input.lines().next().unwrap();
    let ranges = parse_line(line);

    Some(
        ranges.iter()
            .map(|r|
                get_invalid_ids(r).iter().sum::<u64>()
            )
            .sum::<u64>()
    )

}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_invalid_ids_for_digit_count() {
        let result = get_invalid_ids_for_digit_count(2, Some(10), Some(99));
        assert_eq!(result, vec![11,22,33,44,55,66,77,88,99]);

        let result = get_invalid_ids_for_digit_count(4, Some(1000), Some(2010));
        assert_eq!(result, vec![1010,1111,1212,1313,1414,1515,1616,1717,1818,1919]);

        let result = get_invalid_ids_for_digit_count(2, Some(11), Some(22));
        assert_eq!(result, vec![11, 22]);

    }

    #[test]
    fn test_get_invalid_ids() {
        let range = Range { start: 10, end: 99 };
        let result = get_invalid_ids(&range);
        assert_eq!(result, vec![11,22,33,44,55,66,77,88,99]);

        let range = Range { start: 95, end: 205 };
        let result = get_invalid_ids(&range);
        assert_eq!(result, vec![99]);

    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
