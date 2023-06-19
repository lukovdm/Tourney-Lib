use serde::{Deserialize, Serialize};

use crate::data::{Game, Ranking};
use crate::operators::Operator;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EndOp {
    ranking: Option<Ranking>,
    input: Option<Ranking>,
}

impl EndOp {
    fn ranking(&mut self) -> Result<Ranking, String> {
        self.ranking.ok_or("No end ranking found".to_string())
    }

    fn new() -> Self {
        Default::default()
    }
}

impl Operator for EndOp {
    fn init(&mut self) {}

    fn set_input(&mut self, name: &str, value: Ranking) -> Result<(), String> {
        if name == "Input" {
            self.input = Some(value);
            Ok(())
        } else {
            Err(format!("{name} is not an input"))
        }
    }

    fn update(&mut self) {
        self.ranking = self.input.take()
    }

    fn get_output(&mut self, _name: &str) -> Result<Ranking, String> {
        Err("End has not outputs".to_string())
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
