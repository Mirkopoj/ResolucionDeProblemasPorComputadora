pub mod dumb_decider;
pub mod minimax_decider;
pub mod human_decider;

use crate::motor::jugador::Avatar;
use crate::motor::mesa::Mesa;

#[derive(Debug, Clone, Copy)]
pub enum Decision{
    Tirar(usize),
    Mazo,
}

pub trait Decider {

    fn decide(&self, jugador: &Avatar, mesa: &Mesa) -> Decision;
    
}
