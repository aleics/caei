use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
  routing::{get, post},
  Json, Router,
};
use caei_core::{Board, Move};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

#[derive(Clone)]
struct SharedState {
  state: Arc<RwLock<Board>>,
}

impl SharedState {
  fn new(board: Board) -> Self {
    SharedState {
      state: Arc::new(RwLock::new(board)),
    }
  }

  fn read(&self) -> BoardDTO {
    let board = self.state.read().unwrap();
    BoardDTO::from(board.deref())
  }

  fn apply_move(&self, movement: MoveDTO) {
    let mut board = self.state.write().unwrap();
    board.apply_move(Move::from(movement));
  }
}

#[derive(Serialize)]
struct BoardDTO {
  rows: Vec<Vec<i32>>,
}

impl From<&Board> for BoardDTO {
  fn from(board: &Board) -> Self {
    BoardDTO {
      rows: board.as_rows(),
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum MoveDTO {
  Left,
  Right,
  Up,
  Down,
}

impl From<MoveDTO> for Move {
  fn from(value: MoveDTO) -> Self {
    match value {
      MoveDTO::Left => Move::Left,
      MoveDTO::Right => Move::Right,
      MoveDTO::Up => Move::Up,
      MoveDTO::Down => Move::Down,
    }
  }
}

#[derive(Deserialize)]
struct MoveBoard {
  action: MoveDTO,
}

async fn get_board(State(state): State<SharedState>) -> impl IntoResponse {
  Json(state.read())
}

async fn move_board(
  State(state): State<SharedState>,
  Json(input): Json<MoveBoard>,
) -> impl IntoResponse {
  state.apply_move(input.action);
  Json(state.read())
}

fn routes(board: Board) -> Router {
  Router::new()
    .route("/board", get(get_board))
    .route("/board/move", post(move_board))
    .with_state(SharedState::new(board))
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = routes(Board::new());

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
