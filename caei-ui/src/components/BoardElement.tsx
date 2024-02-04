import "./BoardElement.css";

const COLORS = {
  0: { background: "#ccc0b3", color: "#786f66" },
  2: { background: "#eee4da", color: "#786f66" },
  4: { background: "#ede0c8", color: "#786f66" },
  8: { background: "#f2b179", color: "#fff" },
  16: { background: "#f59563", color: "#fff" },
  32: { background: "#f67c5f", color: "#fff" },
  64: { background: "#f65e3b", color: "#fff" },
  128: { background: "#edcf72", color: "#fff" },
  256: { background: "#edcc61", color: "#fff" },
  512: { background: "#edc850", color: "#fff" },
  1024: { background: "#edc53f", color: "#fff" },
  2048: { background: "#edc22e", color: "#fff" },
  4096: { background: "#637b84", color: "#fff" },
  8192: { background: "#7e969f", color: "#fff" },
  16384: { background: "#84b5ca", color: "#fff" },
  32768: { background: "#91cdd5", color: "#fff" },
  65536: { background: "#91d0f8", color: "#fff" },
};

export interface BoardElementProps {
  value: number;
}

export default (props: BoardElementProps) => {
  const value = props.value;
  const { background, color } = COLORS[value] ?? { background: "#000", color: "#fff" };
  const text = value == 0 ? "" : value.toString();


  return <span class="element" style={{ background, color }}>{text}</span>;
}
