use itertools::Itertools;
use std::cmp::Ordering;
use std::usize;

mod carta;
mod mazo;

use carta::*;
use mazo::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Jugador {
    mano: [Option<Carta>; 3],
    posicion: usize,
}

#[derive(Debug, Eq)] #[allow(dead_code)]
struct Envido {
    tantos: u8,
    valor_revelado: u8,
}

impl Ord for Envido {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.tantos.cmp(&other.tantos);
    }
}

impl PartialOrd for Envido {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Envido {
    fn eq(&self, other: &Self) -> bool {
        self.tantos == other.tantos
    }
}

#[allow(unused)]
fn calcular_envido(mano: [Carta; 3]) -> Vec<Envido> {
    let mut ret = Vec::new();
    for carta in mano {
        ret.push(Envido {
            tantos: carta.valor_tantos,
            valor_revelado: carta.valor_juego,
        });
    }
    for par in mano.iter().combinations(2) {
        if par[0].palo != par[1].palo {
            continue;
        }
        let mut tanto = 20;
        let mut valor = 0;
        for &carta in par {
            tanto += carta.valor_tantos;
            valor += carta.valor_juego;
        }
        ret.push(Envido {
            tantos: tanto,
            valor_revelado: valor,
        });
    }
    ret.sort();
    ret
}

#[derive(Debug)]
struct Mesa {
    numero_de_jugadores: u8,
    cartas: Vec<[Option<Carta>; 3]>,
}

fn tirar(jugador: usize, carta: &mut Option<Carta>, mesa: &mut Mesa) {
    let index = match mesa.cartas[jugador].iter().rposition(|c| c.is_none()) {
        Some(i) => i,
        None => return,
    };

    mesa.cartas[jugador][index] = carta.take();
}

#[derive(Debug)]
enum Equipo {
    Nosotros,
    Ellos,
}

fn ganador(mesa: Mesa) -> Option<Equipo> {
    let mut cont_ellos = 0;
    let mut cont_nosotros = 0;
    for ronda in 0..3 {
        if mesa
            .cartas
            .iter()
            .enumerate()
            .map(|(i, &juego)| match juego[ronda] {
                Some(v) => (i, v.valor_juego),
                None => (i, 0),
            })
            .max_by_key(|(_, valor)| *valor)
            .map(|(indice, _)| indice)
            .unwrap()
            % 2
            == 0
        {
            cont_nosotros += 1;
        } else {
            cont_ellos += 1;
        }
    }
    return if cont_ellos < cont_nosotros {
        Some(Equipo::Nosotros)
    } else {
        Some(Equipo::Ellos)
    };
}

fn main() {
    let mut mazo = Mazo::new();

    mazo.mezclar();

    let num_jugadores = 6;

    let mut mesa = Mesa {
        numero_de_jugadores: num_jugadores,
        cartas: (0..num_jugadores).map(|_| [None; 3]).collect(),
    };

    let mut jugadores = mazo.repartir(mesa.numero_de_jugadores.into());

    println!("{:?}", mesa);
    for ronda in 0..3 {
        for jugador in &jugadores {
            println!("{:?}", jugador);
        }
        for jugador in jugadores.iter_mut() {
            tirar(jugador.posicion, &mut jugador.mano[ronda], &mut mesa);
        }
        println!("{:?}", mesa);
    }
    for jugador in &jugadores {
        println!("{:?}", jugador);
    }
    let ganador = ganador(mesa);

    println!("{:?}", ganador);
}
