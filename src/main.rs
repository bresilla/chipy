#[macro_use]
mod gate;
pub use gate::*;
pub use gate::{Gate, Kind,};
mod circuits;
pub use circuits::*;
use std::{thread, time::Duration};

static DELAY: u64 = 1;

fn main() {
    thread::sleep(Duration::from_nanos(DELAY));

    // let mut or = Gate::new(Kind::Or, [OFF, ON]);
    // or.calc();
    // or.print();
    // let mut and = Gate::new(Kind::And, [OFF, ON]);
    // and.calc();
    // and.print();
    // let mut nor = Gate::new(Kind::Nor, [OFF, OFF]);
    // nor.calc();
    // nor.print();
    // let mut not = Gate::new(Kind::Not, [OFF, ON]);
    // not.calc();
    // not.print();

    let mut srl = SRLatch::new(OFF, ON, None);
    // srl.print();
    srl.calc();
    srl.print();

    // srl.calc();
    // srl.print();

    // println!("{}", srl);
}