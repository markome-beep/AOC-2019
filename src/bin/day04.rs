fn check_num(num: &i32) -> bool {
    let mut double = false;
    let mut this = num / 10_i32.pow(5) % 10;
    for p in (0..5).rev() {
        let next = num / 10_i32.pow(p) % 10;
        if this > next {
            return false;
        }
        if this == next {
            double = true;
        }
        this = next;
    }
    double
}

fn check_num_b(num: &i32) -> bool {
    let mut this = (num / 10_i32.pow(5)) % 10;
    let mut chain = 0;
    let mut double = false;
    for p in (0..5).rev() {
        let next = (num / 10_i32.pow(p)) % 10;
        #[cfg(test)]
        {
            dbg!(num, chain, double, this, next);
        }
        if this > next {
            return false;
        }
        if this == next {
            chain += 1;
        } else {
            if chain == 1 {
                double = true;
            }
            chain = 0;
        }
        this = next;
    }

    #[cfg(test)]
    {
        dbg!(double || chain == 1);
    }
    double || chain == 1
}

fn day04a(input: &str) -> usize {
    let range: (i32, i32) = input
        .trim()
        .split_once('-')
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .unwrap();

    (range.0..=range.1).filter(check_num).count()
}

fn day04b(input: &str) -> usize {
    let range: (i32, i32) = input
        .trim()
        .split_once('-')
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .unwrap();

    (range.0..=range.1).filter(check_num_b).count()
}

fn main() {
    println!("{}", day04a(include_str!("../../input/day04.input")));
    println!("{}", day04b(include_str!("../../input/day04.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = 111111;
        let r = check_num(&input);
        assert!(r);
    }

    #[test]
    fn part_1b() {
        let input = 223450;
        let r = check_num(&input);
        assert!(!r);
    }

    #[test]
    fn part_2() {
        let input = 112233;
        let r = check_num_b(&input);
        assert!(r);
    }

    #[test]
    fn part_2b() {
        let input = 123444;
        let r = check_num_b(&input);
        assert!(!r);
    }

    #[test]
    fn part_2c() {
        let input = 111122;
        let r = check_num_b(&input);
        assert!(r);
    }
}
