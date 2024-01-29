use axum::extract::State;
use axum::http::{self, HeaderValue, Method};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use caei_core::{Board, Move};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

struct Game {
  board: Board,
  is_over: bool,
}

#[derive(Clone)]
struct SharedState {
  state: Arc<RwLock<Game>>,
}

impl SharedState {
  fn new(board: Board) -> Self {
    SharedState {
      state: Arc::new(RwLock::new(Game {
        board,
        is_over: false,
      })),
    }
  }

  fn read(&self) -> BoardDTO {
    let game = self.state.read().unwrap();
    BoardDTO::from(&game.board)
  }

  fn apply_move(&self, movement: MoveDTO) {
    let mut game = self.state.write().unwrap();
    if !game.is_over {
      game.is_over = game.board.round(Move::from(movement));
    }
  }

  fn reset(&self) {
    let mut game = self.state.write().unwrap();
    game.board = Board::new();
    game.is_over = false;
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

async fn reset_board(State(state): State<SharedState>) -> impl IntoResponse {
  state.reset();
  Json(state.read())
}

fn routes(board: Board) -> Router {
  Router::new()
    .route("/board", get(get_board))
    .route("/board/move", post(move_board))
    .route("/board/reset", post(reset_board))
    .layer(
      CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([http::header::CONTENT_TYPE]),
    )
    .with_state(SharedState::new(board))
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = routes(Board::new());

  let listener = TcpListener::bind("localhost:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
