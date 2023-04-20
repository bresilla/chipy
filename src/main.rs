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
    // let mut and = Gate::new(Kind::Nand, [OFF, OFF], None);
    // and.calc();
    // and.print();
    // let mut nor = Gate::new(Kind::Nor, [OFF, OFF]);
    // nor.calc();
    // nor.print();
    // let mut not = Gate::new(Kind::Not, [OFF, ON]);
    // not.calc();
    // not.print();

    // let mut latch = SRLatch::new(ON, ON, None);
    // latch.calc();
    // latch.print();
    // latch.s_off();
    // latch.calc();
    // latch.print();
    // latch.calc();
    // latch.print();
    // latch.calc();
    // latch.print();
    // latch.s_on();
    // latch.calc();
    // latch.print();
    // latch.r_off();
    // latch.calc();
    // latch.print();
    

    let mut srl = DFlipFlop::new(OFF, OFF, None);
    // srl.print();
    srl.calc();
    srl.print();
    println!("----------------");
    srl.calc();
    srl.print();
    println!("----------------");
    srl.set_c(ON);
    srl.set_d(ON);
    srl.calc();
    srl.print();
    println!("----------------");
    srl.set_c(OFF);
    srl.calc();
    srl.print();
    println!("----------------");
    srl.set_c(ON);
    srl.calc();
    srl.print();
    println!("----------------");
    srl.set_c(OFF);
    srl.calc();
    srl.print();
    srl.set_c(ON);
    println!("----------------");
    srl.set_d(OFF);
    srl.calc();
    srl.print();
    println!("----------------");
    srl.set_c(OFF);
    srl.calc();
    srl.print();
}