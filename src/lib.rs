pub mod intcode {
    use std::{
        collections::BTreeMap,
        thread::{self, JoinHandle},
    };

    use flume::{Receiver, RecvError, Sender};
    type Token = i64;

    fn tokenize(input: &str) -> BTreeMap<usize, Token> {
        input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .enumerate()
            .collect()
    }

    #[derive(Debug)]
    enum ParamMode {
        Immediate,
        Position,
        Relative,
    }

    impl From<i64> for ParamMode {
        fn from(value: i64) -> Self {
            match value {
                2 => Self::Relative,
                1 => Self::Immediate,
                0 => Self::Position,
                u => panic!("Unkown Param Mode: {}", u),
            }
        }
    }

    #[derive(Debug)]
    struct Param {
        val: i64,
        mode: ParamMode,
    }

    impl Param {
        pub fn eval(&self, prog: &VM) -> i64 {
            match self.mode {
                ParamMode::Immediate => self.val,
                ParamMode::Position => prog.toks.get(&(self.val as usize)).copied().unwrap_or(0),
                ParamMode::Relative => prog
                    .toks
                    .get(&((self.val + prog.relative_base) as usize))
                    .copied()
                    .unwrap_or(0),
            }
        }
    }

    #[derive(Debug)]
    enum Op {
        Exit,
        Add(Param, Param, Param),
        Mul(Param, Param, Param),
        In(Param),
        Out(Param),
        JifT(Param, Param),
        JifF(Param, Param),
        Lt(Param, Param, Param),
        Eq(Param, Param, Param),
        RelBaseOff(Param),
        Unknown(i64),
    }

    pub struct VM {
        toks: BTreeMap<usize, Token>,
        input: Receiver<i64>,
        output: Sender<i64>,
        relative_base: i64,
        int_ptr: usize,
        final_val: i64,
    }

    impl Op {
        pub fn exec(&self, prog: &mut VM) -> Result<(), RecvError> {
            match self {
                Op::Add(a, b, o) => {
                    prog.toks
                        .insert(o.val as usize, a.eval(prog) + b.eval(prog));
                    prog.int_ptr += 4;
                }
                Op::Mul(a, b, o) => {
                    prog.toks
                        .insert(o.val as usize, a.eval(prog) * b.eval(prog));
                    prog.int_ptr += 4;
                }
                Op::In(o) => {
                    prog.toks.insert(o.val as usize, prog.input.recv()?);
                    prog.int_ptr += 2;
                }
                Op::Out(o) => {
                    let v = o.eval(prog);
                    prog.output.send(v).unwrap();
                    prog.final_val = v;
                    prog.int_ptr += 2;
                }
                Op::JifT(a, b) => {
                    if a.eval(prog) != 0 {
                        prog.int_ptr = b.eval(prog) as usize;
                    } else {
                        prog.int_ptr += 3;
                    }
                }
                Op::JifF(a, b) => {
                    if a.eval(prog) == 0 {
                        prog.int_ptr = b.eval(prog) as usize;
                    } else {
                        prog.int_ptr += 3;
                    }
                }
                Op::Lt(a, b, o) => {
                    prog.toks.insert(
                        o.val as usize,
                        if a.eval(prog) < b.eval(prog) { 1 } else { 0 },
                    );
                    prog.int_ptr += 4;
                }
                Op::Eq(a, b, o) => {
                    prog.toks.insert(
                        o.val as usize,
                        if a.eval(prog) == b.eval(prog) { 1 } else { 0 },
                    );
                    prog.int_ptr += 4;
                }
                Op::RelBaseOff(o) => {
                    prog.relative_base += o.eval(prog)
                }
                Op::Exit => {}
                Op::Unknown(c) => panic!("Unkown Code: {}", c),
            };
            Ok(())
        }

        pub fn parse(toks: &BTreeMap<usize, Token>, offset: usize) -> Self {
            let op_code = toks[&(offset)] % 100;
            let param_modes = toks[&(offset)] / 100;
            match op_code {
                1 => Op::Add(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[&(3 + offset)],
                        mode: 0.into(),
                    },
                ),
                2 => Op::Mul(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[&(3 + offset)],
                        mode: 0.into(),
                    },
                ),
                3 => Op::In(Param {
                    val: toks[&(1 + offset)],
                    mode: (param_modes % 10).into(),
                }),
                4 => Op::Out(Param {
                    val: toks[&(1 + offset)],
                    mode: (param_modes % 10).into(),
                }),
                5 => Op::JifT(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                ),
                6 => Op::JifF(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                ),
                7 => Op::Lt(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[&(3 + offset)],
                        mode: 0.into(),
                    },
                ),
                8 => Op::Eq(
                    Param {
                        val: toks[&(1 + offset)],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[&(2 + offset)],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[&(3 + offset)],
                        mode: 0.into(),
                    },
                ),
                9 => Op::RelBaseOff(Param {
                    val: toks[&(1 + offset)],
                    mode: (param_modes % 10).into(),
                }),
                99 => Op::Exit,
                _ => Op::Unknown(op_code),
            }
        }
    }

    impl VM {
        pub fn run(&mut self) -> Option<i64> {
            loop {
                let op = Op::parse(&self.toks, self.int_ptr);
                if matches!(op, Op::Exit) {
                    return Some(self.final_val);
                }
                op.exec(self).ok()?;
            }
        }

        pub fn start(mut self) -> JoinHandle<Option<i64>> {
            thread::spawn(move || self.run())
        }

        pub fn new(s: &str, input: Receiver<i64>, output: Sender<i64>) -> Self {
            VM {
                toks: tokenize(s),
                input,
                output,
                relative_base: 0,
                int_ptr: 0,
                final_val: 0,
            }
        }

        pub fn simple(s: &str) -> (Sender<i64>, Receiver<i64>) {
            let (input_tx, input_rx) = flume::unbounded();
            let (output_tx, output_rx) = flume::unbounded();
            VM::new(s, input_rx, output_tx).start();
            (input_tx, output_rx)
        }
    }
}
