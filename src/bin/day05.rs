use aoc_2019::intcode::VM;

fn day05a(input: &str) -> i32 {
    let (tx, rx) = VM::simple(input);
    tx.send(1).unwrap();
    drop(tx);
    rx.iter().last().unwrap()
}

fn day05b(input: &str) -> i32 {
    let (tx, rx) = VM::simple(input);
    tx.send(5).unwrap();
    drop(tx);
    rx.recv().unwrap()
}

fn main() {
    println!("{}", day05a(include_str!("../../input/day05.input")));
    println!("{}", day05b(include_str!("../../input/day05.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        {
            let (input_tx, input_rx) = flume::unbounded();
            let (output_tx, output_rx) = flume::unbounded();
            input_tx.send(5).unwrap();
            VM::new(input, input_rx, output_tx).run();
            let r = output_rx.recv().unwrap();
            assert_eq!(r, 999);
        }

        {
            let (input_tx, input_rx) = flume::unbounded();
            let (output_tx, output_rx) = flume::unbounded();
            input_tx.send(8).unwrap();
            VM::new(input, input_rx, output_tx).run();
            let r = output_rx.recv().unwrap();
            assert_eq!(r, 1000);
        }

        {
            let (input_tx, input_rx) = flume::unbounded();
            let (output_tx, output_rx) = flume::unbounded();
            input_tx.send(1200).unwrap();
            VM::new(input, input_rx, output_tx).run();
            let r = output_rx.recv().unwrap();
            assert_eq!(r, 1001);
        }

        {
            let (tx, rx) = VM::simple(input);
            tx.send(10).unwrap();
            assert_eq!(rx.recv().unwrap(),1001);
        }
    }

    // #[test]
    // fn simple_pos_mode() {
    //     let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    //
    //     let r = VM::new(input, vec![1]).run();
    //     assert_eq!(r, 1);
    //
    //     let r = VM::new(input, vec![8]).run();
    //     assert_eq!(r, 1);
    //
    //     let r = VM::new(input, vec![0]).run();
    //     assert_eq!(r, 0);
    // }
    //
    // #[test]
    // fn simple_immediate_mode() {
    //     let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    //
    //     let r = VM::new(input, vec![1]).run();
    //     assert_eq!(r, 1);
    //
    //     let r = VM::new(input, vec![8]).run();
    //     assert_eq!(r, 1);
    //
    //     let r = VM::new(input, vec![0]).run();
    //     assert_eq!(r, 0);
    // }
}
