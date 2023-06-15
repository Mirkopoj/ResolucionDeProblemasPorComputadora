use enum_iterator::Sequence;

#[derive(Sequence, Debug, Clone, Copy, PartialEq)]
pub enum Palo {
    Espada,
    Basto,
    Oro,
    Copa,
}

#[derive(Sequence, Debug, Clone, Copy)]
pub enum Numero {
    Ancho,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Sota,
    Caballo,
    Rey,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Carta {
    pub(super)palo: Palo,
    numero: Numero,
    pub valor_tantos: u8,
    pub valor_juego: u8,
}

fn valor_juego(numero: Numero, palo: Palo) -> u8 {
    match (numero, palo) {
        (Numero::Ancho, Palo::Espada) => 14,
        (Numero::Ancho, Palo::Basto) => 13,
        (Numero::Siete, Palo::Espada) => 12,
        (Numero::Siete, Palo::Oro) => 11,
        (Numero::Tres, _) => 10,
        (Numero::Dos, _) => 9,
        (Numero::Ancho, Palo::Copa | Palo::Oro) => 8,
        (Numero::Rey, _) => 7,
        (Numero::Caballo, _) => 6,
        (Numero::Sota, _) => 5,
        (Numero::Siete, Palo::Copa | Palo::Basto) => 4,
        (Numero::Seis, _) => 3,
        (Numero::Cinco, _) => 2,
        (Numero::Cuatro, _) => 1,
    }
}

fn valor_tantos(numero: Numero) -> u8 {
    match numero {
        Numero::Ancho => 1,
        Numero::Dos => 2,
        Numero::Tres => 3,
        Numero::Cuatro => 4,
        Numero::Cinco => 5,
        Numero::Seis => 6,
        Numero::Siete => 7,
        _ => 0,
    }
}

impl Carta {
    pub fn new(numero: Numero, palo: Palo) -> Carta {
        let tantos = valor_tantos(numero);
        let valor = valor_juego(numero, palo);
        Carta {
            palo,
            numero,
            valor_juego: valor,
            valor_tantos: tantos,
        }
    }
}
