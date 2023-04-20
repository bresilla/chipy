use super::io::*;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Kind {
    Xor,
    Xnor,
    Not,
    Or,
    And,
    Nand,
    Nor,
}

impl Kind {
    #[inline(always)]
    pub fn calc(&self, a: bool, b: bool) -> IO {
        match self {
            Kind::Or => io!(a | b),
            Kind::Nor => io!(!(a | b)),
            Kind::And => io!(a & b),
            Kind:: Nand => io!(!(a & b)),
            Kind::Xor => io!(a ^ b),
            Kind::Xnor => io!(!(a ^ b)),
            Kind::Not => io!(!a)
        }
    }

    #[inline(always)]
    pub fn init(&self) -> IO {
        match self {
            Kind::Or | Kind::Nor | Kind::Xor | Kind::Xnor => io!(false),
            Kind::And | Kind::Nand => io!(true),
            Kind::Not => io!(false)
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Not => write!(f, stringify!(NOT)),
            Kind::Or => write!(f, stringify!(OR)),
            Kind::Nor => write!(f, stringify!(NOR)),
            Kind::And => write!(f, stringify!(AND)),
            Kind::Nand => write!(f, stringify!(NAND)),
            Kind::Xor => write!(f, stringify!(XOR)),
            Kind::Xnor => write!(f, stringify!(XNOR)),
        }
    }
}


// #[cfg(test)]
// mod test {
//     use super::super::io::IO;
//     use smallvec::smallvec;
//     use super::*;
//     #[test]
//     fn test_swap_dependency() {
//         let mut g = Gate::new(Kind::Or, smallvec![state!(true), state!(false), state!(true)]);

//         assert_eq!(g.outputs[0], state!(true));
//         assert_eq!(g.outputs[1], state!(false));
//         assert_eq!(g.outputs[2], state!(false));
//     }
// }
