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

    fn get_output(&self, name: &str) -> Result<Ranking, String>;

    fn reset(&mut self);

    fn get_games(&self) -> Vec<Game>;
}

impl Operator for OperatorType {
    fn init(&mut self) {
        match self {
            OperatorType::Start(o) => o.init(),
            OperatorType::End(o) => o.init(),
            OperatorType::Split(o) => o.init(),
            OperatorType::RoundRobin(o) => o.init(),
        }
    }

    fn set_input(&mut self, name: &str, value: Ranking) -> Result<(), String> {
        match self {
            OperatorType::Start(o) => o.set_input(name, value),
            OperatorType::End(o) => o.set_input(name, value),
            OperatorType::Split(o) => o.set_input(name, value),
            OperatorType::RoundRobin(o) => o.set_input(name, value),
        }
    }

    fn update(&mut self) {
        match self {
            OperatorType::Start(o) => o.update(),
            OperatorType::End(o) => o.update(),
            OperatorType::Split(o) => o.update(),
            OperatorType::RoundRobin(o) => o.update(),
        }
    }

    fn get_output(&self, name: &str) -> Result<Ranking, String> {
        match &self {
            OperatorType::Start(o) => o.get_output(name),
            OperatorType::End(o) => o.get_output(name),
            OperatorType::Split(o) => o.get_output(name),
            OperatorType::RoundRobin(o) => o.get_output(name),
        }
    }

    fn reset(&mut self) {
        match self {
            OperatorType::Start(o) => o.reset(),
            OperatorType::End(o) => o.reset(),
            OperatorType::Split(o) => o.reset(),
            OperatorType::RoundRobin(o) => o.reset(),
        }
    }

    fn get_games(&self) -> Vec<Game> {
        match &self {
            OperatorType::Start(o) => o.get_games(),
            OperatorType::End(o) => o.get_games(),
            OperatorType::Split(o) => o.get_games(),
            OperatorType::RoundRobin(o) => o.get_games(),
        }
    }
}
