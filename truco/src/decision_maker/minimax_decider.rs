use super::{Decider, Decision};
use crate::motor::jugador::Avatar;
use crate::motor::mesa::Mesa;
use r3bl_rs_utils::Arena;
use std::collections::VecDeque;
use std::ops::Range;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MinimaxDecider {
    desicion_tree: Arena::<DesicionNode>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct DesicionNode {
    desicion: BayesianDecision,
    benficio_esperado: i8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum BayesianDecision {
    Propia(Decision),
    Rival(AbtractDecision),
    Inicio,
    Final(i8, f32),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum AbtractDecision {
    Matar(f32),
    Pardar(f32),
    Pasar(f32),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Primera {
    Tengo,
    Pardas,
    Tiene,
}

impl Decider for MinimaxDecider {
    
    fn decide(&self, jugador: &Avatar, _: &Mesa) -> Decision{
        for i in 0..3 {
            if jugador.mano[i].is_some() {
                return Decision::Tirar(i);
            }
        }
        Decision::Mazo
    }

}

const VALORES_LOOKUP: [u8;15] = [ 0, 4, 4, 4, 2, 4, 4, 4, 2, 4, 4, 1, 1, 1, 1 ];

fn combinations(n: usize, r: usize) -> usize {
    (n - r + 1..=n).product::<usize>() / (1..=r).product::<usize>()
}

fn probabilidad(a: u8, dado:&[u8], le_quedan: usize, me_importan: Range<usize>) -> f32 {
    let mut conocidas = vec![0; 15]; 
    for &i in dado { conocidas[i as usize] += 1; }
    let desconocidas: Vec<_> = VALORES_LOOKUP.iter().zip(conocidas.iter()).map(|(&x, &y)| x - y).collect();
    let me_importan = desconocidas[me_importan].iter().fold(0, |acc, x| acc + x) as usize;
    let total = 40-dado.len()-le_quedan;
    1.0-(combinations(total-me_importan, le_quedan)/combinations(total, le_quedan)) as f32
}

fn probabilidad_que_me_gane(a: u8, dado:&[u8], le_quedan: usize) -> f32 {
    probabilidad(a, dado, le_quedan, (a+1) as usize..15)
}

fn probabilidad_que_me_emparde(a: u8, dado:&[u8], le_quedan: usize) -> f32 {
    probabilidad(a, dado, le_quedan, a as usize..(a+1) as usize)
}

fn probabilidad_que_pierda(a: u8, dado:&[u8], le_quedan: usize) -> f32 {
    probabilidad(a, dado, le_quedan, 0..a as usize)
}

fn tirar(avatar: &mut Avatar, carta: usize, mesa: &mut Mesa) {
    let index = match mesa.cartas[avatar.posicion].iter().position(|c| c.is_none()) {
        Some(i) => i,
        None => return,
    };
    mesa.cartas[avatar.posicion][index] = avatar.mano[carta].take();
}


#[allow(dead_code)]
impl MinimaxDecider {
    pub fn new() -> MinimaxDecider {
        let mut desicion_tree = Arena::<DesicionNode>::new();

        let root = desicion_tree.add_new_node(
            DesicionNode{
                desicion: BayesianDecision::Inicio,
                benficio_esperado: 0
            },
            None
        );

        llenar_mano(&mut desicion_tree, &[0,1,2], root, 0, true, Primera::Pardas);

        MinimaxDecider { desicion_tree }
    }

    fn actualizar_probabilidades(&self, avatar: Avatar, mesa: Mesa, parent_id: usize, le_quedan: usize){
        if let Some(childs) = self.desicion_tree.get_children_of(parent_id) {
            for child in childs {
                let mut le_quedan = le_quedan;
                let mut avatar = avatar;
                let mut mesa = mesa.clone();
                if let Some(node_arc) = self.desicion_tree.get_node_arc(child) {
                    let node = node_arc.read().unwrap();
                    match node.payload.desicion {
                        BayesianDecision::Propia(desicion) => {
                            match desicion {
                                Decision::Tirar(carta) => {
                                    tirar(&mut avatar, carta, &mut mesa);
                                },
                                Decision::Mazo => {},
                            }
                        },
                        BayesianDecision::Rival(desicion) => {
                            if let Some(propia_previa) = self.desicion_tree.get_node_arc(parent_id) {
                                let propia_previa = propia_previa.read().unwrap();
                                let propia_previa = match propia_previa.payload.desicion {
                                    BayesianDecision::Propia(desicion) => {
                                        match desicion {
                                            Decision::Tirar(n) => {avatar.mano[n]},
                                            _ => None,
                                        }
                                    },
                                    _ => None,
                                };
                                let mut cartas_vistas = Vec::new();
                                for carta in avatar.mano {
                                    if let Some (carta) = carta {
                                        cartas_vistas.push(carta.valor_juego);
                                    }
                                }
                                for mano in &mesa.cartas {
                                    for carta in mano {
                                        if let Some (carta) = carta {
                                            cartas_vistas.push(carta.valor_juego);
                                        }
                                    }
                                }

                                let propia_previa = match propia_previa {
                                    Some(carta) => carta.valor_juego,
                                    None => 0,
                                };
                                match desicion {
                                    AbtractDecision::Matar(_) => {
                                        let new_prob = probabilidad_que_me_gane(propia_previa, &cartas_vistas, le_quedan);
                                        node_arc.write().unwrap().payload.desicion = BayesianDecision::Rival(AbtractDecision::Matar(new_prob));
                                    },
                                    AbtractDecision::Pardar(_) => {
                                        let new_prob = probabilidad_que_me_emparde(propia_previa, &cartas_vistas, le_quedan);
                                        node_arc.write().unwrap().payload.desicion = BayesianDecision::Rival(AbtractDecision::Pardar(new_prob));
                                    },
                                    AbtractDecision::Pasar(_) => {
                                        let new_prob = probabilidad_que_pierda(propia_previa, &cartas_vistas, le_quedan);
                                        node_arc.write().unwrap().payload.desicion = BayesianDecision::Rival(AbtractDecision::Pasar(new_prob));
                                    },
                                }
                                le_quedan -= 1;
                            }
                        },
                        BayesianDecision::Final(_, probabilidad) => {},
                        BayesianDecision::Inicio => {},
                    }
                }
                self.actualizar_probabilidades(avatar, mesa, child, le_quedan);
            }
        }
    }
}

#[allow(dead_code)]
fn llenar_mano(desicion_tree: &mut Arena::<DesicionNode>, disponibles: &[usize], parent_id: usize, ronda: i8, soy_mano: bool, primera: Primera){
    let ronda = ronda + 1;
    for carta in disponibles {
        let propia = desicion_tree.add_new_node(
            DesicionNode{
                desicion: BayesianDecision::Propia(Decision::Tirar(*carta)),
                benficio_esperado: -100,
            },
            Some(parent_id)
        );
        let nuevas_disp: Vec<usize> = disponibles.iter().filter(|&n| n!=carta).map(|r| *r).collect();
        if soy_mano {
            let mata = desicion_tree.add_new_node(
                DesicionNode{
                    desicion: BayesianDecision::Rival(AbtractDecision::Matar(1.0)),
                    benficio_esperado: 100,
                },
                Some(propia)
            );
            let parda = desicion_tree.add_new_node(
                DesicionNode{
                    desicion: BayesianDecision::Rival(AbtractDecision::Pardar(1.0)),
                    benficio_esperado: 100,
                },
                Some(propia)
            );
            let pierde = desicion_tree.add_new_node(
                DesicionNode{
                    desicion: BayesianDecision::Rival(AbtractDecision::Pasar(1.0)),
                    benficio_esperado: 100,
                },
                Some(propia)
            );
            match ronda{
                1 => {
                    llenar_mano(desicion_tree, &nuevas_disp, mata, ronda, false, Primera::Tiene);
                    llenar_mano(desicion_tree, &nuevas_disp, parda, ronda, true, Primera::Pardas);
                    llenar_mano(desicion_tree, &nuevas_disp, pierde, ronda, true, Primera::Tengo);
                }
                2 => {
                    match primera {
                        Primera::Tengo => {
                            llenar_mano(desicion_tree, &nuevas_disp, mata, ronda, false, primera);
                            desicion_tree.add_new_node(
                                DesicionNode{
                                    desicion: BayesianDecision::Final(1, 1.0),
                                    benficio_esperado: 1,
                                },
                                Some(parda)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                    desicion: BayesianDecision::Final(1, 1.0),
                                    benficio_esperado: 1,
                                },
                                Some(pierde)
                            );
                        },
                        Primera::Pardas => {
                            llenar_mano(desicion_tree, &nuevas_disp, parda, ronda, true, primera);
                            desicion_tree.add_new_node(
                                DesicionNode{
                                        desicion: BayesianDecision::Final(-1, 1.0),
                                        benficio_esperado: -1,
                                },
                                Some(mata)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                    desicion: BayesianDecision::Final(1, 1.0),
                                    benficio_esperado: 1,
                                },
                                Some(pierde)
                            );
                        },
                        Primera::Tiene => {}
                    };
                },
                3 => {
                    match primera {
                        Primera::Tengo => {},
                        Primera::Pardas => {
                            desicion_tree.add_new_node(
                                DesicionNode{
                                        desicion: BayesianDecision::Final(-1, 1.0),
                                        benficio_esperado: -1,
                                },
                                Some(mata)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                        desicion: BayesianDecision::Final(0, 1.0),
                                        benficio_esperado: 0,
                                },
                                Some(parda)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                    desicion: BayesianDecision::Final(1, 1.0),
                                    benficio_esperado: 1,
                                },
                                Some(pierde)
                            );
                        },
                        Primera::Tiene => {
                            desicion_tree.add_new_node(
                                DesicionNode{
                                        desicion: BayesianDecision::Final(-1, 1.0),
                                        benficio_esperado: -1,
                                },
                                Some(mata)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                        desicion: BayesianDecision::Final(-1, 1.0),
                                        benficio_esperado: -1,
                                },
                                Some(parda)
                            );
                            desicion_tree.add_new_node(
                                DesicionNode{
                                    desicion: BayesianDecision::Final(1, 1.0),
                                    benficio_esperado: 1,
                                },
                                Some(pierde)
                            );
                        }
                    };
                },
                _ => {}
            };
            continue;
        }
        desicion_tree.add_new_node( //Pierdo
            DesicionNode{
                desicion: BayesianDecision::Final(-1, 1.0),
                benficio_esperado: -1,
            },
            Some(propia)
        );
        let pardo_esperado = match ronda {
            2 => -1,
            3 => 1,
            _ => 0
        };
        desicion_tree.add_new_node( //Pardo
            DesicionNode{
                desicion: BayesianDecision::Final(pardo_esperado, 1.0),
                benficio_esperado: pardo_esperado,
            },
            Some(propia)
        );
        if ronda == 2 { 
            llenar_mano(desicion_tree, &nuevas_disp, propia, ronda, true, primera);
            continue; 
        }
        desicion_tree.add_new_node( //Gano
            DesicionNode{
                desicion: BayesianDecision::Final(1, 1.0),
                benficio_esperado: 1,
            },
            Some(propia)
        );
    }
}

