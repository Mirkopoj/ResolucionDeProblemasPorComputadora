mod motor;
mod decision_maker;

use motor::jugador::Jugador;
use motor::mazo::Mazo;
use motor::mesa::Mesa;

use decision_maker::dumb_decider::DumbDecider;

fn main() {
    let mut mazo = Mazo::new();
    let mut mesa = Mesa::new(6);
    let mut jugadores: Vec<Jugador<DumbDecider>> = Vec::new();
    for i in mesa.indices_de_turnos() {
        jugadores.push(Jugador::new(DumbDecider::new(), i))
    }

    for _ in 0..15 {

        mazo.mezclar();

        mazo.repartir(&mut jugadores);

        println!("{}", mesa);

        let mut ganador = None;

        for _ in 0..3 {
            for i in mesa.indices_de_turnos() {
                jugadores[i].turno(&mut mesa);
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
