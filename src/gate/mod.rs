#[macro_use]
mod io;
pub use io::*;
mod kind;
pub use kind::*;
mod gate;
pub use gate::*;

pub trait State {
    fn is_init(&self) -> bool;
}