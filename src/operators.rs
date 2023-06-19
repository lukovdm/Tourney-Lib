use crate::data::{Game, Ranking};
use crate::operators::end::EndOp;
use crate::operators::round_robin::RoundRobinOp;
use crate::operators::splitter::SplitOp;
use crate::operators::start::StartOp;
use serde::{Deserialize, Serialize};

pub mod end;
pub mod round_robin;
pub mod splitter;
pub mod start;

#[derive(Deserialize, Serialize, Debug)]
pub enum OperatorType {
    Start(StartOp),
    End(EndOp),
    Split(SplitOp),
    RoundRobin(RoundRobinOp),
}

pub trait Operator {
    fn init(&mut self);

    fn set_input(&mut self, name: &str, value: Ranking) -> Result<(), String>;

    fn update(&mut self);

    fn get_output(&mut self, name: &str) -> Result<Ranking, String>;

    fn get_games(&self) -> Vec<Game>;
}