#[allow(dead_code)]
fn pretty_print_tree(root: usize, desicion_tree: &Arena::<DesicionNode>) {
    let mut queue = VecDeque::new();
    queue.push_back(Some(root));

    let mut offset = 40;
    let mut depth = 0;
    while !queue.is_empty() {
        depth += 1;
        if offset > 0 { offset = (offset-1)/3; }
        let level_size = queue.len();
        let mut nodes_on_level = Vec::new();
        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            nodes_on_level.push(node);
            let node = match node {
                Some(n) => n,
                None => { 
                    if depth <4 {
                        for _ in 0..3 { queue.push_back(None); }
                    }
                    continue;
                }
            };
            let node = desicion_tree.get_node_arc(node).unwrap();
            let node = node.read().unwrap();
            let mut child_cont = 0;
            for child in &node.children_ids {
                child_cont += 1;
                queue.push_back(Some(*child));
            }
            match node.payload.desicion {
                BayesianDecision::Final(_, _) => { continue; },
                _ => {},
            }
            for _ in child_cont..3 {
                queue.push_back(None);
            }
        }
        for node in nodes_on_level {
            let value = match node {
                Some(n) => {
                    let node = desicion_tree.get_node_arc(n).unwrap();
                    let node = node.read().unwrap();
                    let value = match node.payload.desicion {
                        BayesianDecision::Propia(_) => "P",
                        BayesianDecision::Rival(_) => "R",
                        BayesianDecision::Inicio => "I",
                        BayesianDecision::Final(_, _) => "F",
                    };
                    value
                }
                None => " "
            };
            print!("{}", " ".repeat(offset));
            print!("{}", value);
            print!("{}", " ".repeat(offset));
        }
        println!();
    }

}

#[test]
fn tree_creation() {
    let mini = MinimaxDecider::new();
    pretty_print_tree(1, &mini.desicion_tree);
}
