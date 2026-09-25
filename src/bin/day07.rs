use aoc_2019::intcode::VM;
use itertools::Itertools;

fn day07a(input: &str) -> (i32, i32) {
    let mut max = 0;
    let mut out = 0;
    for settings in (0..=4).permutations(5) {
        let mut val = 0;
        for s in settings.iter() {
            let (tx, rx) = VM::simple(input);
            tx.send(*s).unwrap();
            tx.send(val).unwrap();
            val = rx.recv().unwrap();
        }
        if max < val {
            max = val;
            out = settings.into_iter().fold(0, |acc, e| acc * 10 + e);
        }
    }
    (out, max)
}

fn day07b(input: &str) -> (i32, i32) {
    let mut max = 0;
    let mut out = 0;
    for settings in (5..=9).permutations(5) {
        let (tx, rx) = flume::unbounded();

        let mut last_rx = rx.clone();
        let mut last_tx = tx.clone();

        let mut vms = Vec::new();
        for s in settings.iter() {
            last_tx.send(*s).unwrap();
            let (next_tx, next_rx) = flume::unbounded();
            let vm = VM::new(input, last_rx, next_tx.clone());
            vms.push(vm);
            last_rx = next_rx;
            last_tx = next_tx;
        }

        let h = vms.into_iter().map(|v| v.start()).last().unwrap();

        tx.send(0).unwrap();
        drop(tx);

        let val = h.join().unwrap().unwrap();
        dbg!("HERE");

        if max < val {
            max = val;
            out = settings.into_iter().fold(0, |acc, e| acc * 10 + e);
        }
    }
    (out, max)
}

fn main() {
    println!("{:?}", day07a(include_str!("../../input/day07.input")));
    println!("{:?}", day07b(include_str!("../../input/day07.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let t = day07a(input);
        assert_eq!(t.0, 43210);
        assert_eq!(t.1, 43210);
    }

    #[test]
    fn part_1_b() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let t = day07a(input);
        assert_eq!(t.0, 1234);
        assert_eq!(t.1, 54321);
    }

    #[test]
    fn part_1_c() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let r = day07a(input);
        assert_eq!(r.0, 10432);
        assert_eq!(r.1, 65210);
    }

    #[test]
    fn part_2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let r = day07b(input);
        assert_eq!(r.0, 98765);
        assert_eq!(r.1, 139629729);
    }
}
