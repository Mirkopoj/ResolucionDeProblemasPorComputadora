use std::fmt::Display;

use crate::{game_logic::GameLogic, tables::Routing};

pub type GameCommandRet = Vec<(Routing, String)>;
pub struct GameCommand {
    name: &'static str,
    description: &'static str,
    run: fn(String, Vec<String>, &mut GameLogic) -> GameCommandRet,
}

impl GameCommand {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run(
        &self,
        name: String,
        commands: Vec<String>,
        game_logic: &mut GameLogic,
    ) -> GameCommandRet {
        (self.run)(name, commands, game_logic)
    }
}

impl Display for GameCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6}\t\t{}", self.name, self.description)
    }
}

pub const STATIC_COMMANDS: usize = 4;
pub const GAME_COMMANDS: &[GameCommand] = &[
    GameCommand {
       name: "help",
        description:
            "help {command0} {command1} .. gives the description of a command, otherwise describes all commands",
        run: help,
    },
    GameCommand {
        name: "seña",
        description: "seña <seña a pasar[1E, 1B, 7E, 7O, 3, 2, 1F, E{mbido}]>, pasa la seña pedida a quien esté mirando",
        run: seña,
    },
    GameCommand {
        name: "mirar",
        description: "mirar <a donde mirar[0..6, mano]>",
        run: mirar,
    },
    GameCommand {
        name: "msg",
        description: "msg <mensaje a enviar>, manda un mensaje a todos los jugadores",
        run: msg,
    },
    GameCommand {
        name: "tirar",
        description: "tirar <carta a tirar[0, 1, 2]>, tira la carta elegida",
        run: tirar,
    },
    GameCommand {
        name: "mazo",
        description: "mazo, para irse al mazo",
        run: mazo,
    },
    GameCommand {
        name: "envido",
        description: "envido {grado [real, falta]}, para cantar envido",
        run: envido,
    },
    GameCommand {
        name: "truco",
        description: "truco, canta el truco o eleva el canto a retruco/vale 4 segun corresponda",
        run: truco,
    },
    GameCommand {
        name: "start",
        description: "",
        run: start,
    },
];

fn usage(name: &String, game_logic: &mut GameLogic) -> String {
    let mut ret = String::new();
    for command in GAME_COMMANDS[..STATIC_COMMANDS]
        .iter()
        .chain(game_logic.available_game_commands(&name).into_iter())
    {
        ret.push_str(&format!("{}\n", command));
    }
    ret
}

fn help(name: String, commands: Vec<String>, game_logic: &mut GameLogic) -> GameCommandRet {
    if commands.len() < 2 {
        let s = usage(&name, game_logic);
        return vec![(Routing::Single(name), s)];
    }
    let mut ret = String::new();
    for command_name in commands[1..].iter() {
        if let Some(command) = GAME_COMMANDS[..STATIC_COMMANDS]
            .iter()
            .chain(game_logic.available_game_commands(&name).into_iter())
            .find(|command| &command.name == command_name)
        {
            ret.push_str(&format!("{}\n", command));
            continue;
        }
        ret.push_str(&format!("ERROR: command {command_name} is not found\n"));
    }
    vec![(Routing::Single(name), ret)]
}

fn seña(name: String, commands: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    if commands.len() < 2 {
        return vec![(Routing::Single(name), format!("ERROR: missing seña\n"))];
    }
    let seña = &commands[1].to_uppercase();
    let señas = ["1E", "1B", "7E", "7O", "3", "2", "1F"];
    let mut ret = format!("{name} hizo la seña del ");
    if let Some(seña) = señas.iter().find(|&&s| s == seña) {
        ret.push_str(&format!("{seña}\n"));
        return vec![(Routing::BroadCast, ret)];
    }
    if seña.to_lowercase().starts_with("e") {
        ret.push_str(&format!("envido\n"));
        return vec![(Routing::BroadCast, ret)];
    }
    vec![(
        Routing::Single(name),
        format!("ERROR: there is no seña for {seña}\n"),
    )]
}

fn mirar(name: String, commands: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    if commands.len() < 2 {
        return vec![(
            Routing::Single(name),
            format!("ERROR: missing who you want to look at\n"),
        )];
    }
    let persona = &commands[1];
    let mut ret = String::new();
    ret.push_str(&format!("{name} mira a {persona}\n"));
    vec![(Routing::BroadCast, ret)]
}

fn msg(name: String, commands: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name}: "));
    for txt in commands[1..].iter() {
        ret.push_str(&(txt.to_owned() + " "));
    }
    ret.push_str("\n");
    vec![(Routing::BroadCast, ret)]
}

fn tirar(name: String, commands: Vec<String>, game_logic: &mut GameLogic) -> GameCommandRet {
    if commands.len() < 2 {
        return vec![(
            Routing::Single(name),
            format!("ERROR: missing card index\n"),
        )];
    }
    let carta = commands[1].parse();
    if carta.is_err() {
        return vec![(
            Routing::Single(name),
            format!("ERROR: card index not a valid number\n"),
        )];
    }
    let carta: usize = carta.unwrap();
    if carta > 2 {
        return vec![(
            Routing::Single(name),
            format!("ERROR: card index not a valid number\n"),
        )];
    }
    match game_logic.tirar(carta, &name) {
        Ok(_) => {
            let mut ret = vec![(Routing::BroadCast, game_logic.try_fin_ronda())];
            ret.push((Routing::BroadCast, format!("{name} tiró {carta}\n")));
            ret.push((Routing::BroadCast, format!("{}\n", game_logic.mesa())));
            let pos_turno = game_logic.mesa().posicion_del_turno();
            for player in game_logic.players() {
                ret.push((
                    Routing::Single(player.name().to_string()),
                    format!("{}\n  0   1   2\n", player.player()),
                ));
                if player.player().posicion() == pos_turno {
                    ret.push((
                        Routing::Single(player.name().to_string()),
                        format!("Es tu turno\n"),
                    ));
                }
            }
            ret
        }
        Err(e) => vec![(Routing::Single(name), e)],
    }
}

fn mazo(name: String, _: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} se fue al mazo\n"));
    vec![(Routing::BroadCast, ret)]
}

fn envido(name: String, commands: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó "));
    if commands.len() >= 2 {
        let grado = &commands[1];
        if grado != "falta" && grado != "real" {
            return vec![(
                Routing::Single(name),
                format!("ERROR: {grado} envido doesn't exist\n"),
            )];
        }
        ret.push_str(&format!("{grado} "));
    }
    ret.push_str(&format!("envido\n"));
    vec![(Routing::BroadCast, ret)]
}

fn truco(name: String, _: Vec<String>, _: &mut GameLogic) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó truco\n"));
    vec![(Routing::BroadCast, ret)]
}

fn start(_: String, _: Vec<String>, game_logic: &mut GameLogic) -> GameCommandRet {
    let mut ret = Vec::new();
    ret.push((Routing::BroadCast, format!("{}\n", game_logic.mesa())));
    let pos_turno = game_logic.mesa().posicion_del_turno();
    for player in game_logic.players() {
        ret.push((
            Routing::Single(player.name().to_string()),
            format!("{}\n  0   1   2\n", player.player()),
        ));
        if player.player().posicion() == pos_turno {
            ret.push((
                Routing::Single(player.name().to_string()),
                format!("Es tu turno\n"),
            ));
        }
    }
    ret
}
