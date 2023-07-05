mod motor;

use motor::mazo::Mazo;
use motor::mesa::Mesa;

fn main() {
    let mut mazo = Mazo::new();
    let mut mesa = Mesa::new(6);

    for _ in 0..15 {
        mazo.mezclar();
        let mut jugadores = mazo.repartir(&mesa);

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
