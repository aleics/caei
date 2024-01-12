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
  values: [i32; LENGTH],
  free: IndexSet<usize>,
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
  pub fn with_values(values: [i32; LENGTH]) -> Self {
    let free = values
      .into_iter()
      .enumerate()
      .filter(|(_, value)| value == &0)
      .map(|(index, _)| index)
      .collect();

    Board { values, free }
  }

  /// Returns the value of the board given a `row` and `column` position
  fn get(&self, row: usize, column: usize) -> i32 {
    self.values[index(row, column)]
  }

  /// Sets a `value` in the board given a `row` and `column` position.
  fn set(&mut self, row: usize, column: usize, value: i32) {
    let index = index(row, column);
    self.set_index(index, value)
  }

  /// Sets a `value` in the board to a random free slot.
  fn set_free_random(&mut self, value: i32) {
    let index = rand::thread_rng().gen_range(0..self.free.len());

    if let Some(free_index) = self.free.get_index(index) {
      self.set_index(*free_index, value);
    }
  }

  /// Sets a `value` in the board by a given `index`
  fn set_index(&mut self, index: usize, value: i32) {
    self.values[index] = value;

    if value == 0 {
      self.free.insert(index);
    } else {
      self.free.remove(&index);
    }
  }

  /// Executes a round given a movement and updates the board. Returns `true` if the game is over.
  pub fn round(&mut self, movement: Move) -> bool {
    self.apply_move(movement);
    self.set_free_random(2);

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
  pub fn apply_move(&mut self, movement: Move) {
    match movement {
      Move::Left => self.move_left(),
      Move::Right => self.move_right(),
      Move::Up => self.move_up(),
      Move::Down => self.move_down(),
    }
  }

  /// Applies a left movement to the board.
  fn move_left(&mut self) {
    for row in 0..ROWS {
      let mut combined = [false; COLUMNS];

      for column in 0..COLUMNS {
        for window in (1..=column).rev() {
          let current_value = self.get(row, window);
          let previous_value = self.get(row, window - 1);

          // If the previous value on the shifting direction is 0,
          // we move the current value to the previous position.
          if previous_value == 0 {
            self.set(row, window - 1, current_value);
            self.set(row, window, 0);
          }
          // If the previous and current value are equal and have not been combined yet,
          // we combine those values in the previous position.
          else if previous_value == current_value && !combined[window - 1] {
            self.set(row, window - 1, previous_value + current_value);
            combined[window - 1] = true;
            self.set(row, window, 0);
          }
        }
      }
    }
  }

  /// Applies a right movement to the board.
  fn move_right(&mut self) {
    for row in 0..ROWS {
      let mut combined = [false; COLUMNS];

      for column in (0..COLUMNS).rev() {
        for window in column..(COLUMNS - 1) {
          let current_value = self.get(row, window);
          let previous_value = self.get(row, window + 1);

          // If the previous value on the shifting direction is 0,
          // we move the current value to the previous position.
          if previous_value == 0 {
            self.set(row, window + 1, current_value);
            self.set(row, window, 0);
          }
          // If the previous and current value are equal and have not been combined yet,
          // we combine those values in the previous position.
          else if previous_value == current_value && !combined[window + 1] {
            self.set(row, window + 1, previous_value + current_value);
            combined[window + 1] = true;
            self.set(row, window, 0);
          }
        }
      }
    }
  }

  /// Applies an up movement to the board.
  fn move_up(&mut self) {
    for column in 0..COLUMNS {
      let mut combined = [false; ROWS];

      for row in 0..ROWS {
        for window in (1..=row).rev() {
          let current_value = self.get(window, column);
          let previous_value = self.get(window - 1, column);

          // If the previous value on the shifting direction is 0,
          // we move the current value to the previous position.
          if previous_value == 0 {
            self.set(window - 1, column, current_value);
            self.set(window, column, 0);
          }
          // If the previous and current value are equal and have not been combined yet,
          // we combine those values in the previous position.
          else if previous_value == current_value && !combined[window - 1] {
            self.set(window - 1, column, previous_value + current_value);
            combined[window - 1] = true;
            self.set(window, column, 0);
          }
        }
      }
    }
  }

  /// Applies a down   movement to the board.
  fn move_down(&mut self) {
    for column in 0..COLUMNS {
      let mut combined = [false; ROWS];

      for row in (0..ROWS).rev() {
        for window in row..(ROWS - 1) {
          let current_value = self.get(window, column);
          let previous_value = self.get(window + 1, column);

          // If the previous value on the shifting direction is 0,
          // we move the current value to the previous position.
          if previous_value == 0 {
            self.set(window + 1, column, current_value);
            self.set(window, column, 0);
          }
          // If the previous and current value are equal and have not been combined yet,
          // we combine those values in the previous position.
          else if previous_value == current_value && !combined[window + 1] {
            self.set(window + 1, column, previous_value + current_value);
            combined[window + 1] = true;
            self.set(window, column, 0);
          }
        }
      }
    }
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

    Board { values, free }
  }
}

