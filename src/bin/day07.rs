use aoc_2019::intcode::VM;
use itertools::Itertools;

fn day07a(input: &str) -> u32 {
    let mut max = 0;
    let mut out = 0;
    for settings in (0..=4).permutations(5) {
        let mut val = 0;
        for s in settings.iter() {
            val = VM::new(input, vec![*s, val]).run();
        }
        if max < val {
            max = val;
            out = settings.into_iter().fold((0,0), |acc, e| (acc.0+1, acc.1+(e*10_i32.pow(acc.0)))).0;
        }
    }
    out
}

fn day07b(_input: &str) -> i32 {
    0
}

fn main() {
    println!("{}", day07a(include_str!("../../input/day07.input")));
    println!("{}", day07b(include_str!("../../input/day07.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        // let a_out = VM::new(input, vec![4, 0]).run();
        // let b_out = VM::new(input, vec![3, a_out]).run();
        // let c_out = VM::new(input, vec![2, b_out]).run();
        // let d_out = VM::new(input, vec![1, c_out]).run();
        // let r = VM::new(input, vec![0, d_out]).run();
        let t = day07a(input);
        assert_eq!(t, 43210);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day07b(input);
        assert_eq!(r, 0);
    }
}

