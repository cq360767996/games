#[derive(Clone, Debug, PartialEq)]
pub enum MineValue {
  Some(i32),
  Mine(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mine {
  pub value: MineValue,
  pub is_open: bool,
  pub flag: bool,
  pub id: String,
}

pub enum GameState {
  Gamimg,
  Lose,
  Win,
}
