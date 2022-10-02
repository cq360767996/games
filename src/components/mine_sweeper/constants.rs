#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Level {
  Easy,
  Medium,
  Hard,
}

#[derive(Clone, Debug)]
pub struct LevelData<'a> {
  pub value: Level,
  pub label: &'a str,
  pub rows: i32,
  pub cols: i32,
  pub mines: i32,
  pub countdown: i32,
}

pub const LEVELS: [LevelData; 3] = [
  LevelData {
    value: Level::Easy,
    label: "简单",
    rows: 9,
    cols: 9,
    mines: 10,
    countdown: 999,
  },
  LevelData {
    value: Level::Medium,
    label: "中等",
    rows: 16,
    cols: 16,
    mines: 40,
    countdown: 999,
  },
  LevelData {
    value: Level::Hard,
    label: "困难",
    rows: 30,
    cols: 16,
    mines: 99,
    countdown: 999,
  },
];
