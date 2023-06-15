use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::carta::{Carta, Numero, Palo};
use crate::Jugador;

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

    pub fn repartir(&self, numero_de_jugadores: usize) -> Vec<Jugador> {
        (0..numero_de_jugadores)
            .map(|numero_de_jugador| Jugador {
                mano: [
                    Some(self.cartas[numero_de_jugador]),
                    Some(self.cartas[numero_de_jugador + numero_de_jugadores]),
                    Some(self.cartas[numero_de_jugador + numero_de_jugadores * 2]),
                ],
                posicion: numero_de_jugador,
            })
            .collect()
    }

}
