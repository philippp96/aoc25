advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Map {
    map: Vec<Vec<char>>,
}

// methods of Map can be added here
impl Map {
    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for dx in [-1, 0, 1].into_iter() {
            for dy in [-1, 0, 1].into_iter() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = pos.x as isize + dx;
                let new_y = pos.y as isize + dy;

                if new_x >= 0
                    && new_x < self.width() as isize
                    && new_y >= 0
                    && new_y < self.height() as isize
                {
                    neighbors.push(Position {
                        x: new_x as usize,
                        y: new_y as usize,
                    });
                }
            }
        }

        neighbors
    }
}

fn parse(input: &str) -> Map {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Map { map }
}

fn get_removable(map: &Map) -> Vec<Position> {
    let mut ret = vec![];

    for x in 0..map.width() {
        for y in 0..map.height() {
            let cell = map.map[y][x];

            if cell != '@' {
                continue;
            }

            let paper_neighbors = map.get_neighbors(Position { x, y })
                .iter()
                .filter(|neighbor| {
                    map.map[neighbor.y][neighbor.x] == '@'
                })
                .count();

            if paper_neighbors < 4 {
                ret.push(Position{ x, y });
            }
        }
    }

    ret
}

fn remove(map: &mut Map, positions: &[Position]) {
    for pos in positions {
        map.map[pos.y][pos.x] = '.';
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input);
    let removable_papers = get_removable(&map);
    Some(removable_papers.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    
    let mut map = parse(input);
    let mut removable_papers = get_removable(&map);
    let mut ret = 0;

    while removable_papers.len() > 0 {
        remove(&mut map, &removable_papers);
        ret = ret + removable_papers.len();
        removable_papers = get_removable(&map);
    }

    Some(ret as u64)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
