use std::fmt::Display;

use crate::tables::Routing;

pub type GameCommandRet = (Routing, String);
pub struct GameCommand {
    name: &'static str,
    description: &'static str,
    run: fn(String, Vec<String>) -> GameCommandRet,
}

impl GameCommand {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run(&self, name: String, commands: Vec<String>) -> GameCommandRet {
        (self.run)(name, commands)
    }
}

impl Display for GameCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6} - {}", self.name, self.description)
    }
}

pub const GAME_COMMANDS: &[GameCommand] = &[
    GameCommand {
       name: "help",
        description:
            "help {command0} {command1} .. gives the description of a command, otherwise describes all commands",
        run: help,
    },
    GameCommand {
        name: "tirar",
        description: "tirar <carta a tirar[1, 2, 3]>, tira la carta elegida",
        run: tirar,
    },
    GameCommand {
        name: "mazo",
        description: "mazo, para irse al mazo",
        run: mazo,
    },
    GameCommand {
        name: "embido",
        description: "embido, para cantar embido",
        run: embido,
    },
    GameCommand {
        name: "real_embido",
        description: "real_embido, para cantar real_embido",
        run: real_embido,
    },
    GameCommand {
        name: "falta_embido",
        description: "embido, para cantar falta_embido",
        run: falta_embido,
    },
    GameCommand {
        name: "truco",
        description: "truco, canta el truco o eleva el canto a retruco/vale 4 segun corresponda",
        run: truco,
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
];

fn usage() -> String {
    let mut ret = String::new();
    for command in GAME_COMMANDS {
        ret.push_str(&format!("{}\n", command));
    }
    ret
}

fn help(name: String, commands: Vec<String>) -> GameCommandRet {
    if commands.len() < 2 {
        return (Routing::Single(name), usage());
    }
    let mut ret = String::new();
    for command_name in commands[1..].iter() {
        if let Some(command) = GAME_COMMANDS
            .iter()
            .find(|command| &command.name == command_name)
        {
            ret.push_str(&format!("{}\n", command));
            continue;
        }
        ret.push_str(&format!("ERROR: command {command_name} is not found\n"));
    }
    (Routing::Single(name), ret)
}

fn tirar(name: String, commands: Vec<String>) -> GameCommandRet {
    if commands.len() < 2 {
        return (Routing::Single(name), format!("ERROR: missing card index\n"));
    }
    let carta = commands[1].parse();
    if carta.is_err() {
        return (Routing::Single(name), format!("ERROR: card index not a valid number\n"));
    }
    let carta: usize = carta.unwrap();
    if carta > 2 {
        return (Routing::Single(name), format!("ERROR: card index not a valid number\n"));
    }
    let mut ret = String::new();
    ret.push_str(&format!("{name} tiró {carta}\n"));
    (Routing::BroadCast, ret)
}

fn mazo(name: String, _: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} se fue al mazo\n"));
    (Routing::BroadCast, ret)
}

fn embido(name: String, _: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó embido\n"));
    (Routing::BroadCast, ret)
}

fn real_embido(name: String, _: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó real_embido\n"));
    (Routing::BroadCast, ret)
}

fn falta_embido(name: String, _: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó falta_embido\n"));
    (Routing::BroadCast, ret)
}

fn truco(name: String, _: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name} cantó truco\n"));
    (Routing::BroadCast, ret)
}

fn seña(name: String, commands: Vec<String>) -> GameCommandRet {
    if commands.len() < 2 {
        return (Routing::Single(name), format!("ERROR: missing seña\n"));
    }
    let seña = &commands[1];
    let mut ret = String::new();
    ret.push_str(&format!("{name} hizo la seña del {seña}\n"));
    (Routing::BroadCast, ret)
}

fn mirar(name: String, commands: Vec<String>) -> GameCommandRet {
    if commands.len() < 2 {
        return (Routing::Single(name), format!("ERROR: missing how you want to look at\n"));
    }
    let persona = &commands[1];
    let mut ret = String::new();
    ret.push_str(&format!("{name} mira a {persona}\n"));
    (Routing::BroadCast, ret)
}

fn msg(name: String, commands: Vec<String>) -> GameCommandRet {
    let mut ret = String::new();
    ret.push_str(&format!("{name}: "));
    for txt in commands[1..].iter() {
        ret.push_str(&(txt.to_owned()+" "));
    }
    ret.push_str("\n");
    (Routing::BroadCast, ret)
}

