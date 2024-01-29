import "./Header.css";

export default ({ onReset } : { onReset: () => void }) =>
  <div class="header">
    <div class="title">2048</div>
    <div class="subTitle">
      <span>Join the numbers and <br/>get to the 2048 tile and <strong>beyond</strong>!</span>
      <button class="reset" onClick={onReset}>New game</button>
    </div>
  </div>