advent_of_code::solution!(5);


#[derive(Debug, Clone, Copy)]
struct IngredientIDRange {
    start: u64,
    end: u64,
}

impl IngredientIDRange {
    fn contains(&self, n: u64) -> bool {
        self.start <= n && n <= self.end
    }
}

fn parse_range(line: &str) -> IngredientIDRange {
    let tmp = line
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    IngredientIDRange { start: tmp[0], end: tmp[1] }
}

fn parse(input: &str) -> (Vec<IngredientIDRange>, Vec<u64>) {

    let ingredientIDRanges: Vec<IngredientIDRange> = input.lines()
        .take_while(|&line| !line.is_empty())
        .map(|line| parse_range(line))
        .collect();

    let ingredientsToCheck: Vec<u64> = input.lines()
        .skip(ingredientIDRanges.len() + 1)
        .map(|s| s.parse().unwrap())
        .collect();

    (ingredientIDRanges, ingredientsToCheck)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (IngredientIDRanges, ingredientsToCheck) = parse(input);

    let mut ret = 0;

    for ingredient in ingredientsToCheck {
        if IngredientIDRanges.iter().any(|range| range.contains(ingredient)) {
            ret += 1;
        }
    }

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ingredient_ranges = input
        .lines()
        .take_while(|&line| !line.is_empty())
        .map(|line| parse_range(line))
        .collect::<Vec<IngredientIDRange>>();

    ingredient_ranges.sort_by_key(|id_range| id_range.start);

    let mut result = 0u64;
    let mut current_range_opt: Option<IngredientIDRange> = None;

    for &id_range in ingredient_ranges.iter() {
        if let Some(ref mut current_range) = current_range_opt {
            if id_range.start <= current_range.end {
                // have to merge
                current_range.end = current_range.end.max(id_range.end);
            } else {
                result += current_range.end - current_range.start + 1;
                current_range_opt = Some(id_range);

            }
        } else {
            current_range_opt = Some(id_range);
        }
    }

    result += current_range_opt.unwrap().end - current_range_opt.unwrap().start + 1;

    Some(result)

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
        assert_eq!(result, Some(14));
    }
}
