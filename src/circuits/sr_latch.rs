use super::*;
use crate::{IO, OFF, ON, NONE};
use crate::{Gate, Kind};
use std::fmt::{Display, Formatter};

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
    pub fn set_s(&mut self, new_state: bool) -> &Self {
        self.s.set(new_state);
        self
    }
    pub fn get_s(&self) -> IO { self.s }
    pub fn set_r(&mut self, new_state: bool) -> &Self {
        self.r.set(new_state);
        self
    }
    pub fn get_r(&self) -> IO { self.r }

    pub fn get_q(&self) -> IO { self.q }
    pub fn get_qn(&self) -> IO { self.qn }
    
    pub fn name(&mut self, name: String) -> &Self {
        self.name = Some(name);
        self
    }
    pub fn print(&self) { println!("{}", self);}
}

impl Node for SRLatch {
    #[inline(always)]
    fn is_init(&self) -> bool { self.q.is_init() && self.qn.is_init() }
    fn calc(&mut self) -> &Self {
        if !self.is_init() {         
            self.q =  OFF;
            self.qn = ON; 
        }
        let name = if self.name.is_none() { None } else { Some("Q".to_string()) };
        let mut q = Gate::new(Kind::Nor, [self.r, self.qn], name);
        q.calc();
        self.q = q.out();

        let mut qn = Gate::new(Kind::Nor, [self.s, self.q], Some("QN".to_string()));
        qn.calc();
        self.qn = qn.out();
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
