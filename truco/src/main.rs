mod carta;
mod jugador;
mod mazo;
mod mesa;

use carta::*;
use jugador::*;
use mazo::*;
use mesa::*;

fn main() {
    let mut mazo = Mazo::new();

    mazo.mezclar();

    let mut mesa = Mesa::new(6);

    for _ in 0..15 {

        let mut jugadores = mazo.repartir(mesa.numero_de_jugadores.into());

        println!("{}", mesa);

        for jugador in &jugadores {
            println!("{}", jugador);
        }

        let mut ganador = None;

        for _ in 0..3 {
            for i in mesa.posicion_de_mano..mesa.posicion_de_mano + mesa.numero_de_jugadores {
                jugadores[i % mesa.numero_de_jugadores].turno(&mut mesa);
                println!("{}", mesa);
            }

            mesa.final_de_ronda();

            ganador = mesa.ganador();
            if ganador.is_some() {
                break;
            }
        }

        println!("Ganador {:?}", ganador);
        mesa.siguiente();
    }
}
