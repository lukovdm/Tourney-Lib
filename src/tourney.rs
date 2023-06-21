use petgraph::algo::toposort;
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::visit::{EdgeRef, IntoNodeReferences, NodeRef};
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
    pub(crate) from: String,
    pub(crate) to: String,
    #[serde(skip)]
    pub(crate) data: Option<Ranking>,
}

pub struct Tourney {
    pub(crate) tourney: StableDiGraph<Node, Edge, u32>,
    pub(crate) starts: Vec<NodeIndex>,
    pub(crate) ends: Vec<NodeIndex>,
}

impl Default for Tourney {
    fn default() -> Self {
        let mut graph = StableDiGraph::new();
        let start = graph.add_node(Node {
            id: 0,
            op: OperatorType::Start(StartOp::new()),
        });
        let end = graph.add_node(Node {
            id: 1,
            op: OperatorType::End(EndOp::new()),
        });
        Tourney {
            tourney: graph,
            starts: Vec::from([start]),
            ends: Vec::from([end]),
        }
    }
}

impl Tourney {
    pub fn new() -> Self {
        Default::default()
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
        from_handle: &str,
        to: NodeIndex,
        to_handle: &str,
    ) -> EdgeIndex {
        self.tourney.add_edge(
            from,
            to,
            Edge {
                from: from_handle.to_string(),
                to: to_handle.to_string(),
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
        let mut inputs = Vec::new();
        for e_i in self.tourney.edges_directed(idx, Direction::Incoming) {
            if let Some(data) = e_i.weight().data.clone() {
                let name = &e_i.weight().to;
                inputs.push((name.clone(), data.clone()));
            }
        }
        let node = self.tourney.node_weight_mut(idx).unwrap();
        for (name, data) in inputs {
            node.op.set_input(&*name, data).expect("Input not allowed");
        }

        node.op.update();

        let mut output_names = Vec::new();
        let mut outputs = Vec::new();
        for e_o in self.tourney.edges(idx) {
            let name = &e_o.weight().from;
            output_names.push((name.clone(), e_o.id()));
        }
        let node = self.tourney.node_weight(idx).unwrap();
        for (name, idx) in output_names {
            outputs.push((node.op.get_output(&*name).expect("Input not allowed"), idx));
        }
        for (value, idx) in outputs {
            self.tourney.edge_weight_mut(idx).unwrap().data = Some(value);
        }
    }

    pub fn reset(&mut self) {
        for node in self.tourney.node_indices().collect::<Vec<_>>() {
            self.tourney.node_weight_mut(node).unwrap().op.reset()
        }
    }
}
