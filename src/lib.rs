pub mod intcode {
    type Token = i32;

    fn tokenize(input: &str) -> Vec<Token> {
        input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    }
enum ParamMode { Immediate, Position, }

    impl From<i32> for ParamMode {
        fn from(value: i32) -> Self {
            match value {
                1 => Self::Immediate,
                0 => Self::Position,
                u => panic!("Unkown Param Mode: {}", u),
            }
        }
    }

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

    pub struct Program {
        toks: Vec<Token>,
        input: i32,
        output: Vec<i32>,
        int_ptr: usize,
    }

    impl Op {
        pub fn exec(&self, prog: &mut Program) {
            match self {
                Op::Add(a, b, o) => {
                    prog.toks[o.val as usize] = a.eval(&prog.toks) + b.eval(&prog.toks)
                }
                Op::Mul(a, b, o) => {
                    prog.toks[o.val as usize] = a.eval(&prog.toks) * b.eval(&prog.toks)
                }
                Op::In(o) => prog.toks[o.val as usize] = prog.input,
                Op::Out(o) => prog.output.push(prog.toks[o.val as usize]),
                Op::Exit => {}
                Op::Unknown(c) => panic!("Unkown Code: {}", c),

                Op::JifT(a, b) => {
                    if a.eval(&prog.toks) != 0 {
                        prog.int_ptr = b.eval(&prog.toks) as usize - 3;
                    }
                }
                Op::JifF(a, b) => {
                    if a.eval(&prog.toks) == 0 {
                        prog.int_ptr = b.eval(&prog.toks) as usize - 3;
                    }
                }
                Op::Lt(a, b, o) => {
                    prog.toks[o.val as usize] = if a.eval(&prog.toks) < b.eval(&prog.toks) {
                        1
                    } else {
                        0
                    }
                }
                Op::Eq(a, b, o) => {
                    prog.toks[o.val as usize] = if a.eval(&prog.toks) == b.eval(&prog.toks) {
                        1
                    } else {
                        0
                    }
                }
            }
        }

        pub fn parse(toks: &[Token]) -> (Self, usize) {
            let op_code = toks[0] % 100;
            let param_modes = toks[0] / 100;
            match op_code {
                1 => (
                    Op::Add(
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
                    4,
                ),
                2 => (
                    Op::Mul(
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
                    4,
                ),
                3 => (
                    Op::In(Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    }),
                    2,
                ),
                4 => (
                    Op::Out(Param {
                        val: toks[1],
                        mode: (param_modes % 10).into(),
                    }),
                    2,
                ),
                5 => (
                    Op::JifT(
                        Param {
                            val: toks[1],
                            mode: (param_modes % 10).into(),
                        },
                        Param {
                            val: toks[2],
                            mode: (param_modes / 10 % 10).into(),
                        },
                    ),
                    3,
                ),
                6 => (
                    Op::JifF(
                        Param {
                            val: toks[1],
                            mode: (param_modes % 10).into(),
                        },
                        Param {
                            val: toks[2],
                            mode: (param_modes / 10 % 10).into(),
                        },
                    ),
                    3,
                ),
                7 => (
                    Op::Lt(
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
                    4,
                ),
                8 => (
                    Op::Eq(
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
                    4,
                ),
                99 => (Op::Exit, 0),
                _ => (Op::Unknown(op_code), 0),
            }
        }
    }

    impl Program {
        pub fn run(&mut self) -> i32 {
            while self.int_ptr < self.toks.len() {
                let (op, offset) = Op::parse(&self.toks[self.int_ptr..]);
                if matches!(op, Op::Exit) {
                    break;
                }
                op.exec(self);
                self.int_ptr += offset;
            }
            *self.output.last().unwrap()
        }

        pub fn new(s: &str, input: i32) -> Self {
            Program {
                toks: tokenize(s),
                input,
                output: vec![],
                int_ptr: 0,
            }
        }
    }
}
