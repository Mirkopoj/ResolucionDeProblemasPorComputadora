use super::{Decider, Decision};
use crate::motor::jugador::Avatar;
use crate::motor::mesa::Mesa;

#[derive(Debug)]
pub struct DumbDecider;

impl Decider for DumbDecider {
    
    fn decide(&self, jugador: &Avatar, _: &Mesa) -> Decision{
        for i in 0..3 {
            if jugador.mano[i].is_some() {
                return Decision::Tirar(i);
            }
        }
        Decision::Mazo
    }

}

impl DumbDecider {
    pub fn new() -> DumbDecider {
        DumbDecider {  }
    }
}