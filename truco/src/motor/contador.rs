use std::fmt::Display;
use crate::motor::mesa::Equipo;

pub struct Contador {
    puntos: [u8; 2],
    a_30: bool,
}

impl Contador {

    pub fn new(a_30: bool) -> Contador {
        Contador { puntos: [0;2], a_30 }
    }
    
    pub fn sumar(&mut self, ganador: Option<Equipo>) -> bool {
        if let Some(equipo) = ganador {
            match equipo {
                Equipo::Nosotros => { self.puntos[0] += 1; },
                Equipo::Ellos => { self.puntos[1] += 1; },
            };
        }
        let puntos_ganadores = if self.a_30 {30} else {15};
        let hay_ganador = if puntos_ganadores <= self.puntos[0].max(self.puntos[1]) 
            { true } else { false };

        hay_ganador
    }
}


impl Display for Contador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " Nosotros │ Ellos")?;
        writeln!(f, "──────────┼──────────")?;

        for i in 0..3 {
            let n_1 = if self.puntos[0] > (0+i*5) {"▁"} else {" "};
            let e_1 = if self.puntos[1] > (0+i*5) {"▁"} else {" "};
            let n_2 = if self.puntos[0] > (1+i*5) {"▕"} else {" "};
            let e_2 = if self.puntos[1] > (1+i*5) {"▕"} else {" "};
            let n_3 = if self.puntos[0] > (2+i*5) {"▏"} else {" "};
            let e_3 = if self.puntos[1] > (2+i*5) {"▏"} else {" "};
            let n_4 = if self.puntos[0] > (3+i*5) {"▔"} else {" "};
            let e_4 = if self.puntos[1] > (3+i*5) {"▔"} else {" "};
            let n_5 = if self.puntos[0] > (4+i*5) {"╱"} else {" "};
            let e_5 = if self.puntos[1] > (4+i*5) {"╱"} else {" "};
            writeln!(f, "    {}     │    {}     ", n_1, e_1)?;
            writeln!(f, "   {}{}{}    │   {}{}{}    ", n_2, n_5, n_3, e_2, e_5, e_3)?;
            writeln!(f, "    {}     │    {}     ", n_4, e_4)?;
        }
        if self.a_30 {
            writeln!(f, "──────────┼──────────")?;
            for i in 3..6 {
                let n_1 = if self.puntos[0] > (0+i*5) {"▁"} else {" "};
                let e_1 = if self.puntos[1] > (0+i*5) {"▁"} else {" "};
                let n_2 = if self.puntos[0] > (1+i*5) {"▕"} else {" "};
                let e_2 = if self.puntos[1] > (1+i*5) {"▕"} else {" "};
                let n_3 = if self.puntos[0] > (2+i*5) {"▏"} else {" "};
                let e_3 = if self.puntos[1] > (2+i*5) {"▏"} else {" "};
                let n_4 = if self.puntos[0] > (3+i*5) {"▔"} else {" "};
                let e_4 = if self.puntos[1] > (3+i*5) {"▔"} else {" "};
                let n_5 = if self.puntos[0] > (4+i*5) {"╱"} else {" "};
                let e_5 = if self.puntos[1] > (4+i*5) {"╱"} else {" "};
                writeln!(f, "    {}     │    {}     ", n_1, e_1)?;
                writeln!(f, "   {}{}{}    │   {}{}{}    ", n_2, n_5, n_3, e_2, e_5, e_3)?;
                writeln!(f, "    {}     │    {}     ", n_4, e_4)?;
            } 
        }


        write!(f,"")
    }
}


