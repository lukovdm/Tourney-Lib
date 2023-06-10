use crate::data::{Game, Ranking};
use crate::operators::{Handle, Operator};
use std::collections::HashMap;

pub struct StartOp {
    pub(crate) size: usize,
    pub(crate) seeding: Option<Ranking>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum StartInputs {}

impl Handle for StartInputs {
    fn to_handle(&self) -> String {
        "".to_string()
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum StartOutputs {
    Output,
}

impl Handle for StartOutputs {
    fn to_handle(&self) -> String {
        "Output".to_string()
    }
}

impl StartOp {
    pub fn init_seeding(&mut self, r: Ranking) {
        self.seeding = Some(r);
    }
}

impl Operator for StartOp {
    type Inputs = StartInputs;
    type Outputs = StartOutputs;

    fn init(&mut self) {}

    fn update(
        &mut self,
        mut inputs: HashMap<StartInputs, Ranking>,
    ) -> Result<HashMap<StartOutputs, Ranking>, String> {
        match self.seeding {
            None => Ok(HashMap::new()),
            Some(ref r) => Ok(HashMap::from([(StartOutputs::Output, r.clone())])),
        }
    }

    fn get_games(&self) -> Vec<Game> {
        Vec::new()
    }
}
