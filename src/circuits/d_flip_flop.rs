use crate::IO;
// use crate::{Gate, Kind};
// use super::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]

pub struct DFlipFlop {
    d: IO,
    g: IO,
    clock: IO,
    reset: IO,
    write: IO,
    read: IO,
    name: String,
}



// impl Node for DFlipFlop {
//     fn calc(&mut self) -> &Self {
//         let name = format!("DFLIPFLOP:{}", self.name);

//         let input = self.d;
//         let clock = Gate::new(Kind::And, [self.clock, self.write]);
//         // let clock = self.g.and2(self.clock, self.write, name.clone());
//         let ninput = Gate::new(Kind::Not, [input, input]);
//         // let ninput = g.not1(input, name.clone());
    
//         // let s_and = g.and2(input, clock, name.clone());
//         let s_and = Gate::new(Kind::And, [input, clock.out()]);
//         // let r_and = g.and2(ninput, clock, name.clone());
//         let r_and = Gate::new(Kind::And, [ninput.out(), clock.out()]);
    
//         // let r_or = g.or2(r_and, reset, name.clone());
//         let r_or = Gate::new(Kind::Or, [r_and.out(), self.reset]);
    
//         // let q = sr_latch(g, s_and, r_or, name.clone());
        
//         // g.and2(q, read, name)
//         self

//         // let name = format!("SRLATCH:{}", self.name);
//         // self.q = Gate::new(Kind::Nor, [self.r, io!(false)]).out();
//         // self.qn = Gate::new(Kind::Nor, [self.s, self.q]).out();
//         // self
//     }
// }




