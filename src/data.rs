#[derive(Debug, Clone)]
pub struct Team {
    pub(crate) id: u32,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    team1: Team,
    team2: Team,
    score1: u8,
    score2: u8,
}

pub type Ranking = Vec<Team>;
