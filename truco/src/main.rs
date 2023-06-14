use std::cmp::Ordering;
use std::usize;
use enum_iterator::{all, Sequence};
use rand::seq::SliceRandom;
use rand::thread_rng;
use itertools::Itertools;

#[derive(Sequence, Debug, Clone, Copy, PartialEq)]
enum Palo {
    Espada,
    Basto,
    Oro,
    Copa
}

#[derive(Sequence, Debug, Clone, Copy)]
enum Numero {
    Ancho,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Sota,
    Caballo,
    Rey,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Carta {
    palo: Palo,
    numero: Numero,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Jugador {
    mano: [Option<Carta>;3],
    posicion: usize,
}

fn mezclar(pre_mazo: Vec<Carta>) -> Vec<Carta> {
    let mut mut_mazo = pre_mazo;
    let mut rng = thread_rng();

    mut_mazo.shuffle(&mut rng);

    mut_mazo
}

fn repartir(mazo: &Vec<Carta>, numero_de_jugadores: usize) -> Vec<Jugador> {
    (0..numero_de_jugadores)
        .map(|numero_de_jugador| 
             Jugador{
                 mano: [
                     Some(mazo[numero_de_jugador]),
                     Some(mazo[numero_de_jugador+numero_de_jugadores]),
                     Some(mazo[numero_de_jugador+numero_de_jugadores*2])
                 ],
                 posicion: numero_de_jugador
             })
        .collect()
}

#[derive(Debug, Eq)]
#[allow(dead_code)]
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
fn valor_juego(carta: Carta) -> u8 {
    match carta {
        Carta{ numero: Numero::Ancho, palo: Palo::Espada} => 14,
        Carta{ numero: Numero::Ancho, palo: Palo::Basto} => 13,
        Carta{ numero: Numero::Siete, palo: Palo::Espada} => 12,
        Carta{ numero: Numero::Siete, palo: Palo::Oro} => 11,
        Carta{ numero: Numero::Tres, palo: _} => 10,
        Carta{ numero: Numero::Dos, palo: _} => 9,
        Carta{ numero: Numero::Ancho, palo: Palo::Copa|Palo::Oro} => 8,
        Carta{ numero: Numero::Rey, palo: _} => 7,
        Carta{ numero: Numero::Caballo, palo: _} => 6,
        Carta{ numero: Numero::Sota, palo: _} => 5,
        Carta{ numero: Numero::Siete, palo: Palo::Copa|Palo::Basto} => 4,
        Carta{ numero: Numero::Seis, palo: _} => 3,
        Carta{ numero: Numero::Cinco, palo: _} => 2,
        Carta{ numero: Numero::Cuatro, palo: _} => 1,
    }
}

#[allow(unused)]
fn valor_tantos(carta: Carta) -> u8 {
    match carta.numero {
        Numero::Ancho => 1,
        Numero::Dos => 2,
        Numero::Tres => 3,
        Numero::Cuatro => 4,
        Numero::Cinco => 5,
        Numero::Seis => 6,
        Numero::Siete => 7,
        _ => 0
    }
}

#[allow(unused)]
fn calcular_envido(mano: [Carta;3]) -> Vec<Envido>{
    let mut ret = Vec::new();
    for carta in mano {
        ret.push(Envido{tantos: valor_tantos(carta), valor_revelado: valor_juego(carta)});
    }
    for par in mano.iter().combinations(2) {
        if par[0].palo != par[1].palo { continue; } 
        let mut tanto = 20;
        let mut valor = 0;
        for &carta in par {
            tanto += valor_tantos(carta);
            valor += valor_juego(carta);
        }
        ret.push(Envido { tantos: tanto, valor_revelado: valor });
    }
    ret.sort();
    ret
}

#[derive(Debug)]
struct Mesa {
    numero_de_jugadores: u8,
    cartas: Vec<[Option<Carta>;3]>,
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
    Ellos
}

fn ganador(mesa: Mesa) -> Option<Equipo> {
    let mut cont_ellos = 0;
    let mut cont_nosotros = 0;
    for ronda in 0..3 {
        if mesa
            .cartas
            .iter()
            .enumerate()
            .map(
                |(i, &juego)| 
                match juego[ronda] {
                    Some(v) => (i, valor_juego(v)),
                    None => (i, 0),
                }
                )
            .max_by_key(|(_, valor)| *valor)
            .map(|(indice, _)| indice)
            .unwrap() % 2 == 0
            {
                cont_nosotros += 1;
            } else {
                cont_ellos += 1;
            }
    }
    return if cont_ellos<cont_nosotros {
        Some(Equipo::Nosotros)
    } else {
        Some(Equipo::Ellos)
    };
}

fn main() {
    let mazo: Vec<Carta> = all::<Palo>()
        .map(|pal| all::<Numero>().map(move |num| Carta { palo: pal, numero: num }))
        .flatten()
        .collect();

    let mazo = mezclar(mazo);

    let num_jugadores = 6;

    let mut mesa = Mesa {
        numero_de_jugadores: num_jugadores,
        cartas: (0..num_jugadores).map(|_| [None;3]).collect(),
    };

    let mut jugadores = repartir(&mazo, mesa.numero_de_jugadores.into());

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
