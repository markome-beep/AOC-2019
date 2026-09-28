use itertools::Itertools;

fn day08a(input: &str) -> u32 {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chunks(25 * 6)
        .into_iter()
        .map(|chunk| {
            chunk.fold((0, 0, 0), |(a, b, c), d| match d {
                0 => (a + 1, b, c),
                1 => (a, b + 1, c),
                2 => (a, b, c + 1),
                _ => (a, b, c),
            })
        })
        .min()
        .map(|(_, b, c)| b * c)
        .unwrap()
}

fn day08b(input: &str) -> String {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chunks(25 * 6)
        .into_iter()
        .fold(vec![None; 25 * 6], |mut acc, chunk| {
            for (i, d) in chunk.enumerate() {
                if acc[i].is_some() {
                    continue;
                }
                match d {
                    0 => acc[i] = Some(0),
                    1 => acc[i] = Some(1),
                    2 => acc[i] = None,
                    _ => (),
                }
            }
            acc
        })
        .into_iter()
        .filter_map(|d| d.map(|d| d.to_string()))
        .chunks(25)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .join("\n")
        .replace("0", "█")
        .replace("1", " ")
}

fn main() {
    println!("{}", day08a(include_str!("../../input/day08.input")));
    println!("{}", day08b(include_str!("../../input/day08.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day08a(input);
        assert_eq!(r, 0);
    }
}
