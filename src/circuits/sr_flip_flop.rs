use super::*;
use crate::{IO, OFF, ON, NONE, DELAY};
use crate::{Gate, Kind};
use std::fmt::{Display, Formatter};
use std::{thread, time::Duration};


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SRFlipFlop {
    //inputs
    c: IO,
    s: IO,
    r: IO,
    //outputs
    q: IO,
    qn: IO,
    //internal
    srlatch: SRLatch,
    name: Option<String>,
}

impl SRFlipFlop {
    pub fn new(c: IO, s: IO, r: IO, name: Option<String>) -> Self {
        SRFlipFlop {
            c,
            s,
            r,
            q: NONE,
            qn: NONE,
            srlatch: SRLatch::new(OFF, ON, None),
            name,
        }
    }
    pub fn get_s(&self) -> IO { self.s }
    pub fn set_s(&mut self, io: IO) { self.s = io }
    pub fn get_r(&self) -> IO { self.r }
    pub fn set_r(&mut self, io: IO) { self.r = io }
    pub fn get_c(&self) -> IO { self.c }
    pub fn set_c(&mut self, io: IO) { self.c = io }
    pub fn get_q(&self) -> IO { self.q }
    pub fn get_qn(&self) -> IO { self.qn }
    
    pub fn name(&mut self, name: String) { self.name = Some(name) }
    pub fn print(&self) { println!("{}", self);}
}

impl Node for SRFlipFlop {
    #[inline(always)]
    fn is_init(&self) -> bool { self.q.is_init() && self.qn.is_init() }
    fn calc(&mut self) -> &Self {
        thread::sleep(Duration::from_nanos(DELAY));
        if self.s.is_on() && self.r.is_on() {
            self.q =  NONE;
            self.qn = NONE;
            return self;
        }
        let mut s_1 = Gate::new(Kind::Nand, [self.s, self.c], None);
        s_1.calc();
        let mut r_1 = Gate::new(Kind::Nand, [self.r, self.c], None);
        r_1.calc();
        self.srlatch.set_s(s_1.out());
        self.srlatch.set_r(r_1.out());
        self.srlatch.calc();
        self.q = self.srlatch.get_q();
        self.qn = self.srlatch.get_qn();
        self
    }
    fn calc_op(&mut self) -> &Self {
        self.calc()
    }
}

impl Display for SRFlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_none() { "SRFLIPFLOP".to_string() } else { format!("SRFLIPFLOP:{}", self.name.clone().unwrap()) };
        write!(f, "kind: <{}>, clock: <{}>, inputs: <{}, {}>, outputs: <{}, {}>",
            name,
            self.c,
            self.s,
            self.r,
            self.q,
            self.qn
        )
    }
}


