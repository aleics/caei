use caei_core::{Board, Move};
use std::io::stdin;

fn main() {
  let mut game = Board::new();

  loop {
    println!("{}", game);
    println!("a, w, s, d or exit: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let done = match buffer.as_str().trim() {
      "a" => game.round(Move::Left),
      "d" => game.round(Move::Right),
      "w" => game.round(Move::Up),
      "s" => game.round(Move::Down),
      "exit" => true,
      _ => {
        println!("Unknown move {}. Try again.", buffer);
        false
      }
    };

    if done {
      println!("Game over");
      println!("{}", game);
      break;
    }
  }
}
