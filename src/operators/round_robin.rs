use crate::data::{Game, Ranking};
use crate::operators::Operator;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundRobinOp {}

impl Operator for RoundRobinOp {
    fn init(&mut self) {
        todo!()
    }

    fn set_input(&mut self, name: &str, value: Ranking) -> Result<(), String> {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn get_output(&self, name: &str) -> Result<Ranking, String> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn get_games(&self) -> Vec<Game> {
        todo!()
    }
}
