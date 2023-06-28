use super::{Decider, Decision};
use crate::motor::jugador::Avatar;
use crate::motor::mesa::Mesa;
use itertools::Itertools;
use r3bl_rs_utils::{Arena, Node};
use std::collections::VecDeque;
use std::ops::Range;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MinimaxDecider {
    desicion_tree: Arena<DesicionNode>,
    aux: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct DesicionNode {
    desicion: BayesianDecision,
    beneficio_esperado: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum BayesianDecision {
    Propia(Decision),
    Rival(AbtractDecision),
    Inicio,
    Final(f32, AbtractDecision),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum AbtractDecision {
    Matar(f32),
    Pardar(f32),
    Pasar(f32),
}

impl AbtractDecision {
    fn probabilidad(&self) -> f32 {
        match *self {
            AbtractDecision::Matar(ret) => ret,
            AbtractDecision::Pardar(ret) => ret,
            AbtractDecision::Pasar(ret) => ret,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Primera {
    Tengo,
    Pardas,
    Tiene,
}

impl Decider for MinimaxDecider {
    fn decide(&mut self, jugador: &Avatar, mesa: &Mesa) -> Decision {
        if mesa.ronda_en_juego == 0 {
            self.aux = if jugador.posicion == mesa.posicion_de_mano {
                true
            } else {
                false
            };
        }
        if self.aux {
            let le_quedan = cuantas_le_quedan(jugador.posicion, mesa);
            let este_nodo = self.este_nodo(jugador, mesa);
            self.actualizar_probabilidades(*jugador, mesa.clone(), este_nodo, le_quedan);
            self.actualizar_valores_esperados(este_nodo);
            return self.pick(jugador, este_nodo);
        }
        for i in 0..3 {
            if jugador.mano[i].is_some() {
                return Decision::Tirar(i);
            }
        }
        Decision::Mazo
    }
}

const VALORES_LOOKUP: [u8; 15] = [0, 4, 4, 4, 2, 4, 4, 4, 2, 4, 4, 1, 1, 1, 1];

fn combinations(n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    (n - r + 1..=n).product::<usize>() / (1..=r).product::<usize>()
}

fn probabilidad(dado: &[u8], le_quedan: usize, me_importan: Range<usize>) -> f32 {
    let mut conocidas = vec![0; 15];
    for &i in dado {
        conocidas[i as usize] += 1;
    }
    let desconocidas: Vec<_> = VALORES_LOOKUP
        .iter()
        .zip(conocidas.iter())
        .map(|(&x, &y)| x - y)
        .collect();
    let me_importan = desconocidas[me_importan].iter().fold(0, |acc, x| acc + x) as usize;
    let total = 40 - dado.len();
    1.0 - (combinations(total - me_importan, le_quedan) as f32
        / combinations(total, le_quedan) as f32)
}

#[test]
fn combinations_test() {
    let comb = combinations(36, 3);
    assert_eq!(comb, 7140);
}

#[test]
fn prob_ancho() {
    let prob = probabilidad(&[13,1,1], 3, 13..15) * 100000.0;
    assert_eq!(prob.trunc(), (3.0 * 100000.0 / 37.0_f32).trunc());
}

fn probabilidad_que_me_gane(a: u8, dado: &[u8], le_quedan: usize) -> f32 {
    probabilidad(dado, le_quedan, (a + 1) as usize..15)
}

fn probabilidad_que_me_emparde(a: u8, dado: &[u8], le_quedan: usize) -> f32 {
    probabilidad(dado, le_quedan, a as usize..(a + 1) as usize)
}

fn probabilidad_que_pierda(a: u8, dado: &[u8], le_quedan: usize) -> f32 {
    probabilidad(dado, le_quedan, 0..a as usize)
}

fn tirar(avatar: &mut Avatar, carta: usize, mesa: &mut Mesa) {
    let index = match mesa.cartas[avatar.posicion]
        .iter()
        .position(|c| c.is_none())
    {
        Some(i) => i,
        None => return,
    };
    mesa.cartas[avatar.posicion][index] = avatar.mano[carta].take();
}

fn cuantas_le_quedan(posicion: usize, mesa: &Mesa) -> usize {
    mesa.cartas
        .iter()
        .enumerate()
        .filter(|(p, _)| *p == posicion+1%mesa.numero_de_jugadores)
        .map(|(_, c)| c)
        .flatten()
        .flatten()
        .fold(3, |acc, _| acc - 1)

}

#[allow(dead_code)]
impl MinimaxDecider {
    pub fn new() -> MinimaxDecider {
        let mut desicion_tree = Arena::<DesicionNode>::new();

        let root = desicion_tree.add_new_node(
            DesicionNode {
                desicion: BayesianDecision::Inicio,
                beneficio_esperado: 0.0,
            },
            None,
        );

        llenar_mano(
            &mut desicion_tree,
            &[0, 1, 2],
            root,
            0,
            true,
            Primera::Pardas,
        );

        MinimaxDecider {
            desicion_tree,
            aux: false,
        }
    }

    fn actualizar_probabilidades(
        &self,
        avatar: Avatar,
        mesa: Mesa,
        parent_id: usize,
        le_quedan: usize,
    ) {
        if let Some(childs) = self.desicion_tree.get_children_of(parent_id) {
            for child in childs {
                let mut le_quedan = le_quedan;
                let mut avatar = avatar;
                let mut mesa = mesa.clone();
                if let Some(node_arc) = self.desicion_tree.get_node_arc(child) {
                    let mut node = node_arc.write().unwrap();
                    match node.payload.desicion {
                        BayesianDecision::Propia(desicion) => match desicion {
                            Decision::Tirar(carta) => {
                                tirar(&mut avatar, carta, &mut mesa);
                            }
                            Decision::Mazo => {}
                        },
                        BayesianDecision::Rival(desicion) => {
                            if let Some(propia_previa) =
                                mesa.cartas[avatar.posicion][mesa.ronda_en_juego]
                            {
                                let propia_previa = propia_previa.valor_juego;
                                let mut cartas_vistas = Vec::new();
                                for carta in avatar.mano {
                                    if let Some(carta) = carta {
                                        cartas_vistas.push(carta.valor_juego);
                                    }
                                }
                                for mano in &mesa.cartas {
                                    for carta in mano {
                                        if let Some(carta) = carta {
                                            cartas_vistas.push(carta.valor_juego);
                                        }
                                    }
                                }
                                match desicion {
                                    AbtractDecision::Matar(_) => {
                                        let new_prob = probabilidad_que_me_gane(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Rival(
                                            AbtractDecision::Matar(new_prob),
                                        );
                                    }
                                    AbtractDecision::Pardar(_) => {
                                        let new_prob = probabilidad_que_me_emparde(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Rival(
                                            AbtractDecision::Pardar(new_prob),
                                        );
                                    }
                                    AbtractDecision::Pasar(_) => {
                                        let new_prob = probabilidad_que_pierda(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Rival(
                                            AbtractDecision::Pasar(new_prob),
                                        );
                                    }
                                }
                                le_quedan -= 1;
                            }
                        }
                        BayesianDecision::Final(beneficio_esperado, tipo_de_final) => {
                            if let Some(propia_previa) =
                                mesa.cartas[avatar.posicion][mesa.ronda_en_juego]
                            {
                                let propia_previa = propia_previa.valor_juego;
                                let mut cartas_vistas = Vec::new();
                                for carta in avatar.mano {
                                    if let Some(carta) = carta {
                                        cartas_vistas.push(carta.valor_juego);
                                    }
                                }
                                for mano in &mesa.cartas {
                                    for carta in mano {
                                        if let Some(carta) = carta {
                                            cartas_vistas.push(carta.valor_juego);
                                        }
                                    }
                                }
                                match tipo_de_final {
                                    AbtractDecision::Matar(_) => {
                                        let new_prob = probabilidad_que_me_gane(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Final(
                                            beneficio_esperado,
                                            AbtractDecision::Matar(new_prob),
                                        );
                                    }
                                    AbtractDecision::Pardar(_) => {
                                        let new_prob = probabilidad_que_me_emparde(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Final(
                                            beneficio_esperado,
                                            AbtractDecision::Pardar(new_prob),
                                        );
                                    }
                                    AbtractDecision::Pasar(_) => {
                                        let new_prob = probabilidad_que_pierda(
                                            propia_previa,
                                            &cartas_vistas,
                                            le_quedan,
                                        );
                                        node.payload.desicion = BayesianDecision::Final(
                                            beneficio_esperado,
                                            AbtractDecision::Pasar(new_prob),
                                        );
                                    }
                                }
                            }
                        }
                        BayesianDecision::Inicio => {}
                    }
                }
                self.actualizar_probabilidades(avatar, mesa, child, le_quedan);
            }
        }
    }

    fn actualizar_valores_esperados(&self, parent_id: usize) -> f32 {
        if let Some(childs) = self.desicion_tree.get_children_of(parent_id) {
            let mut beneficio_esperado = Vec::new();
            for child in childs {
                if let Some(node_arc) = self.desicion_tree.get_node_arc(child) {
                    let nuevo_valor_esperado = self.actualizar_valores_esperados(child);
                    let mut node = node_arc.write().unwrap();
                    match node.payload.desicion {
                        BayesianDecision::Propia(_) => {
                            node.payload.beneficio_esperado = nuevo_valor_esperado;
                            beneficio_esperado.push((2.0, node.payload.beneficio_esperado));
                        }
                        BayesianDecision::Rival(desicion) => {
                            node.payload.beneficio_esperado = nuevo_valor_esperado;
                            beneficio_esperado
                                .push((desicion.probabilidad(), node.payload.beneficio_esperado));
                        }
                        BayesianDecision::Final(beneficio, desicion) => {
                            beneficio_esperado.push((desicion.probabilidad(), beneficio));
                        }
                        BayesianDecision::Inicio => {}
                    }
                }
            }
            let prob_propia = beneficio_esperado
                .iter()
                .filter(|(p, _)| *p <= 1.0)
                .fold(1.0, |acc, (x, _)| acc - x);
            let (last_p, last_b) = beneficio_esperado.last().unwrap();
            return beneficio_esperado
                .iter()
                .map(|&(p, b)| if p > 1.0 { (prob_propia, b) } else { (p, b) })
                .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .scan(1.0, |acc, (p, b)| {
                    let scaled_p = if (p,b)!=(*last_p, *last_b) {
                            *acc * p
                        }else{
                            *acc
                        };
                    *acc -= scaled_p;
                    Some((scaled_p, b))
                })
                .fold(0.0, |acc, (p, b)| acc + (p * b as f32));
        }
        return 0.0;
    }

    fn este_nodo(&self, _avatar: &Avatar, _mesa: &Mesa) -> usize {
        0
    }

    fn pick(&self, jugador: &Avatar, _este_nodo: usize) -> Decision {
        for i in 0..3 {
            if jugador.mano[i].is_some() {
                return Decision::Tirar(i);
            }
        }
        Decision::Mazo
    }

    /*fn minimax(&self, parent_id: usize, alpha: f32, beta: f32, maximizingPlayer: bool) {
       if game_over {
          return evaluation;
       }
       let childs = self.desicion_tree.get_children_of(parent_id);
       if maximizingPlayer {
         let max_eval = f32::NEG_INFINITY;
         for child in childs {
            let eval = self.minimax(child, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha{
               break;
             }
         }
          return max_eval
       } else {
          let min_eval = f32::INFINITY;
          for child in childs {
            let eval = self.minimax(child, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
               break;
            }
          }
          return min_eval
       }
    }*/
}

fn known_cards(avatar: &Avatar, mesa: &Mesa) -> Vec<u8>{ 
    let mut ret = Vec::new();
    for carta in avatar.mano {
        if let Some(carta) = carta {
            ret.push(carta.valor_juego);
        }
    }
    for mano in &mesa.cartas {
        for carta in mano {
            if let Some(carta) = carta {
                ret.push(carta.valor_juego);
            }
        }
    }
    ret
}

trait ExpectedValue {
    fn update_expected_value(&mut self, desicion_tree: &Arena<DesicionNode>, avatar: Avatar, mesa: Mesa);
}

impl ExpectedValue for Node<DesicionNode> {
    fn update_expected_value(&mut self, desicion_tree: &Arena<DesicionNode>, avatar: Avatar, mesa: Mesa) {
        let mut evaluations = Vec::new();
        for &child in &self.children_ids {
            let node = desicion_tree.get_node_arc(child).unwrap();
            let mut node = node.write().unwrap();
            node.update_expected_value(desicion_tree, avatar, mesa.clone());
            evaluations.push(node.payload);
        }
        let minimizing = match self.payload.desicion {
            BayesianDecision::Rival(_) => true,
            _ => false,
        };
        let knowns = known_cards(&avatar, &mesa);
        evaluations
            .iter()
            .sorted_by(|a,b| {
                let (a,b) = if minimizing {
                    (a.beneficio_esperado, b.beneficio_esperado)
                } else {
                    (b.beneficio_esperado, a.beneficio_esperado)
                };
                a.partial_cmp(&b).unwrap()
            });
    }
}

fn llenar_mano(
    desicion_tree: &mut Arena<DesicionNode>,
    disponibles: &[usize],
    parent_id: usize,
    ronda: i8,
    soy_mano: bool,
    primera: Primera,
) {
    let ronda = ronda + 1;
    for carta in disponibles {
        let propia = desicion_tree.add_new_node(
            DesicionNode {
                desicion: BayesianDecision::Propia(Decision::Tirar(*carta)),
                beneficio_esperado: -100.0,
            },
            Some(parent_id),
        );
        let nuevas_disp: Vec<usize> = disponibles
            .iter()
            .filter(|&n| n != carta)
            .map(|r| *r)
            .collect();
        if soy_mano {
            let mata = desicion_tree.add_new_node(
                DesicionNode {
                    desicion: BayesianDecision::Rival(AbtractDecision::Matar(1.0)),
                    beneficio_esperado: 100.0,
                },
                Some(propia),
            );
            let parda = desicion_tree.add_new_node(
                DesicionNode {
                    desicion: BayesianDecision::Rival(AbtractDecision::Pardar(1.0)),
                    beneficio_esperado: 100.0,
                },
                Some(propia),
            );
            let pierde = desicion_tree.add_new_node(
                DesicionNode {
                    desicion: BayesianDecision::Rival(AbtractDecision::Pasar(1.0)),
                    beneficio_esperado: 100.0,
                },
                Some(propia),
            );
            match ronda {
                1 => {
                    llenar_mano(
                        desicion_tree,
                        &nuevas_disp,
                        mata,
                        ronda,
                        false,
                        Primera::Tiene,
                    );
                    llenar_mano(
                        desicion_tree,
                        &nuevas_disp,
                        parda,
                        ronda,
                        true,
                        Primera::Pardas,
                    );
                    llenar_mano(
                        desicion_tree,
                        &nuevas_disp,
                        pierde,
                        ronda,
                        true,
                        Primera::Tengo,
                    );
                }
                2 => {
                    match primera {
                        Primera::Tengo => {
                            llenar_mano(desicion_tree, &nuevas_disp, mata, ronda, false, primera);
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        1.0,
                                        AbtractDecision::Pardar(1.0),
                                    ),
                                    beneficio_esperado: 1.0,
                                },
                                Some(parda),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        1.0,
                                        AbtractDecision::Pasar(1.0),
                                    ),
                                    beneficio_esperado: 1.0,
                                },
                                Some(pierde),
                            );
                        }
                        Primera::Pardas => {
                            llenar_mano(desicion_tree, &nuevas_disp, parda, ronda, true, primera);
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        -1.0,
                                        AbtractDecision::Matar(1.0),
                                    ),
                                    beneficio_esperado: -1.0,
                                },
                                Some(mata),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        1.0,
                                        AbtractDecision::Pasar(1.0),
                                    ),
                                    beneficio_esperado: 1.0,
                                },
                                Some(pierde),
                            );
                        }
                        Primera::Tiene => {}
                    };
                }
                3 => {
                    match primera {
                        Primera::Tengo => {}
                        Primera::Pardas => {
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        -1.0,
                                        AbtractDecision::Matar(1.0),
                                    ),
                                    beneficio_esperado: -1.0,
                                },
                                Some(mata),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        0.0,
                                        AbtractDecision::Pardar(1.0),
                                    ),
                                    beneficio_esperado: 0.0,
                                },
                                Some(parda),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        1.0,
                                        AbtractDecision::Pasar(1.0),
                                    ),
                                    beneficio_esperado: 1.0,
                                },
                                Some(pierde),
                            );
                        }
                        Primera::Tiene => {
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        -1.0,
                                        AbtractDecision::Matar(1.0),
                                    ),
                                    beneficio_esperado: -1.0,
                                },
                                Some(mata),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        -1.0,
                                        AbtractDecision::Pardar(1.0),
                                    ),
                                    beneficio_esperado: -1.0,
                                },
                                Some(parda),
                            );
                            desicion_tree.add_new_node(
                                DesicionNode {
                                    desicion: BayesianDecision::Final(
                                        1.0,
                                        AbtractDecision::Pasar(1.0),
                                    ),
                                    beneficio_esperado: 1.0,
                                },
                                Some(pierde),
                            );
                        }
                    };
                }
                _ => {}
            };
            continue;
        }
        desicion_tree.add_new_node(
            //Pierdo
            DesicionNode {
                desicion: BayesianDecision::Final(-1.0, AbtractDecision::Matar(1.0)),
                beneficio_esperado: -1.0,
            },
            Some(propia),
        );
        let pardo_esperado = match ronda {
            2 => -1.0,
            3 => 1.0,
            _ => 0.0,
        };
        desicion_tree.add_new_node(
            //Pardo
            DesicionNode {
                desicion: BayesianDecision::Final(pardo_esperado, AbtractDecision::Pardar(1.0)),
                beneficio_esperado: pardo_esperado,
            },
            Some(propia),
        );
        if ronda == 2 {
            llenar_mano(desicion_tree, &nuevas_disp, propia, ronda, true, primera);
            continue;
        }
        desicion_tree.add_new_node(
            //Gano
            DesicionNode {
                desicion: BayesianDecision::Final(1.0, AbtractDecision::Pasar(1.0)),
                beneficio_esperado: 1.0,
            },
            Some(propia),
        );
    }
}

#[allow(dead_code)]
fn pretty_print_tree(root: usize, desicion_tree: &Arena<DesicionNode>) {
    let mut queue = VecDeque::new();
    queue.push_back(Some(root));

    let mut offset = 40;
    let mut depth = 0;
    while !queue.is_empty() {
        depth += 1;
        if offset > 0 {
            offset = (offset - 1) / 3;
        }
        let level_size = queue.len();
        let mut nodes_on_level = Vec::new();
        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            nodes_on_level.push(node);
            let node = match node {
                Some(n) => n,
                None => {
                    if depth < 4 {
                        for _ in 0..3 {
                            queue.push_back(None);
                        }
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
                BayesianDecision::Final(_, _) => {
                    continue;
                }
                _ => {}
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
                None => " ",
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
