use std::{collections::VecDeque, os::linux::raw::stat};

advent_of_code::solution!(7);

type StateType = Vec<bool>;

fn parse_line(line: &str) -> StateType {
    line
        .chars()
        .map(|c| match c {
            'S' | '^' => true,
            _ => false,
        })
        .collect()
}


pub fn part_one(input: &str) -> Option<u64> {

    let initial_state: StateType = input
        .lines()
        .next()
        .map(parse_line)
        .unwrap_or_default();


    Some(
        input.lines()
            .skip(1)
            .fold(
                (0u64, initial_state),
                |(num_splits, state), line| {

                    let op = parse_line(line);

                    println!("{:?}", op);

                    let split_positions = state.iter()
                        .zip(op.iter())
                        .enumerate()
                        .filter_map(|(idx, (&s, &o))| if s && o { Some(idx) } else { None })
                        .collect::<Vec<usize>>();


                    let mut next_state: Vec<bool> = state.iter()
                        .enumerate()
                        .map(|(i, &v)| if split_positions.contains(&i) {false} else {v})
                        .collect();

                    next_state = next_state.iter()
                        .enumerate()
                        .map(|(i, &v)| if i > 0 && split_positions.contains(&(i-1)) || split_positions.contains(&(i+1)) {true} else {v})
                        .collect();

                    (num_splits + split_positions.len() as u64, next_state)
                }
            ).0
        )

}

fn solve_part2(state: Vec<u64>, pos: usize, line_iter: &Vec<Vec<bool>>) -> Vec<u64> {

    if pos < line_iter.len() {

        let line = &line_iter[pos];

        let mut new_state = state.clone();
            
        // iterate over split indices
        for i_split in line
            .iter()
            .enumerate()
            .filter_map(|(i, &f)| if f {Some(i)} else {None}) {

            if state[i_split] > 0 {
                new_state[i_split] -= state[i_split];
                if i_split > 0 {
                    new_state[i_split - 1] += state[i_split];
                }
                if i_split < line.len() - 1 {
                    new_state[i_split + 1] += state[i_split];
                }
            }
        }

        return solve_part2(new_state, pos+1, line_iter);

    }

    state
}

pub fn part_two(input: &str) -> Option<u64> {

    let lines = input.lines().map(|l| parse_line(l)).collect::<Vec<Vec<bool>>>();
    let initial_state = lines[0]
        .iter()
        .map(|&f| if f {1} else {0})
        .collect::<Vec<u64>>();
    
    let final_state = solve_part2(initial_state, 1, &lines);
    Some(final_state.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
