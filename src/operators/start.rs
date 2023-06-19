use serde::{Deserialize, Serialize};

use crate::data::{Game, Ranking};
use crate::operators::Operator;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StartOp {
    seeding: Option<Ranking>,
    output: Option<Ranking>,
}

impl StartOp {
    pub fn set_seeding(&mut self, r: Ranking) {
        self.seeding = Some(r);
    }

    pub fn new() -> Self {
        Default::default()
    }
}

impl Operator for StartOp {
    fn init(&mut self) {}

    fn set_input(&mut self, _name: &str, _value: Ranking) -> Result<(), String> {
        Err("Start does not accept inputs".to_string())
    }

    fn update(&mut self) {
        self.output = self.seeding.clone();
    }

    fn get_output(&mut self, name: &str) -> Result<Ranking, String> {
        if name == "Output" {
            self.output.ok_or("No seeding set".to_string())
        } else {
            Err(format!("{name} is not an output"))
        }
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
