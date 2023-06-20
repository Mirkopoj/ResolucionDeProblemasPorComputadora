mod motor;
mod decision_maker;

use decision_maker::Decider;
use motor::jugador::Jugador;
use motor::mazo::Mazo;
use motor::mesa::Mesa;

use decision_maker::dumb_decider::DumbDecider;
use decision_maker::minimax_decider::MinimaxDecider;

fn main() {
    let mut mazo = Mazo::new();
    let mut mesa = Mesa::new(6);
    let mut jugadores: Vec<Jugador<dyn Decider>> = Vec::new();
    for i in mesa.indices_de_turnos() {
        if i%2==0 {
            jugadores.push(Jugador::new(Box::new(DumbDecider::new()), i))
        } else {
            jugadores.push(Jugador::new(Box::new(MinimaxDecider::new()), i))
        };
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
