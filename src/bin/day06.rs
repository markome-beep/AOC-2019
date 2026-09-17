use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Dep {
    Center(String),
    Count(i32),
}

type Orbits = HashMap<String, Dep>;

fn count(orbits: &mut Orbits, target: &str) -> i32 {
    let t = orbits.get(target).unwrap().clone();

    match t {
        Dep::Center(c) => {
            let new = 1 + count(orbits, &c);
            orbits.insert(target.to_owned(), Dep::Count(new));
            new
        }
        Dep::Count(c) => c,
    }
}

fn day06a(input: &str) -> i32 {
    let mut orbits: Orbits = HashMap::new();

    input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .for_each(|(c, o)| {
            orbits.insert(o.to_owned(), Dep::Center(c.to_owned()));
        });

    orbits.insert(String::from("COM"), Dep::Count(0));

    orbits.clone().keys().map(|t| count(&mut orbits, t)).sum()
}

fn count_b(orbits: &mut Orbits, target: &str) -> i32 {
    let t = orbits.get(target).unwrap().clone();

    match t {
        Dep::Center(c) => {
            let new = 1 + count_b(orbits, &c);
            orbits.insert(target.to_owned(), Dep::Count(new));
            new
        }
        Dep::Count(c) => -c,
    }
}

fn day06b(input: &str) -> i32 {
    let mut orbits: Orbits = HashMap::new();

    input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .for_each(|(c, o)| {
            orbits.insert(o.to_owned(), Dep::Center(c.to_owned()));
        });

    orbits.insert(String::from("COM"), Dep::Count(0));
    count_b(&mut orbits, "YOU") + count_b(&mut orbits, "SAN") - 2
}

fn main() {
    println!("{}", day06a(include_str!("../../input/day06.input")));
    println!("{}", day06b(include_str!("../../input/day06.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let r = day06b(input);
        assert_eq!(r, 4);
    }
}
