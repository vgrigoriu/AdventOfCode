use color_eyre::Report;
use color_eyre::Result as EyreResult;
use color_eyre::eyre::Context;
use color_eyre::eyre::OptionExt;
use color_eyre::eyre::eyre;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = Report;

    fn from_str(s: &str) -> EyreResult<Self> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(eyre!("No such register: {s}")),
        }
    }
}

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Register::A => write!(f, "a")?,
            Register::B => write!(f, "b")?,
            Register::C => write!(f, "c")?,
            Register::D => write!(f, "d")?,
        };

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Value(i64);

impl FromStr for Value {
    type Err = Report;

    fn from_str(s: &str) -> EyreResult<Self> {
        Ok(Value(s.parse().wrap_err_with(|| {
            format!("can't parse number from {s}")
        })?))
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy)]
enum Arg {
    Register(Register),
    Value(Value),
}

impl FromStr for Arg {
    type Err = Report;

    fn from_str(s: &str) -> EyreResult<Self> {
        match s {
            "a" | "b" | "c" | "d" => Ok(Arg::Register(s.parse()?)),
            _ => Ok(Arg::Value(s.parse()?)),
        }
    }
}

impl Debug for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Arg::Register(r) => write!(f, "{:?}", r),
            Arg::Value(v) => write!(f, "{:?}", v),
        }
    }
}
enum Instruction {
    Cpy { arg: Arg, r: Register },
    Inc { r: Register },
    Dec { r: Register },
    Jnz { cmp: Arg, jump: Arg },
    Tgl { arg: Arg },
    Noop,
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match self {
            Instruction::Cpy { arg, r } => Instruction::Jnz {
                cmp: *arg,
                jump: Arg::Register(*r),
            },
            Instruction::Inc { r } => Instruction::Dec { r: *r },
            Instruction::Dec { r } => Instruction::Inc { r: *r },
            Instruction::Jnz {
                cmp,
                jump: Arg::Register(r),
            } => Instruction::Cpy { arg: *cmp, r: *r },
            Instruction::Tgl {
                arg: Arg::Register(r),
            } => Instruction::Inc { r: *r },
            // All other combinations are invalid.
            _ => Instruction::Noop,
        }
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> EyreResult<Self> {
        // jnz 1 5
        // cpy 7 c
        // inc d
        // dec c
        match &s[..3] {
            "jnz" => {
                let (cmp, jump) = &s[4..]
                    .split_once(" ")
                    .ok_or_eyre("Expected two arguments")?;
                Ok(Instruction::Jnz {
                    cmp: cmp.parse().unwrap(),
                    jump: jump.parse()?,
                })
            }
            "cpy" => {
                let (arg, r) = &s[4..]
                    .split_once(" ")
                    .ok_or_eyre("Expected two arguments")?;
                Ok(Instruction::Cpy {
                    arg: arg.parse().unwrap(),
                    r: r.parse().unwrap(),
                })
            }
            "inc" => {
                let r = &s[4..];
                Ok(Instruction::Inc { r: r.parse()? })
            }
            "dec" => {
                let r = &s[4..];
                Ok(Instruction::Dec { r: r.parse()? })
            }
            "tgl" => {
                let arg = &s[4..];
                Ok(Instruction::Tgl { arg: arg.parse()? })
            }
            _ => Err(eyre!("Unexpected instruction: {s}")),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Cpy { arg, r } => {
                write!(f, "cpy {:?} {:?}", arg, r)
            }
            Instruction::Inc { r } => {
                write!(f, "inc {:?}", r)
            }
            Instruction::Dec { r } => {
                write!(f, "dec {:?}", r)
            }
            Instruction::Jnz { cmp, jump } => {
                write!(f, "jnz {:?} {:?}", cmp, jump)
            }
            Instruction::Tgl { arg } => {
                write!(f, "tgl {:?}", arg)
            }
            Instruction::Noop => {
                write!(f, "noop")
            }
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = Report;

    fn from_str(s: &str) -> EyreResult<Self> {
        let instructions = s
            .lines()
            .map(|l| {
                l.parse::<Instruction>()
                    .wrap_err_with(|| format!("error parsing {l}"))
            })
            .collect::<EyreResult<_>>()?;

        Ok(Program { instructions })
    }
}

pub struct Computer {
    a: Value,
    b: Value,
    c: Value,
    d: Value,
    program: Program,
    ip: usize,
}

impl Computer {
    pub fn new(program: &str, a: i64, b: i64, c: i64, d: i64) -> Self {
        Self {
            a: Value(a),
            b: Value(b),
            c: Value(c),
            d: Value(d),
            program: program.parse().expect("error parsing program"),
            ip: 0,
        }
    }

    pub fn a(&self) -> i64 {
        self.a.0
    }

    pub fn is_done(&self) -> bool {
        self.program.instructions.len() <= self.ip
    }

    pub fn step(&mut self) {
        if self.is_done() {
            return;
        }

        match &self.program.instructions[self.ip] {
            Instruction::Cpy { arg, r } => {
                self.set(*r, self.value(*arg));
                self.ip += 1;
            }
            Instruction::Inc { r } => {
                self.set(*r, Value(self.value(Arg::Register(*r)).0 + 1));
                self.ip += 1;
            }
            Instruction::Dec { r } => {
                self.set(*r, Value(self.value(Arg::Register(*r)).0 - 1));
                self.ip += 1;
            }
            Instruction::Jnz { cmp, jump } => {
                if self.value(*cmp).0 != 0 {
                    let jump = self.value(*jump);
                    self.ip = (self.ip as i64 + jump.0) as usize;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Tgl { arg } => {
                let jump = self.value(*arg);
                let ip_to_change = self.ip as i64 + jump.0;
                if ip_to_change < 0 || self.program.instructions.len() as i64 <= ip_to_change {
                    // nothing happens
                    self.ip += 1;
                    return;
                }
                let ip_to_change = ip_to_change as usize;
                self.program.instructions[ip_to_change].toggle();
                self.ip += 1;
            }
            Instruction::Noop => {
                self.ip += 1;
            }
        }
    }

    fn set(&mut self, r: Register, value: Value) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
        }
    }

    fn value(&self, arg: Arg) -> Value {
        match arg {
            Arg::Value(v) => v,
            Arg::Register(Register::A) => self.a,
            Arg::Register(Register::B) => self.b,
            Arg::Register(Register::C) => self.c,
            Arg::Register(Register::D) => self.d,
        }
    }
}

impl Debug for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "      A       B       C       D")?;
        writeln!(
            f,
            "{:7} {:7} {:7} {:7}",
            self.a.0, self.b.0, self.c.0, self.d.0
        )?;
        writeln!(f)?;
        for (n, instr) in self.program.instructions.iter().enumerate() {
            if n == self.ip {
                write!(f, "> ")?;
            } else {
                write!(f, "  ")?;
            }
            writeln!(f, "{instr:?}")?;
        }
        Ok(())
    }
}
