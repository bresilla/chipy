use std::fmt::{self, Display, Formatter};
use colorized::*;
use std::ops;

#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct IO {
    pub state: Option<bool>,
}

macro_rules! io {
    ( $x:expr ) => {{
        IO { state: Some($x) }
    }};
}

pub const NONE: IO = IO { state: None };
pub const OFF: IO = io!(false);
pub const ON: IO = io!(true);

impl IO {
    pub fn new() -> IO { IO { state: None } }
    pub fn set(&mut self, new_state: bool) -> &Self { 
        self.state = Some(new_state);
        self
    }

    pub fn flip(&mut self) -> &Self { 
        match self.state {
            Some(true) => self.state = Some(false),
            Some(false) => self.state = Some(true),
            None => { unreachable!("NONE") }
        }
        self
    }
    pub fn is_off(&self) -> bool { *self == OFF }
    pub fn is_on(&self) -> bool { *self == ON }
    #[inline(always)]
    pub fn is_init(&self) -> bool { *self == OFF || *self == ON }
    pub fn print(&self) {
        println!("{}", self);
    }
}

impl ops::Not for IO {
    type Output = IO;
    fn not(self) -> Self::Output {
        match self.state {
            Some(true) => OFF,
            Some(false) => ON,
            None => NONE
        }
    }
}

impl Display for IO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.state {
            Some(true) => write!(f, "{}", String::from("1").color(Colors::GreenFg)),
            Some(false) => write!(f, "{}", String::from("0").color(Colors::RedFg)),
            None => { write!(f, "/") }
        }
    }
}