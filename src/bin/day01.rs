fn day01a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(|num| num / 3 - 2)
        .sum()
}

fn day01b(input: &str) -> i32 {
    let mut modules: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut sum = 0;
    while !modules.is_empty() {
        let Some(payload) = modules.pop() else { break };
        let fuel = payload / 3 - 2;
        if fuel > 0 {
            sum += fuel;
            modules.push(fuel);
        }
    }
    sum
}

fn main() {
    println!("{}", day01a(include_str!("../../input/day01.input")));
    println!("{}", day01b(include_str!("../../input/day01.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day01a(input);
        assert_eq!(r, 0);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day01b(input);
        assert_eq!(r, 0);
    }
}
