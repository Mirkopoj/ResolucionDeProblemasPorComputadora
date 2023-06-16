mod carta;
mod mazo;
mod jugador;
mod mesa;

use carta::*;
use mazo::*;
use jugador::*;
use mesa::*;

fn main() {
    let mut mazo = Mazo::new();

    mazo.mezclar();

    let mut mesa = Mesa::new(2);

    let mut jugadores = mazo.repartir(mesa.numero_de_jugadores.into());

    println!("{:?}", mesa);

    for jugador in &jugadores {
        println!("{:?}", jugador);
    }

    for _ in 0..3 {
        for i in mesa.posicion_de_mano..mesa.posicion_de_mano+mesa.numero_de_jugadores {
            jugadores[i%mesa.numero_de_jugadores].turno(&mut mesa);
        }
        
        mesa.final_de_ronda();
        for carta in &mesa.cartas {
            println!("{:?}", carta);
        }
        println!("{:?}", mesa.rondas);
        for i in mesa.posicion_de_mano..mesa.posicion_de_mano+mesa.numero_de_jugadores {
            println!("{:?}", jugadores[i%mesa.numero_de_jugadores]);
        }
        
        let ganador = mesa.ganador();
        println!("ganador {:?}", ganador);
        if ganador.is_some() { break; }
        
    }

    println!("Terminado");
     
}
