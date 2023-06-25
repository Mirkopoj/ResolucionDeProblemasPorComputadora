mod decision_maker;
mod motor;

use decision_maker::Decider;
use motor::contador::Contador;
use motor::jugador::Jugador;
use motor::mazo::Mazo;
use motor::mesa::Mesa;

use decision_maker::dumb_decider::DumbDecider;
use decision_maker::human_decider::HumanDecider;
use decision_maker::minimax_decider::MinimaxDecider;

fn main() {
    let mut mazo = Mazo::new();
    let mut mesa = Mesa::new(2);
    let mut contador = Contador::new(false);
    let mut jugadores: Vec<Jugador<dyn Decider>> = Vec::new();
    for i in mesa.indices_de_turnos() {
        if i == 0 {
            jugadores.push(Jugador::new(Box::new(HumanDecider::new()), i));
            continue;
        }
        if i % 2 == 0 {
            jugadores.push(Jugador::new(Box::new(DumbDecider::new()), i));
        } else {
            jugadores.push(Jugador::new(Box::new(MinimaxDecider::new()), i));
        }
    }

    loop {
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

        if contador.sumar(ganador) {
            break;
        }

        println!("{}", contador);
        mesa.siguiente();
    }
    println!("{}", contador);
    println!();
    println!("Ganador {}", contador.ganador());
}
