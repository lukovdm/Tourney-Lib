use serde::{Deserialize, Serialize};

use crate::data::{Game, Ranking};
use crate::operators::Operator;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SplitOp {
    at: usize,
    input: Option<Ranking>,
    output: Option<(Ranking, Ranking)>,
}

impl SplitOp {
    fn new(at: usize) -> Self {
        SplitOp {
            at,
            ..Default::default()
        }
    }
}

impl Operator for SplitOp {
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
        self.output = self.input.as_mut().map(|x| {
            let y = x.split_off(self.at);
            (x.clone(), y)
        });
    }

    fn get_output(&self, name: &str) -> Result<Ranking, String> {
        match (name, &self.output) {
            ("Top", Some((t, _b))) => Ok(t.clone()),
            ("Bottom", Some((_t, b))) => Ok(b.clone()),
            (_, None) => Err("Output not ready".to_string()),
            _ => Err(format!("{name} is not an output")),
        }
    }

    fn reset(&mut self) {
        self.input = None;
        self.output = None;
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
