use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Team {
    pub(crate) id: u32,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Game {
    id: u32,
    team1: Team,
    team2: Team,
    score1: u8,
    score2: u8,
}

pub type Ranking = Vec<Team>;
