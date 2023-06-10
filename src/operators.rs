use crate::data::{Game, Ranking, Team};
use crate::operators::end::EndOp;
use crate::operators::round_robin::RoundRobinOp;
use crate::operators::splitter::SplitOp;
use crate::operators::start::StartOp;
use std::collections::HashMap;

pub mod end;
pub mod round_robin;
pub mod splitter;
pub mod start;

pub enum OperatorType {
    Start(StartOp),
    End(EndOp),
    Split(SplitOp),
    RoundRobin(RoundRobinOp),
}

pub trait Operator {
    type Inputs;
    type Outputs;

    fn init(&mut self);

    fn update(
        &mut self,
        inputs: HashMap<Self::Inputs, Ranking>,
    ) -> Result<HashMap<Self::Outputs, Ranking>, String>;

    fn get_games(&self) -> Vec<Game>;
}

pub trait Handle {
    fn to_handle(&self) -> String;
}
