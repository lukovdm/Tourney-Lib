use crate::data::{Game, Ranking};
use crate::operators::{Handle, Operator};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct EndOp {
    pub(crate) size: usize,
    pub(crate) ranking: Ranking,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EndInputs {
    Input,
}

impl Handle for EndInputs {
    fn to_handle(&self) -> String {
        "Input".to_string()
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EndOutputs {}

impl Handle for EndOutputs {
    fn to_handle(&self) -> String {
        "".to_string()
    }
}

impl EndOp {
    fn get_ranking(&mut self) -> &Ranking {
        &self.ranking
    }
}

impl Operator for EndOp {
    type Inputs = EndInputs;
    type Outputs = EndOutputs;

    fn init(&mut self) {}

    fn update(
        &mut self,
        mut inputs: HashMap<EndInputs, Ranking>,
    ) -> Result<HashMap<EndOutputs, Ranking>, String> {
        match inputs.entry(EndInputs::Input) {
            Entry::Occupied(ranking_entry) => {
                self.ranking = ranking_entry.remove();
                Ok(HashMap::new())
            }
            Entry::Vacant(_) => Err("Input missing".to_string()),
        }
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
