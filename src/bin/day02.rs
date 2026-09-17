type Token = usize;

fn tokenize(input: &str) -> Vec<Token> {
    input
        .trim()
        .split(',')
        .map(|n| {
            n.parse::<usize>()
                .unwrap_or_else(|_| panic!("Unable to parse {n}"))
        })
        .collect()
}

fn execute(tokens: &mut [Token]) -> Token {
    let mut i = 0;
    while i < tokens.len() {
        let op = tokens[i];
        match op {
            1 => {
                if let [arg1, arg2, pos] = tokens[i + 1..=i + 3] {
                    tokens[pos] = tokens[arg1] + tokens[arg2];
                } else {
                    unreachable!()
                }
            }
            2 => {
                if let [arg1, arg2, pos] = tokens[i + 1..=i + 3] {
                    tokens[pos] = tokens[arg1] * tokens[arg2];
                } else {
                    unreachable!()
                }
            }
            99 => {
                break;
            }
            _ => unreachable!(),
        }

        i += 4;
    }
    tokens[0]
}

fn day02a(input: &str) -> usize {
    let mut tokens = tokenize(input);
    tokens[1] = 12;
    tokens[2] = 2;
    execute(&mut tokens)
}

fn day02b(input: &str) -> usize {
    let og_tokens = tokenize(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut tokens = og_tokens.clone();
            tokens[1] = noun;
            tokens[2] = verb;
            if execute(&mut tokens) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", day02a(include_str!("../../input/day02.input")));
    println!("{}", day02b(include_str!("../../input/day02.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1,0,0,0,99";
        let r = day02a(input);
        assert_eq!(r, 2);
    }

    #[test]
    fn part_1b() {
        let input = "1,1,1,4,99,5,6,0,99";
        let r = day02a(input);
        assert_eq!(r, 30);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day02b(input);
        assert_eq!(r, 0);
    }
}
