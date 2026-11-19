use aoc_2019::intcode::VM;

fn day09a(input: &str) -> i64 {
    let (tx, rx) = VM::simple(input);
    tx.send(1).unwrap();
    drop(tx);
    rx.iter().last().unwrap()
}

fn day09b(_input: &str) -> i32 {
    0
}

fn main() {
    println!("{}", day09a(include_str!("../../input/day09.input")));
    println!("{}", day09b(include_str!("../../input/day09.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day09a(input);
        assert_eq!(r, 0);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day09b(input);
        assert_eq!(r, 0);
    }
}
