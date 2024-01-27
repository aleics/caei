import { For } from "solid-js";
import BoardElement from "./BoardElement";
import "./Board.css";

export interface BoardProps {
  elements: number[];
}

export default (props: BoardProps) =>
  <div class="board">
    <For each={props.elements}>{(element) =>
      <BoardElement value={element} />
    }</For>
  </div>;
