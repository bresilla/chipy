#[macro_use]
mod sr_flip_flop;
pub use sr_flip_flop::*;
mod sr_latch;
pub use sr_latch::*;
mod d_flip_flop;
pub use d_flip_flop::*;

// in nanoseconds
static DELAY: u64 = 1;
pub trait Node {
    fn is_init(&self) -> bool;
    fn calc(&mut self) -> &Self;
    fn calc_op(&mut self) -> &Self;
    // fn out(&mut self) -> IO;
    // fn set(&mut self) -> &Self;
}
