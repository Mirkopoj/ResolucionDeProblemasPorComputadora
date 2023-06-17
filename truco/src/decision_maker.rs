pub mod dumb_decider;
mod minimax_decider;
mod human_decider;

use crate::motor::jugador::Jugador;
use crate::motor::mesa::Mesa;

pub enum Decision{
    Tirar(usize),
    Mazo,
}

pub trait Decider {

    fn decide(&self, jugador: &Jugador<Self>, mesa: &Mesa) -> Decision;
    
}
