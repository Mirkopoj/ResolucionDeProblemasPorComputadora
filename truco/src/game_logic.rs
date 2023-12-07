use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    task::JoinHandle,
};

use crate::{
    game_commands::{GameCommand, GAME_COMMANDS},
    motor::{
        contador::Contador,
        jugador::Jugador,
        mazo::Mazo,
        mesa::{Equipo, Mesa},
    },
};

pub struct ServerPlayer {
    jugador: Jugador,
    name: String,
    stream: OwnedWriteHalf,
    join_handle: JoinHandle<OwnedReadHalf>,
}

impl ServerPlayer {
    pub fn new(
        name: String,
        stream: OwnedWriteHalf,
        pos: usize,
        join_handle: JoinHandle<OwnedReadHalf>,
    ) -> Self {
        Self {
            jugador: Jugador::new(pos),
            name,
            stream,
            join_handle,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn server_tuple(&mut self) -> (&mut String, &mut OwnedWriteHalf) {
        (&mut self.name, &mut self.stream)
    }

    pub fn player(&mut self) -> &mut Jugador {
        &mut self.jugador
    }

    pub async fn join(self) -> (OwnedReadHalf, OwnedWriteHalf ) {
        (self.join_handle.await.unwrap(), self.stream) 
    }
}

pub struct GameLogic {
    name: String,
    mazo: Mazo,
    mesa: Mesa,
    contador: Contador,
    jugadores: Vec<ServerPlayer>,
    ganador: Option<Equipo>,
    tiene_envido: PoseedorEnvido,
}

impl GameLogic {
    pub fn new(name: String, chairs: usize, a_treinta: bool) -> Self {
        Self {
            name,
            mazo: Mazo::new(),
            mesa: Mesa::new(chairs),
            contador: Contador::new(a_treinta),
            jugadores: Vec::with_capacity(chairs),
            ganador: None,
            tiene_envido: PoseedorEnvido::Nadie,
        }
    }

    pub fn add_player(
        &mut self,
        name: String,
        stream: OwnedWriteHalf,
        join_handle: JoinHandle<OwnedReadHalf>,
    ) -> bool {
        self.jugadores.push(ServerPlayer::new(
            name,
            stream,
            self.jugadores.len(),
            join_handle,
        ));
        if self.player_count() == self.mesa.numero_de_jugadores() {
            println!("Table {}: Begining game", self.name);
            self.init();
            return true;
        }
        false
    }

    pub fn player_count(&self) -> usize {
        self.jugadores.len()
    }

    pub fn players(&mut self) -> &mut Vec<ServerPlayer> {
        &mut self.jugadores
    }

    pub fn streams(self) -> Vec<ServerPlayer> {
        self.jugadores
    }

    pub fn tirar(&mut self, carta: usize, name: &String) -> Result<(), String> {
        let jugador = self
            .jugadores
            .iter_mut()
            .find(|p| p.name() == name)
            .unwrap()
            .player();
        if jugador.carta(carta).is_none() {
            return Err("Card allready played\n".to_string());
        }
        jugador.tirar(carta, &mut self.mesa);
        Ok(())
    }

    pub fn mesa(&self) -> &Mesa {
        &self.mesa
    }

    pub fn game_over(&self) -> bool {
        self.contador.hay_ganador()
    }

    pub fn try_fin_ronda(&mut self) -> String {
        if self.mesa.ronda_finalizada() {
            self.tiene_envido = PoseedorEnvido::Nadie;
            self.mesa.final_de_ronda();
            let ganador = self.mesa.ganador();
            if ganador.is_some() {
                if self.contador.sumar(ganador) {
                    return format!("{:#}\nGanador {}\n", self.contador, self.contador.ganador());
                } else {
                    self.mesa.siguiente();
                    self.init();
                    return format!("{:#}\n", self.contador);
                }
            }
        }
        "".to_string()
    }

    pub fn available_game_commands(&self, name: &String) -> Vec<&'static GameCommand> {
        if name == "  start  " {
            return vec![GAME_COMMANDS.iter().find(|c| c.name() == "start").unwrap()];
        }
        let mut ret = Vec::new();
        if let Some(jugador) = self.jugadores.iter().find(|p| p.name == *name) {
            if self.es_su_turno(jugador) {
                ret.push(GAME_COMMANDS.iter().find(|c| c.name() == "tirar").unwrap());
                ret.push(GAME_COMMANDS.iter().find(|c| c.name() == "mazo").unwrap());
            }
            if self.tiene_envido(jugador) {
                ret.push(GAME_COMMANDS.iter().find(|c| c.name() == "envido").unwrap());
            }
            if self.tiene_truco(jugador) {
                ret.push(GAME_COMMANDS.iter().find(|c| c.name() == "truco").unwrap());
            }
        }
        ret
    }

    fn es_su_turno(&self, jugador: &ServerPlayer) -> bool {
        self.mesa.posicion_del_turno() == jugador.jugador.posicion()
    }

    fn tiene_envido(&self, jugador: &ServerPlayer) -> bool {
        match self.tiene_envido {
            PoseedorEnvido::Turno => self.es_su_turno(jugador),
            PoseedorEnvido::Equipo(e) => jugador.jugador.equipo() == e,
            PoseedorEnvido::Nadie => false,
        }
    }

    fn tiene_truco(&self, _jugador: &ServerPlayer) -> bool {
        false
    }

    fn init(&mut self) {
        self.mazo.mezclar();
        self.mazo
            .repartir(self.jugadores.iter_mut().map(|p| p.player()).collect());
        self.ganador = None;
        self.tiene_envido = PoseedorEnvido::Turno;
    }
}

enum PoseedorEnvido {
    Turno,
    Equipo(Equipo),
    Nadie,
}
