use super::{Decider, Decision};
use crate::motor::jugador::Avatar;
use crate::motor::mesa::Mesa;
use std::io;

#[derive(Debug)]
pub struct HumanDecider;

impl Decider for HumanDecider {
    fn decide(&mut self, jugador: &Avatar, _: &Mesa) -> Decision {
        loop {
            println!("{}", jugador);
            for i in 0..3 {
                let opcion = if jugador.carta(i).is_some() {
                    format!("  {} ", i + 1)
                } else {
                    "    ".to_string()
                };
                print!("{}", opcion);
            }
            println!("  4 -> Mazo");
            let mut user_input = String::new();
            let stdin = io::stdin();
            stdin
                .read_line(&mut user_input)
                .expect("Error leyendo la entrada del usuario");
            match user_input.replace(|c: char| !c.is_digit(5), "").parse() {
                Ok(n) => {
                    match n {
                        1 | 2 | 3 => {
                            let n = n - 1;
                            if jugador.carta(n).is_some() {
                                return Decision::Tirar(n);
                            }
                        }
                        4 => {
                            return Decision::Mazo;
                        }
                        _ => {}
                    };
                }
                Err(e) => {
                    println!("{}", e);
                }
            };
            println!("Ingrese una opcion valida");
        }
    }
}

impl HumanDecider {
    pub fn new() -> HumanDecider {
        HumanDecider {}
    }
}
