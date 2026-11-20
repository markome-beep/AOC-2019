use std::collections::HashMap;

use aoc_2019::intcode::VM;
use itertools::Itertools;

fn day11a(input: &str) -> i64 {
    let (tx, rx) = VM::simple(input);

    let mut panels: HashMap<(i32, i32), i64> = HashMap::new();
    let (mut pos, mut dir) = ((0, 0), (0, -1));

    loop {
        let _ = tx.send(*panels.get(&pos).unwrap_or(&0));

        let Ok(color) = rx.recv() else { break };
        let turn = rx.recv().unwrap();

        panels.insert(pos, color);
        dir = if turn == 0 {
            (dir.1, -dir.0)
        } else {
            (-dir.1, dir.0)
        };
        pos = (pos.0 + dir.0, pos.1 + dir.1)
    }

    panels.len() as i64
}

fn day11b(input: &str) -> String {
    let (tx, rx) = VM::simple(input);

    let mut panels: HashMap<(i32, i32), i64> = HashMap::new();
    let (mut pos, mut dir) = ((0, 0), (0, -1));

    loop {
        let _ = tx.send(*panels.get(&pos).unwrap_or(&1));

        let Ok(color) = rx.recv() else { break };
        let turn = rx.recv().unwrap();

        panels.insert(pos, color);
        dir = if turn == 0 {
            (dir.1, -dir.0)
        } else {
            (-dir.1, dir.0)
        };
        pos = (pos.0 + dir.0, pos.1 + dir.1)
    }

    let white: Vec<_> = panels
        .iter()
        .filter_map(|(&p, &c)| if c == 1 { Some(p) } else { None })
        .collect();

    let (x_min, x_max) = white
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (y_min, y_max) = white
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    (y_min..=y_max).into_iter().map(|y| {
        (x_min..=x_max).map(|x| match panels.get(&(x, y)) {
            Some(&1) => '█',
            _ => ' ',
        }).collect::<String>()
    }).join("\n")
}
fn main() {
    println!("{}", day11a(include_str!("../../input/day11.input")));
    println!("{}", day11b(include_str!("../../input/day11.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day11a(input);
        assert_eq!(r, 0);
    }

    // #[test]
    // fn part_2() {
    //     let input = "";
    //     let r = day11b(input);
    //     assert_eq!(r, 0);
    // }
}
