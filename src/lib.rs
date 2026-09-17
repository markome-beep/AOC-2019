pub mod intcode {
    type Token = i32;

    fn tokenize(input: &str) -> Vec<Token> {
        input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }

    #[derive(Debug)]
    enum ParamMode {
        Immediate,
        Position,
    }

    impl From<i32> for ParamMode {
        fn from(value: i32) -> Self {
            match value {
                1 => Self::Immediate,
                0 => Self::Position,
                u => panic!("Unkown Param Mode: {}", u),
            }
        }
    }

    #[derive(Debug)]
    struct Param {
        val: i32,
        mode: ParamMode,
    }

    impl Param {
        pub fn eval(&self, prog: &[Token]) -> i32 {
            match self.mode {
                ParamMode::Immediate => self.val,
                ParamMode::Position => prog[self.val as usize],
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
        Unknown(i32),
    }

    pub struct VM {
        toks: Vec<Token>,
        input: Vec<i32>,
        output: Vec<i32>,
        int_ptr: usize,
    }

    impl Op {
        pub fn exec(&self, prog: &mut VM) {
            match self {
                Op::Add(a, b, o) => {
                    prog.toks[o.val as usize] = a.eval(&prog.toks) + b.eval(&prog.toks);
                    prog.int_ptr += 4;
                }
                Op::Mul(a, b, o) => {
                    prog.toks[o.val as usize] = a.eval(&prog.toks) * b.eval(&prog.toks);
                    prog.int_ptr += 4;
                }
                Op::In(o) => {
                    prog.toks[o.val as usize] = prog.input.pop().unwrap();
                    prog.int_ptr += 2;
                }
                Op::Out(o) => {
                    prog.output.push(o.eval(&prog.toks));
                    prog.int_ptr += 2;
                }
                Op::JifT(a, b) => {
                    if a.eval(&prog.toks) != 0 {
                        prog.int_ptr = b.eval(&prog.toks) as usize;
                    } else {
                        prog.int_ptr += 3;
                    }
                }
                Op::JifF(a, b) => {
                    if a.eval(&prog.toks) == 0 {
                        prog.int_ptr = b.eval(&prog.toks) as usize;
                    } else {
                        prog.int_ptr += 3;
                    }
                }
                Op::Lt(a, b, o) => {
                    prog.toks[o.val as usize] = if a.eval(&prog.toks) < b.eval(&prog.toks) {
                        1
                    } else {
                        0
                    };
                    prog.int_ptr += 4;
                }
                Op::Eq(a, b, o) => {
                    prog.toks[o.val as usize] = if a.eval(&prog.toks) == b.eval(&prog.toks) {
                        1
                    } else {
                        0
                    };
                    prog.int_ptr += 4;
                }
                Op::Exit => {}
                Op::Unknown(c) => panic!("Unkown Code: {}", c),
            }
        }

        pub fn parse(toks: &[Token]) -> Self {
            let op_code = toks[0] % 100;
            let param_modes = toks[0] / 100;
            match op_code {
                1 => Op::Add(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[3],
                        mode: 0.into(),
                    },
                ),
                2 => Op::Mul(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[3],
                        mode: 0.into(),
                    },
                ),
                3 => Op::In(Param {
                    val: toks[1],
                    mode: (param_modes % 10).into(),
                }),
                4 => Op::Out(Param {
                    val: toks[1],
                    mode: (param_modes % 10).into(),
                }),
                5 => Op::JifT(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                ),
                6 => Op::JifF(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                ),
                7 => Op::Lt(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[3],
                        mode: 0.into(),
                    },
                ),
                8 => Op::Eq(
                    Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    },
                    Param {
                        val: toks[2],
                        mode: (param_modes / 10 % 10).into(),
                    },
                    Param {
                        val: toks[3],
                        mode: 0.into(),
                    },
                ),
                99 => Op::Exit,
                _ => Op::Unknown(op_code),
            }
        }
    }

    impl VM {
        pub fn run(&mut self) -> i32 {
            loop {
                let op = Op::parse(&self.toks[self.int_ptr..]);
                if matches!(op, Op::Exit) {
                    break;
                }
                // #[cfg(test)]
                // {
                    dbg!(&op);
                // }
                op.exec(self);
            }
            *self.output.last().unwrap()
        }

        pub fn new(s: &str, input: Vec<i32>) -> Self {
            VM {
                toks: tokenize(s),
                input,
                output: vec![],
                int_ptr: 0,
            }
        }
    }
}
