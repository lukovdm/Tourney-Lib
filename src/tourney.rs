use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;

use petgraph::algo::toposort;
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::visit::{EdgeRef, IntoEdgesDirected, IntoNodeReferences, NodeRef};
use petgraph::Direction;
use serde::{Deserialize, Serialize};

use crate::data::Ranking;
use crate::operators::end::EndOp;
use crate::operators::start::StartOp;
use crate::operators::OperatorType::{End, Start};
use crate::operators::{Operator, OperatorType};

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    pub(crate) id: u32,
    pub(crate) op: OperatorType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Edge {
    pub(crate) size: u32,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) data: Option<Ranking>,
}

pub struct Tourney {
    pub(crate) tourney: StableDiGraph<Node, Edge, u32>,
    pub(crate) starts: Vec<NodeIndex>,
    pub(crate) ends: Vec<NodeIndex>,
}

impl Tourney {
    pub fn new(size: usize) -> Self {
        let mut graph = StableDiGraph::new();
        let start = graph.add_node(Node {
            id: 0,
            op: OperatorType::Start(StartOp {
                size,
                seeding: None,
            }),
        });
        let end = graph.add_node(Node {
            id: 1,
            op: OperatorType::End(EndOp {
                size,
                ranking: vec![],
            }),
        });
        Tourney {
            tourney: graph,
            starts: Vec::from([start]),
            ends: Vec::from([end]),
        }
    }

    pub fn from_graph(g: StableDiGraph<Node, Edge>) -> Self {
        let mut starts = vec![];
        let mut ends = vec![];
        for n in g.node_references() {
            match n.weight().op {
                Start(..) => starts.push(n.id()),
                End(..) => ends.push(n.id()),
                _ => (),
            }
        }
        Tourney {
            tourney: g,
            starts,
            ends,
        }
    }

    pub fn add_node(&mut self, op: Node) -> NodeIndex {
        let is_start = matches!(op.op, Start(..));
        let is_end = matches!(op.op, End(..));
        let node = self.tourney.add_node(op);
        if is_start {
            self.starts.push(node);
        }
        if is_end {
            self.ends.push(node);
        }
        return node;
    }

    pub fn add_edge(
        &mut self,
        from: NodeIndex,
        from_handle: String,
        to: NodeIndex,
        to_handle: String,
        size: u32,
    ) -> EdgeIndex {
        self.tourney.add_edge(
            from,
            to,
            Edge {
                size,
                from: from_handle,
                to: to_handle,
                data: None,
            },
        )
    }

    pub fn update(&mut self) {
        let nodes = toposort(&self.tourney, None).expect("Cycle detected");
        for node in nodes {
            self.update_idx(node);
        }
    }

    fn update_idx(&mut self, idx: NodeIndex) {
        let mut input = HashMap::new();
        let edge_idxs: Vec<EdgeIndex> = self
            .tourney
            .edges_directed(idx, Direction::Incoming)
            .map(|e| e.id())
            .collect();
        for edge_idx in edge_idxs {
            let mut weight = self.tourney.edge_weight(edge_idx).unwrap();
            if let Some(ranking) = &weight.data {
                input.insert(weight.to.clone(), ranking.clone());
            }
        }

        if let Some(node) = self.tourney.node_weight(idx) {
            match node.op {
                Start(_) => {
                    if let Start(ref mut op) = self.tourney.node_weight_mut(idx).unwrap().op {
                        let mut res = op.update(HashMap::new()).expect("TODO: panic message");
                    } else {
                        panic!("Should not happen")
                    }
                }
                End(ref op) => {
                    if let End(ref mut op) = self.tourney.node_weight_mut(idx).unwrap().op {
                        op.update(EndInputs::map_handle(input))
                            .expect("Update error");
                    } else {
                        panic!("Should not happen")
                    }
                }
                OperatorType::Split(ref op) => {}
                OperatorType::RoundRobin(_) => {}
            }
        }

        let edge_idxs: Vec<EdgeIndex> = self.tourney.edges(idx).map(|e| e.id()).collect();
        for edge_idx in edge_idxs {
            let mut weight = self.tourney.edge_weight_mut(edge_idx).unwrap();
            if let Occupied(x) =
                res.entry(StartOutputs::from_handle(&*weight.from).expect("Invalid from edge"))
            {
                weight.data = Some(x.remove())
            }
        }
    }
}
