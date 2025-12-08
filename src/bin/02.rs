use core::num;
use std::iter;

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

fn get_concatenated_number(part: u64, part_digits: u32, total_digits: u32) -> u64 {
    let d = total_digits / part_digits;
    (0..d)
        .map(|e| part * 10u64.pow(part_digits * e))
        .sum::<u64>()
}

fn all_digits_equal(n: u64) -> bool {
    let mut msp = n;
    let d = msp % 10;
    
    while msp > 0 {
        if msp % 10 != d {
            return false;
        }
        msp /= 10;
    }

    true
}

fn prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn get_invalid_ids_for_digit_count(num_digits: u32, lower_bound: Option<u64>, upperbound: Option<u64>, div: u32, allow_all_digits_equal: bool) -> Vec<u64> {

    let mut ret = vec![];

    if num_digits % div != 0 || num_digits == 0 {
        return ret;
    }

    let mut result = 0;

    let part_digits = num_digits / div;

    let lower_bound = lower_bound.unwrap_or(10u64.pow(num_digits - 1));
    let upperbound = upperbound.unwrap_or(10u64.pow(num_digits) - 1);

    let f = 10u64.pow(part_digits * (div-1));
    let lower_bound_msp = lower_bound / f;
    let upper_bound_msp = upperbound / f;

    let lower_bound_msp_concatenated = get_concatenated_number(lower_bound_msp, part_digits, num_digits);
    if lower_bound_msp_concatenated >= lower_bound && lower_bound_msp_concatenated <= upperbound {
        if !all_digits_equal(lower_bound_msp) || allow_all_digits_equal {
            ret.push(lower_bound_msp_concatenated);
        }
    }


    for val in (lower_bound_msp+1)..upper_bound_msp {
        if !all_digits_equal(val) || allow_all_digits_equal {
            ret.push(get_concatenated_number(val, part_digits, num_digits));
        }
    }

    let upper_bound_msp_concatenated = get_concatenated_number(upper_bound_msp, part_digits, num_digits);
    if upper_bound_msp != lower_bound_msp && upper_bound_msp_concatenated <= upperbound && upper_bound_msp_concatenated >= lower_bound {
        if !all_digits_equal(upper_bound_msp) || allow_all_digits_equal {
            ret.push(upper_bound_msp_concatenated);
        }
    }

    println!("num_digits: {}, div: {}, lower_bound: {:?}, upperbound: {:?} -> {:?}", num_digits, div, lower_bound, upperbound, ret);
    ret

}

fn get_invalid_ids(range: &Range, digits: Option<u32>) -> Vec<u64> {
    let mut num_digits_start = (range.start as f64).log10().ceil() as u32;

    if range.start == 10u64.pow(num_digits_start) {
        num_digits_start += 1;
    }

    let mut num_digits_end = (range.end as f64).log10().ceil() as u32;

    if range.end == 10u64.pow(num_digits_end) {
        num_digits_end += 1;
    }

    
    let mut ret = vec![];

    for n_digits in (num_digits_start..=num_digits_end) {
        let mut allow_single_digit = true;

        let lower_bound = if n_digits == num_digits_start { Some(range.start) } else { None };
        let upper_bound = if n_digits == num_digits_end { Some(range.end) } else { None };

        let mut digits_iter = digits.map_or(2..(n_digits+1), |d| d..(d+1));

        let invalid_ids = digits_iter
            .flat_map(|n|
                if !prime(n) {
                    vec![]
                } else {
                    let ret = get_invalid_ids_for_digit_count(
                        n_digits, 
                        lower_bound, 
                        upper_bound, 
                        n,
                        digits.map_or(allow_single_digit, |_| true)
                    );
                    if (1..=9).any(|x| ret.contains(&get_concatenated_number(x, 1, n_digits))) {
                        allow_single_digit = false;
                    }
                    ret
                }
            )
            .collect::<Vec<u64>>();
        
        ret.extend(invalid_ids);
    }

    println!("{:?} -> {:?}", range, ret);
    ret
}

fn solve(input: &str, digits_part: Option<u32>) -> Option<u64> {
    let line = input.lines().next().unwrap();
    let ranges = parse_line(line);

    Some(
        ranges.iter()
            .map(|r|
                get_invalid_ids(r, digits_part).iter().sum::<u64>()
            )
            .sum::<u64>()
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, Some(2))
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, None)
}

#[cfg(test)]
mod tests {
    use advent_of_code::template::commands::all;

    use super::*;

    #[test]
    fn test_get_invalid_ids_for_digit_count() {
    }

    #[test]
    fn test_all_digets_equal() {
        assert_eq!(all_digits_equal(2), true);
    }

    #[test]
    fn test_get_invalid_ids() {
        let range = Range { start: 10, end: 99 };
        let result = get_invalid_ids(&range, Some(2));
        assert_eq!(result, vec![11,22,33,44,55,66,77,88,99]);

        let range = Range { start: 95, end: 205 };
        let result = get_invalid_ids(&range, Some(2));
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
        assert_eq!(result, Some(4174379265));
    }
}
