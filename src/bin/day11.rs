fn day11a(_input: &str) -> i32 {
    0
}

fn day11b(_input: &str) -> i32 {
    0
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

    #[test]
    fn part_2() {
        let input = "";
        let r = day11b(input);
        assert_eq!(r, 0);
    }
}