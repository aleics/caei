use indexmap::set::IndexSet;
use rand::Rng;
use std::fmt::{Display, Formatter};

const ROWS: usize = 4;
const COLUMNS: usize = 4;
const LENGTH: usize = ROWS * COLUMNS;

fn index(row: usize, col: usize) -> usize {
  row * ROWS + col
}

pub enum Move {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Debug, PartialEq)]
pub struct Board {
  values: [u64; LENGTH],
  free: IndexSet<usize>,
  pub score: u64,
}

impl Board {
  /// Creates a new board game with two random values
  pub fn new() -> Self {
    let mut board = Board::default();

    board.set_free_random(2);
    board.set_free_random(4);

    board
  }

  /// Initializes the game with already defined board `values`.
  pub fn with_values(values: [u64; LENGTH], score: u64) -> Self {
    let free = values
      .into_iter()
      .enumerate()
      .filter(|(_, value)| value == &0)
      .map(|(index, _)| index)
      .collect();

    Board {
      values,
      free,
      score,
    }
  }

  /// Returns the value of the board given a `row` and `column` position
  fn get(&self, row: usize, column: usize) -> u64 {
    self.values[index(row, column)]
  }

  /// Returns a copy of the board as a vector of rows
  pub fn as_rows(&self) -> Vec<Vec<u64>> {
    (0..LENGTH)
      .step_by(COLUMNS)
      .map(|i| self.values[i..i + ROWS].to_vec())
      .collect()
  }

  /// Sets a `value` in the board given a `row` and `column` position.
  fn set(&mut self, row: usize, column: usize, value: u64) {
    let index = index(row, column);
    self.set_index(index, value)
  }

  /// Sets a `value` in the board to a random free slot.
  fn set_free_random(&mut self, value: u64) {
    let max = self.free.len();
    if max == 0 {
      return;
    }

    let index = rand::thread_rng().gen_range(0..max);

    if let Some(free_index) = self.free.get_index(index) {
      self.set_index(*free_index, value);
    }
  }

  /// Sets a `value` in the board by a given `index`
  fn set_index(&mut self, index: usize, value: u64) {
    self.values[index] = value;

    if value == 0 {
      self.free.insert(index);
    } else {
      self.free.swap_remove(&index);
    }
  }

  /// Executes a round given a movement and updates the board. Returns `true` if the game is over.
  pub fn round(&mut self, movement: Move) -> bool {
    let changed = self.apply_move(movement);
    if changed {
      self.set_free_random(2);
    }

    self.is_over()
  }

  /// Returns `true` if the game is over, meaning that there's no free slot and no values can be merged together.
  pub fn is_over(&self) -> bool {
    self.free.is_empty() && !self.can_merge()
  }

  /// Returns `true` if any value in the board can be merged with another one.
  fn can_merge(&self) -> bool {
    for row in 0..ROWS {
      for column in 0..COLUMNS {
        // Check if a value can be merged horizontally
        let horizontally = row < ROWS - 1 && self.get(row, column) == self.get(row + 1, column);
        if horizontally {
          return true;
        }

        // Check if a value can be merge vertically
        let vertically = column < COLUMNS - 1 && self.get(row, column) == self.get(row, column + 1);
        if vertically {
          return true;
        }
      }
    }

    false
  }

  /// Applies a given `movement` to the board.
  pub fn apply_move(&mut self, movement: Move) -> bool {
    match movement {
      Move::Left => self.move_left(),
      Move::Right => self.move_right(),
      Move::Up => self.move_up(),
      Move::Down => self.move_down(),
    }
  }

