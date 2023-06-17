use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::motor::carta::{Carta, Numero, Palo};
use crate::motor::mesa::Mesa;
use crate::motor::jugador::Jugador;

pub struct Mazo {
    cartas: Vec<Carta>,
}

impl Mazo {
    pub fn new() -> Mazo {
        Mazo {
            cartas: all::<Palo>()
                .map(|pal| all::<Numero>().map(move |num| Carta::new(num, pal)))
                .flatten()
                .collect(),
        }
    }

    pub fn mezclar(&mut self) {
        let mut rng = thread_rng();
        self.cartas.shuffle(&mut rng);
    }

    pub fn repartir(&self, mesa: &Mesa) -> Vec<Jugador> {
        (0..mesa.numero_de_jugadores)
            .map(|numero_de_jugador| Jugador {
                mano: [
                    Some(self.cartas[numero_de_jugador]),
                    Some(self.cartas[numero_de_jugador + mesa.numero_de_jugadores]),
                    Some(self.cartas[numero_de_jugador + mesa.numero_de_jugadores * 2]),
                ],
                posicion: numero_de_jugador,
            })
            .collect()
    }
}