#[cfg(test)]
mod tests {
  use crate::{Board, Move};

  #[test]
  fn move_left() {
    let mut game = Board::with_values([0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 0]);

    game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([2, 0, 0, 0, 4, 0, 0, 0, 4, 2, 0, 0, 4, 2, 0, 0])
    );

    let mut game = Board::with_values([0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2]);

    game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 4, 4, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0])
    );

    let mut game = Board::with_values([0, 2, 4, 0, 8, 0, 2, 2, 2, 2, 0, 4, 2, 0, 0, 2]);

    game.apply_move(Move::Left);

    assert_eq!(
      game,
      Board::with_values([2, 4, 0, 0, 8, 4, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0])
    );
  }

  #[test]
  fn move_right() {
    let mut game = Board::with_values([2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2]);

    game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 2, 4, 0, 0, 2, 4])
    );

    let mut game = Board::with_values([0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0]);

    game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 2, 0, 0, 0, 4])
    );

    let mut game = Board::with_values([0, 4, 2, 0, 2, 2, 0, 8, 4, 0, 2, 2, 2, 0, 0, 2]);

    game.apply_move(Move::Right);

    assert_eq!(
      game,
      Board::with_values([0, 0, 4, 2, 0, 0, 4, 8, 0, 0, 4, 4, 0, 0, 0, 4])
    );
  }

  #[test]
  fn move_up() {
    let mut game = Board::with_values([0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0]);

    game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([2, 4, 4, 4, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,])
    );

    let mut game = Board::with_values([0, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 2]);

    game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([0, 4, 2, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    );

    let mut game = Board::with_values([0, 8, 2, 2, 2, 0, 2, 0, 4, 2, 0, 0, 0, 2, 4, 2]);

    game.apply_move(Move::Up);

    assert_eq!(
      game,
      Board::with_values([2, 8, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    );
  }

  #[test]
  fn move_down() {
    let mut game = Board::with_values([2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 0, 2, 2]);

    game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 4, 4, 4,])
    );

    let mut game = Board::with_values([0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 0, 0, 2, 2, 0]);

    game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 2, 4])
    );

    let mut game = Board::with_values([0, 2, 4, 2, 4, 2, 0, 0, 2, 0, 2, 0, 0, 8, 2, 2]);

    game.apply_move(Move::Down);

    assert_eq!(
      game,
      Board::with_values([0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 2, 8, 4, 4])
    );
  }

  #[test]
  fn is_over() {
    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 4]);

    assert_eq!(game.is_over(), false);

    let game = Board::with_values([4, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 2]);

    assert_eq!(game.is_over(), false);

    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 4, 4, 2, 8, 4, 2]);

    assert_eq!(game.is_over(), false);

    let game = Board::with_values([8, 4, 2, 4, 2, 8, 4, 2, 8, 4, 2, 4, 2, 8, 4, 2]);

    assert_eq!(game.is_over(), true);
  }
}