  /// Applies a left movement to the board.
  fn move_left(&mut self) -> bool {
    let mut changed = false;

    for row in 0..ROWS {
      let mut i = 0;
      let mut j = i + 1;

      while i < COLUMNS && j < COLUMNS {
        let current = self.get(row, i);
        let next = self.get(row, j);

        // If the next value is 0, we ignore it
        if next == 0 {
          j += 1;
        }
        // If the current value is 0, we interchange values
        else if current == 0 {
          self.set(row, i, next);
          self.set(row, j, 0);
          j += 1;
          changed = true;
        } else {
          // If values can be merged, we merge next into current
          if current == next {
            let value = current * 2;

            self.set(row, i, value);
            self.set(row, j, 0);
            self.score += value;

            changed = true;
          }

          // Start next round from the next current index
          i += 1;
          j = i + 1;
        }
      }
    }

    changed
  }

  /// Applies a right movement to the board.
  fn move_right(&mut self) -> bool {
    let mut changed = false;

    for row in 0..ROWS {
      let mut i: i32 = COLUMNS as i32 - 1;
      let mut j: i32 = i - 1;

      while i > 0 && j >= 0 {
        let current = self.get(row, i as usize);
        let next = self.get(row, j as usize);

        // If the next value is 0, we ignore it
        if next == 0 {
          j -= 1;
        }
        // If the current value is 0, we interchange values
        else if current == 0 {
          self.set(row, i as usize, next);
          self.set(row, j as usize, 0);
          j -= 1;
          changed = true;
        } else {
          // If values can be merged, we merge next into current
          if current == next {
            let value = current * 2;

            self.set(row, i as usize, value);
            self.set(row, j as usize, 0);
            self.score += value;

            changed = true;
          }

          // Start next round from the next current index
          i -= 1;
          j = i - 1;
        }
      }
    }

    changed
  }

  /// Applies an up movement to the board.
  fn move_up(&mut self) -> bool {
    let mut changed = false;

    for column in 0..COLUMNS {
      let mut i = 0;
      let mut j = i + 1;

      while i < ROWS && j < ROWS {
        let current = self.get(i, column);
        let next = self.get(j, column);

        // If the next value is 0, we ignore it
        if next == 0 {
          j += 1;
        }
        // If the current value is 0, we interchange values
        else if current == 0 {
          self.set(i, column, next);
          self.set(j, column, 0);
          j += 1;
          changed = true;
        } else {
          // If values can be merged, we merge next into current
          if current == next {
            let value = current * 2;

            self.set(i, column, value);
            self.set(j, column, 0);
            self.score += value;

            changed = true;
          }

          // Start next round from the next current index
          i += 1;
          j = i + 1;
        }
      }
    }

    changed
  }

  /// Applies a down movement to the board.
  fn move_down(&mut self) -> bool {
    let mut changed = false;

    for column in 0..COLUMNS {
      let mut i: i32 = ROWS as i32 - 1;
      let mut j: i32 = i - 1;

      while i > 0 && j >= 0 {
        let current = self.get(i as usize, column);
        let next = self.get(j as usize, column);

        // If the next value is 0, we ignore it
        if next == 0 {
          j -= 1;
        }
        // If the current value is 0, we interchange values
        else if current == 0 {
          self.set(i as usize, column, next);
          self.set(j as usize, column, 0);
          j -= 1;
          changed = true;
        } else {
          // If values can be merged, we merge next into current
          if current == next {
            let value = current * 2;

            self.set(i as usize, column, value);
            self.set(j as usize, column, 0);
            self.score += value;

            changed = true;
          }

          // Start next round from the next current index
          i -= 1;
          j = i - 1;
        }
      }
    }

    changed
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut board = String::new();

    for i in 0..ROWS {
      board.push('[');
      for j in 0..COLUMNS {
        board.push_str(&self.get(i, j).to_string());
        if j < COLUMNS - 1 {
          board.push(' ')
        }
      }
      board.push(']');
      if i < ROWS - 1 {
        board.push('\n')
      }
    }
    writeln!(f, "{}", board)
  }
}

