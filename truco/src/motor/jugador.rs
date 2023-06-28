use itertools::Itertools;
use std::{cmp::Ordering, fmt::Display};

use crate::decision_maker::{Decider, Decision};
use crate::motor::carta::Carta;
use crate::motor::mesa::Mesa;

#[derive(Debug)]
pub struct Jugador<DecisionMaker: Decider + ?Sized> {
    pub(crate) avatar: Avatar,
    decision_maker: Box<DecisionMaker>,
}

#[derive(Debug, Clone, Copy)]
pub struct Avatar {
    pub(crate) mano: [Option<Carta>; 3],
    pub(crate) posicion: usize,
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

impl<DecisionMaker: Decider + ?Sized> Jugador<DecisionMaker> {
    pub fn new(decision_maker: Box<DecisionMaker>, posicion: usize) -> Jugador<DecisionMaker> {
        Jugador {
            avatar: Avatar {
                mano: [None, None, None],
                posicion,
            },
            decision_maker,
        }
    }

    #[allow(unused)]
    fn calcular_envido(&self) -> Vec<Envido> {
        let mut ret = Vec::new();
        let mano: Vec<Carta> = self
            .avatar
            .mano
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();
        let e: Result<(), ()> = Ok(());
        for carta in &mano {
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

     fn tirar(&mut self, carta: usize, mesa: &mut Mesa) {
        let index = match mesa.cartas[self.avatar.posicion]
            .iter()
            .position(|c| c.is_none())
        {
            Some(i) => i,
            None => return,
        };
        mesa.cartas[self.avatar.posicion][index] = self.avatar.mano[carta].take();
    }

    pub fn turno(&mut self, mesa: &mut Mesa) {
        loop {
            match self.decision_maker.decide(&self.avatar, mesa) {
                Decision::Tirar(carta) => {
                    self.tirar(carta, mesa);
                    break;
                }
                Decision::Mazo => break,
            }
        }
    }
}

impl<DecisionMaker: Decider> Display for Jugador<DecisionMaker> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.avatar)
    }
}

impl Display for Avatar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for carta in self.mano {
            match carta {
                Some(c) => write!(f, " {}", c),
                None => write!(f, "    "),
            }?;
        }
        writeln!(f, "")
    }
}
