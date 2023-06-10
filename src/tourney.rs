use crate::data::Ranking;
use crate::operators::end::EndOp;
use crate::operators::start::StartOp;
use crate::operators::OperatorType::{End, Start};
use crate::operators::{Operator, OperatorType};
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use std::collections::HashMap;

pub struct Node {
    id: u32,
    pub(crate) op: OperatorType,
}

pub struct Edge {
    size: u32,
    from: String,
    to: String,
}

pub struct Tourney {
    pub(crate) tourney: StableDiGraph<Node, Edge>,
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
            },
        )
    }

    pub fn update(&mut self) {
        for s in &self.starts {}
    }

    fn update_idx(&mut self, idx: NodeIndex, input: HashMap<String, Ranking>) {
        match self.tourney[*s].op {
            Start(ref mut op) => {
                let res = op.update(HashMap::new()).expect("TODO: panic message");
            }
            End(_) => {}
            OperatorType::Split(_) => {}
            OperatorType::RoundRobin(_) => {}
        }
    }
}