impl Default for Board {
  fn default() -> Self {
    let values = [0; LENGTH];
    let free = values.iter().enumerate().map(|(index, _)| index).collect();

    Board {
      values,
      free,
      score: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{Board, Move};

  #[test]
  fn move_left() {
    let mut game = Board::with_values([0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 0], 0);

    let changed = game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([2, 0, 0, 0, 4, 0, 0, 0, 4, 2, 0, 0, 4, 2, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2], 0);

    let changed = game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 4, 4, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 2, 4, 0, 8, 0, 2, 2, 2, 2, 0, 4, 2, 0, 0, 2], 0);

    let changed = game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([2, 4, 0, 0, 8, 4, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([2, 2, 2, 2, 4, 2, 2, 0, 2, 2, 0, 4, 2, 0, 2, 2], 0);

    let changed = game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([4, 4, 0, 0, 4, 4, 0, 0, 4, 4, 0, 0, 4, 2, 0, 0], 20)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 0, 0, 0, 2, 0, 0, 0, 4, 2, 0, 0, 8, 4, 2, 0], 12);

    let changed = game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 2, 0, 0, 0, 4, 2, 0, 0, 8, 4, 2, 0], 12)
    );
    assert!(!changed);
  }

  #[test]
  fn move_right() {
    let mut game = Board::with_values([2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2], 0);

    let changed = game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 2, 4, 0, 0, 2, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0], 0);

    let changed = game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 2, 0, 0, 0, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 4, 2, 0, 2, 2, 0, 8, 4, 0, 2, 2, 2, 0, 0, 2], 0);

    let changed = game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 4, 2, 0, 0, 4, 8, 0, 0, 4, 4, 0, 0, 0, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([2, 2, 2, 2, 4, 2, 2, 0, 2, 2, 0, 4, 2, 0, 2, 2], 0);

    let changed = game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 4, 4, 0, 0, 4, 4, 0, 0, 4, 4, 0, 0, 2, 4], 20)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 4, 0, 2, 4, 8], 0);

    let changed = game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 4, 0, 2, 4, 8], 0)
    );
    assert!(!changed);
  }

  #[test]
  fn move_down() {
    let mut game = Board::with_values([2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 0, 2, 2], 0);

    let changed = game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 4, 4, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 0, 0, 2, 2, 0], 0);

    let changed = game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 2, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 2, 4, 2, 4, 2, 0, 0, 2, 0, 2, 0, 0, 8, 2, 2], 0);

    let changed = game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 2, 8, 4, 4], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([2, 0, 4, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 4, 2, 2], 0);

    let changed = game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 2, 4, 4, 4, 4], 20)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 4, 0, 2, 4, 8], 0);

    let changed = game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 4, 0, 2, 4, 8], 0)
    );
    assert!(!changed);
  }

  #[test]
  fn move_up() {
    let mut game = Board::with_values([0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0], 0);

    let changed = game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([2, 4, 4, 4, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 2], 0);

    let changed = game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([0, 4, 2, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([0, 8, 2, 2, 2, 0, 2, 0, 4, 2, 0, 0, 0, 2, 4, 2], 0);

    let changed = game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([2, 8, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0], 12)
    );
    assert!(changed);

    let mut game = Board::with_values([2, 4, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 4, 2], 0);

    let changed = game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([4, 4, 4, 4, 4, 4, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0], 20)
    );
    assert!(changed);

    let mut game = Board::with_values([8, 4, 2, 0, 4, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0], 0);

    let changed = game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([8, 4, 2, 0, 4, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0], 0)
    );
    assert!(!changed);
  }

  #[test]
  fn is_over() {
    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 4], 0);

    assert!(!game.is_over());

    let game = Board::with_values([4, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 2], 0);

    assert!(!game.is_over());

    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 4, 4, 2, 8, 4, 2], 0);

    assert!(!game.is_over());

    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 2], 0);

    assert!(game.is_over());
  }

  #[test]
  fn as_rows() {
    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 4], 0);
    assert_eq!(
      game.as_rows(),
      vec![
        vec![8, 4, 2, 4],
        vec![2, 8, 4, 2],
        vec![8, 4, 2, 4],
        vec![2, 8, 4, 4],
      ]
    );
  }
}
