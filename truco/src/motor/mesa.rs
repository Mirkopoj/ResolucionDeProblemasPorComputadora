use colored::Colorize;
use std::fmt::Display;

use crate::motor::carta::Carta;

#[derive(Debug)]
pub struct Mesa {
    pub(super) numero_de_jugadores: usize,
    pub(super) cartas: Vec<[Option<Carta>; 3]>,
    pub(super) rondas: Vec<Option<Equipo>>,
    ronda_en_juego: usize,
    pub(super) posicion_de_mano: usize,
    cuenta_vueltas: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Equipo {
    Nosotros,
    Ellos,
}

impl Mesa {
    pub fn new(numero_de_jugadores: usize) -> Mesa {
        let cartas = (0..numero_de_jugadores).map(|_| [None; 3]).collect();
        Mesa {
            numero_de_jugadores,
            cartas,
            rondas: Vec::new(),
            ronda_en_juego: 0,
            posicion_de_mano: 0,
            cuenta_vueltas: 0,
        }
    }

    pub fn final_de_ronda(&mut self) {
        let valores_jugados =
            self.cartas
                .iter()
                .enumerate()
                .map(|(i, &juego)| match juego[self.ronda_en_juego] {
                    Some(v) => (i, v.valor_juego),
                    None => (i, 0),
                });
        let max_nosotros = valores_jugados
            .clone()
            .filter(|(i, _)| i % 2 == 0)
            .max_by_key(|(_, valor)| *valor)
            .map(|(_, valor)| valor)
            .unwrap();
        let max_ellos = valores_jugados
            .clone()
            .filter(|(i, _)| i % 2 == 1)
            .max_by_key(|(_, valor)| *valor)
            .map(|(_, valor)| valor)
            .unwrap();
        let max_index = valores_jugados
            .rev()
            .max_by_key(|(_, valor)| *valor)
            .map(|(indice, _)| indice)
            .unwrap();
        self.ronda_en_juego += 1;
        if max_nosotros == max_ellos {
            self.rondas.push(None);
            return;
        }
        self.posicion_de_mano = max_index;
        if max_nosotros > max_ellos {
            self.rondas.push(Some(Equipo::Nosotros));
        } else {
            self.rondas.push(Some(Equipo::Ellos));
        }
    }

    pub fn ganador(&self) -> Option<Equipo> {
        if self.rondas.len() < 2 {
            return None;
        }
        if self.rondas.len() > 3 {
            panic!();
        }
        let nos = self
            .rondas
            .iter()
            .filter(|&c| *c == Some(Equipo::Nosotros))
            .count();
        if nos >= 2 {
            return Some(Equipo::Nosotros);
        }
        let ellos = self
            .rondas
            .iter()
            .filter(|&c| *c == Some(Equipo::Ellos))
            .count();
        if ellos >= 2 {
            return Some(Equipo::Ellos);
        }
        if ellos + nos == self.rondas.len() {
            return None;
        }
        if self.rondas[0].is_some() {
            return self.rondas[0];
        }
        if self.rondas[1].is_some() {
            return self.rondas[1];
        }
        if self.rondas.len() < 3 {
            return None;
        }
        if self.rondas[2].is_some() {
            return self.rondas[2];
        }
        None
    }

    pub fn siguiente(&mut self){
        self.ronda_en_juego = 0;
        self.rondas = Vec::new();
        self.cuenta_vueltas += 1;
        self.posicion_de_mano = self.cuenta_vueltas%self.numero_de_jugadores;
        self.cartas = (0..self.numero_de_jugadores).map(|_| [None; 3]).collect();
    }

    pub fn indices_de_turnos(&self) -> Vec<usize> {
        (self.posicion_de_mano..self.posicion_de_mano+self.numero_de_jugadores)
            .map(|i| i%self.numero_de_jugadores)
            .collect()
    }
}

impl Display for Mesa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..3).rev() {
            let max_val = self.cartas.iter().filter(|&c| c[i].is_some()).map(|c| c[i].unwrap().valor_juego).max();
            for cartas in &self.cartas {
                match cartas[i] {
                    Some(c) => {
                        let mut prt = format!("{}", c).white();
                        if Some(c.valor_juego) == max_val {
                            prt = prt.green();
                        }
                        write!(f, " {}", prt)
                    },
                    None => write!(f, "    "),
                }?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
