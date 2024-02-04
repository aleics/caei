import "./Header.css";

export interface HeaderProps {
  score: number;
  onReset: () => void;
}

export default (props : HeaderProps) =>
  <div class="header">
    <div class="title">
      <span>2048</span>
      <div class="score">
        <span>Score</span><br/>
        <strong>{props.score}</strong>
      </div>
    </div>
    <div class="subTitle">
      <span>Join the numbers and <br/>get to the 2048 tile and <strong>beyond</strong>!</span>
      <button class="reset" onClick={props.onReset}>New game</button>
    </div>
  </div>