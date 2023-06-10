use crate::data::{Game, Ranking, Team};
use crate::operators::Operator;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct SplitOp {
    at: usize,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SplitInputs {
    ToSplit,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SplitOutputs {
    Top,
    Bottom,
}

impl Operator for SplitOp {
    type Inputs = SplitInputs;
    type Outputs = SplitOutputs;

    fn init(&mut self) {}

    fn update(
        &mut self,
        mut inputs: HashMap<SplitInputs, Ranking>,
    ) -> Result<HashMap<SplitOutputs, Ranking>, String> {
        match inputs.entry(SplitInputs::ToSplit) {
            Entry::Occupied(ranking_entry) => {
                let ranking = ranking_entry.remove();
                let (top, bottom) = ranking.split_at(self.at);
                let mut output = HashMap::new();
                output.insert(SplitOutputs::Top, Vec::from(top));
                output.insert(SplitOutputs::Bottom, Vec::from(bottom));
                return Ok(output);
            }
            Entry::Vacant(..) => Err("Input not found".to_string()),
        }
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
