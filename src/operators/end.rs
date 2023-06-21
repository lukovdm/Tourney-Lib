use serde::{Deserialize, Serialize};

use crate::data::{Game, Ranking};
use crate::operators::Operator;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EndOp {
    ranking: Option<Ranking>,
    input: Option<Ranking>,
}

impl EndOp {
    pub fn ranking(&self) -> Result<Ranking, String> {
        self.ranking
            .clone()
            .ok_or("No end ranking found".to_string())
    }

    pub fn new() -> Self {
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

    fn get_output(&self, _name: &str) -> Result<Ranking, String> {
        Err("End has not outputs".to_string())
    }

    fn reset(&mut self) {
        self.input = None;
        self.ranking = None;
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
