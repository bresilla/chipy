use super::*;
use crate::{IO, OFF, ON, NONE, DELAY};
use crate::{Gate, Kind};
use std::fmt::{Display, Formatter};
use std::{thread, time::Duration};


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DFlipFlop {
    //inputs
    c: IO,
    d: IO,
    //outputs
    q: IO,
    qn: IO,
    //internal
    sr_flip_flop: SRFlipFlop,
    name: Option<String>,
}

impl DFlipFlop {
    pub fn new(c: IO, d: IO, name: Option<String>) -> Self {
        DFlipFlop {
            c,
            d,
            q: NONE,
            qn: NONE,
            sr_flip_flop: SRFlipFlop::new(OFF, OFF, ON, None),
            name,
        }
    }
    pub fn get_d(&self) -> IO { self.d }
    pub fn set_d(&mut self, io: IO) { self.d = io }
    pub fn get_c(&self) -> IO { self.c }
    pub fn set_c(&mut self, io: IO) { self.c = io }
    pub fn get_q(&self) -> IO { self.q }
    pub fn get_qn(&self) -> IO { self.qn }
    
    pub fn name(&mut self, name: String) { self.name = Some(name) }
    pub fn print(&self) { println!("{}", self);}
}

impl Node for DFlipFlop {
    #[inline(always)]
    fn is_init(&self) -> bool { self.q.is_init() && self.qn.is_init() }
    fn calc(&mut self) -> &Self {
        thread::sleep(Duration::from_nanos(DELAY));
        let mut d_not = Gate::new(Kind::Not, [self.d, self.d], None);
        d_not.calc();
        self.sr_flip_flop.set_c(self.c);
        self.sr_flip_flop.set_s(self.d);
        self.sr_flip_flop.set_r(d_not.out());
        self.sr_flip_flop.calc();
        self.q = self.sr_flip_flop.get_q();
        self.qn = self.sr_flip_flop.get_qn();
        self
    }
    fn calc_op(&mut self) -> &Self {
        self.calc()
    }
}

impl Display for DFlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_none() { "DFLIPFLOP".to_string() } else { format!("DFLIPFLOP:{}", self.name.clone().unwrap()) };
        write!(f, "kind: <{}>, clock: <{}>, input: <{}>, outputs: <{}, {}>",
            name,
            self.c,
            self.d,
            self.q,
            self.qn
        )
    }
}


