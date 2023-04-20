use crate::State;

use super::io::*;
use super::kind::*;
use std::fmt::{Display, Formatter};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Gate {
    kind: Kind,
    input: [IO; 2],
    output: IO,
    //internal
    name: Option<String>,
}

impl Gate {
    pub fn new(kind: Kind, inputs: [IO; 2], name: Option<String>) -> Self {
        Gate { kind, input: inputs, output: IO::new(), name }
    }
    pub fn calc(&mut self) -> &Self {
        if self.input[0].is_init() && self.input[1].is_init() {
            self.output = self.kind.calc(
                self.input[0].state.unwrap(), 
                self.input[1].state.unwrap()
            );
        }
        self
    }
    pub fn out(&self) -> IO { self.output }
    pub fn set(&mut self, index: usize, io: IO) -> &Self {
        self.input[index];
        self
    }
    pub fn name(&mut self, name: String) -> &Self {
        self.name = Some(name);
        self
    }
    pub fn print(&self) {
        println!("{}", self);
    }
}

impl State for Gate {
    #[inline(always)]
    fn is_init(&self) -> bool { self.out() == OFF || self.out() == ON }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_none() {  self.kind.to_string() } else { format!("{}:{}",  self.kind.to_string(), self.name.clone().unwrap()) };
        write!(f, "kind: <{}>, inputs: <{}, {}>, output: <{}>", 
            name,
            self.input[0],
            self.input[1], 
            self.output
        )
    }
}