use super::*;
use crate::{IO, OFF, ON, NONE, DELAY};
use crate::{Gate, Kind};
use std::fmt::{Display, Formatter};
use std::{thread, time::Duration};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SRLatch {
    //inputs
    s: IO,
    r: IO,
    //outputs
    q: IO,
    qn: IO,
    //internal
    name: Option<String>,
}

impl SRLatch {
    pub fn new(s: IO, r: IO, name: Option<String>) -> Self {
        SRLatch {
            s,
            r,
            q: NONE,
            qn: NONE,
            name,
        }
    }
    pub fn get_s(&self) -> IO { self.s }
    pub fn set_s(&mut self, io: IO) { self.s = io }
    pub fn get_r(&self) -> IO { self.r }
    pub fn set_r(&mut self, io: IO) { self.r = io }
    pub fn get_q(&self) -> IO { self.q }
    pub fn get_qn(&self) -> IO { self.qn }
    
    pub fn name(&mut self, name: String) { self.name = Some(name) }
    pub fn print(&self) { println!("{}", self);}
}

impl Node for SRLatch {
    #[inline(always)]
    fn is_init(&self) -> bool { self.q.is_init() && self.qn.is_init() }
    fn calc(&mut self) -> &Self {
        thread::sleep(Duration::from_nanos(DELAY));
        if !self.is_init() {         
            self.q =  OFF;
            self.qn = !self.q; 
        }
        let mut q = Gate::new(Kind::Nand, [self.s, self.qn], None);
        q.calc();
        self.q = q.out();
        let mut qn = Gate::new(Kind::Nand, [self.r, self.q], None);
        qn.calc();
        self.qn = qn.out();
        let mut q = Gate::new(Kind::Nand, [self.s, self.qn], None);
        q.calc();
        self.q = q.out();
        self
    }
    fn calc_op(&mut self) -> &Self {
        thread::sleep(Duration::from_nanos(DELAY));
        if !self.is_init() {         
            self.q =  OFF;
            self.qn = ON; 
        }
        if self.s.is_on() && self.r.is_off() {
            self.q = OFF;
        } else if self.s.is_off() && self.r.is_on() {
            self.q = ON;
        }
        self.qn = !self.q; 
        self
    }
}

impl Display for SRLatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_none() { "SRLATCH".to_string() } else { format!("SRLATCH:{}", self.name.clone().unwrap()) };
        write!(f, "kind: <{}>, inputs: <{}, {}>, outputs: <{}, {}>",
            name,
            self.s,
            self.r, 
            self.q,
            self.qn
        )
    }
}
