use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::Extend;
use std::vec;
use itertools::Itertools;


advent_of_code::solution!(8);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}



impl Coord {
    fn distance_vector(&self, other: &Self) -> Self {
        Self { x: other.x-self.x, y: other.y-self.y, z: other.z - self.z }
    }

    fn straight_line_distance(&self, other: &Self) -> f64 {
        let d = self.distance_vector(other);
        ((d.x.pow(2) + d.y.pow(2) + d.z.pow(2)) as f64).sqrt()
    }

}

impl From<Vec<i64>> for Coord {
    fn from(v: Vec<i64>) -> Self {
        assert_eq!(v.len(), 3);
        Coord { x: v[0], y: v[1], z: v[2] }
    }
    
}

#[derive(Debug)]
enum CircuitOrRef {
    Circuit(Coord, HashSet<Coord>),
    Link(Coord),
}

impl CircuitOrRef {
    
    fn get_containing_circuit(&self) -> Coord {
        match self {
            CircuitOrRef::Circuit(c, hs) => *c,
            CircuitOrRef::Link(c) => *c,
        }
    }
}

fn parse_line(line: &str) -> Coord {
    let v = line.split(',').map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
    Coord::from(v)
}

pub fn part_one(input: &str) -> Option<u64> {

    let coords = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut coord_to_circuit = HashMap::new();

    for (i, &c) in coords.iter().enumerate() {
        coord_to_circuit.insert(
            c,
            CircuitOrRef::Circuit(c, HashSet::from([c])),
        );
    }

    for (c1, c2) in coords
        .iter()
        .tuple_combinations()
        .sorted_by(|(a, b), (c, d)|
            a.straight_line_distance(b).total_cmp(&c.straight_line_distance(d))
        )
        .take(1000) {
            let s1 = coord_to_circuit.get(c1).unwrap();
            let s2= coord_to_circuit.get(c2).unwrap();


            let c1s = s1.get_containing_circuit();
            let c2s = s2.get_containing_circuit();

            if c1s != c2s {
                let set1 = match &coord_to_circuit[&c1s] {
                    CircuitOrRef::Circuit(_, hs) => hs,
                    _ => panic!()
                }.clone();
                let set2 = match &coord_to_circuit[&c2s] {
                    CircuitOrRef::Circuit(_, hs) => hs,
                    _ => panic!()
                }.clone();

                for c in set2.iter() {
                    coord_to_circuit.insert(*c, CircuitOrRef::Link(c1s));
                }

                let mut new_set = set1;
                new_set.extend(set2);

                println!("{:?} <--> {:?}, new size = {:?}", c1, c2, new_set.len());
                coord_to_circuit.insert(c1s, CircuitOrRef::Circuit(c1s, new_set));

            }
    }

    Some(
        coord_to_circuit
            .iter()
            .filter_map(
                |(c, cor)| {
                    match cor {
                        CircuitOrRef::Circuit(_, set) => Some(set),
                        _ => None,
                    }
                }
            )
            .sorted_by_key(|s| -(s.len() as i64))
            .take(3)
            .map(|s| { println!("{:?}\n", s);  s.len() as u64 })
            .product()
    )
}

pub fn part_two(input: &str) -> Option<i64> {

    let coords = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut coord_to_circuit = HashMap::new();

    for (i, &c) in coords.iter().enumerate() {
        coord_to_circuit.insert(
            c,
            CircuitOrRef::Circuit(c, HashSet::from([c])),
        );
    }

    for (c1, c2) in coords
        .iter()
        .tuple_combinations()
        .sorted_by(|(a, b), (c, d)|
            a.straight_line_distance(b).total_cmp(&c.straight_line_distance(d))
        ) {
            let s1 = coord_to_circuit.get(c1).unwrap();
            let s2= coord_to_circuit.get(c2).unwrap();


            let c1s = s1.get_containing_circuit();
            let c2s = s2.get_containing_circuit();

            if c1s != c2s {
                let set1 = match &coord_to_circuit[&c1s] {
                    CircuitOrRef::Circuit(_, hs) => hs,
                    _ => panic!()
                }.clone();
                let set2 = match &coord_to_circuit[&c2s] {
                    CircuitOrRef::Circuit(_, hs) => hs,
                    _ => panic!()
                }.clone();

                for c in set2.iter() {
                    coord_to_circuit.insert(*c, CircuitOrRef::Link(c1s));
                }

                if set1.len() + set2.len() == coords.len() {
                    return Some(c1.x * c2.x);
                }

                let mut new_set = set1;
                new_set.extend(set2);

                println!("{:?} <--> {:?}, new size = {:?}", c1, c2, new_set.len());
                coord_to_circuit.insert(c1s, CircuitOrRef::Circuit(c1s, new_set));

            }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
