use aoc_2019::intcode::Program;

fn day05a(input: &str) -> i32 {
    Program::new(input, 1).run()
}

fn day05b(input: &str) -> i32 {
    Program::new(input, 5).run()
}

fn main() {
    println!("{}", day05a(include_str!("../../input/day05.input")));
    println!("{}", day05b(include_str!("../../input/day05.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day05a(input);
        assert_eq!(r, 0);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day05b(input);
        assert_eq!(r, 0);
    }
}
