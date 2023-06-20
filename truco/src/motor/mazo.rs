use itertools::enumerate;
use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::decision_maker::Decider;
use crate::motor::carta::{Carta, Numero, Palo};
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

    pub fn repartir<T: Decider + ?Sized>(&self, jugadores: &mut Vec<Jugador<T>>) {
        let numero_de_jugadores = jugadores.len();
        for (numero_de_jugador, mut jugador) in enumerate(jugadores) {
            jugador.avatar.mano =  [
                    Some(self.cartas[numero_de_jugador]),
                    Some(self.cartas[numero_de_jugador + numero_de_jugadores]),
                    Some(self.cartas[numero_de_jugador + numero_de_jugadores * 2]),
                ];
        }
    }
}
