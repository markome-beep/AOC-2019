use std::{char, collections::HashSet, hash::Hash};

#[derive(Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
    d: i32,
}

impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos {}

fn parse_wire(input: &str) -> HashSet<Pos> {
    let moves: Vec<(char, i32)> = input
        .trim()
        .split(',')
        .map(|mov| {
            (
                mov.chars().next().expect("ERROR"),
                mov[1..].parse::<i32>().expect("ERROR"),
            )
        })
        .collect();

    let mut positions = HashSet::new();
    let mut pos = Pos { x: 0, y: 0, d: 0 };
    for m in moves {
        let dir = match m.0 {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..m.1 {
            pos = Pos {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
                d: pos.d + 1,
            };
            positions.insert(pos);
        }
    }
    positions
}

fn day03a(input: &str) -> i32 {
    let wires = input.trim().split_once('\n').expect("ERROR HERE");
    let w1 = parse_wire(wires.0);
    let w2 = parse_wire(wires.1);
    let mut crosses: HashSet<Pos> = HashSet::new();

    for p in w1 {
        if w2.contains(&p) {
            crosses.insert(p);
        }
    }

    crosses
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .expect("ERROR 3")
}

fn day03b(input: &str) -> i32 {
    let wires = input.trim().split_once('\n').expect("ERROR HERE");
    let w1 = parse_wire(wires.0);
    let w2 = parse_wire(wires.1);

    w1.iter().filter_map(|p1| {
        w2.get(p1).map(|p2| (p1, p2))
    }
    ).map(|(p1, p2)| p1.d + p2.d).min().expect("")
}

fn main() {
    println!("{}", day03a(include_str!("../../input/day03.input")));
    println!("{}", day03b(include_str!("../../input/day03.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let r = day03b(input);
        assert_eq!(r, 610);
    }
}
