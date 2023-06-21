use serde::{Deserialize, Serialize};

use crate::data::{Game, Ranking};
use crate::operators::Operator;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StartOp {
    #[serde(skip)]
    seeding: Option<Ranking>,
    #[serde(skip)]
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

    fn get_output(&self, name: &str) -> Result<Ranking, String> {
        if name == "Output" {
            self.output.clone().ok_or("No seeding set".to_string())
        } else {
            Err(format!("{name} is not an output"))
        }
    }

    fn reset(&mut self) {
        self.seeding = None;
        self.output = None;
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
