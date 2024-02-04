import { BoardState } from "../services/board";
import Board from "./Board";
import Footer from "./Footer";
import Header from "./Header";
import "./Game.css";

export interface GameProps {
  board: BoardState;
  onReset: () => void;
}

export default (props: GameProps) =>
  <div class="game">
    <Header score={props.board.score} onReset={props.onReset} />
    <Board elements={props.board.elements} />
    <Footer />
  </div>;
