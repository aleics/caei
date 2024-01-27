import { createResource, type Component, createSignal, createEffect } from 'solid-js';
import Header from './components/Header';
import Board from './components/Board';
import "./App.css";

export type BoardAction = 'init' | BoardMove;
export type BoardMove = 'left' | 'right' | 'up' | 'down';

async function moveBoard(action: BoardMove): Promise<number[]> {
  const options: RequestInit = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ action })
  };

  const { rows } = await fetch(`http://localhost:8080/board/move`, options)
    .then((response) => response.json());

  return rows.flat();
}

async function loadBoard(): Promise<number[]> {
  const { rows } = await fetch(`http://localhost:8080/board`)
    .then((response) => response.json());

  return rows.flat();
}

async function fetchData({ action }: { action: BoardAction }): Promise<number[]> {
  return action == 'init' ? loadBoard() : moveBoard(action);
}

const App: Component = () => {
  const [move, setMove] = createSignal<{ action: BoardAction }>({ action: 'init' });
  const [elements] = createResource(move, fetchData)

  document.addEventListener('keydown', (event) => {
    switch(event.key) {
      case 'ArrowUp':
        setMove({ action: 'up' });
        break;
      case 'ArrowDown':
        setMove({ action: 'down' });
        break;
      case 'ArrowLeft':
        setMove({ action: 'left' });
        break;
      case 'ArrowRight':
        setMove({ action: 'right' });
        break;
    }
  });

  return (
    <div class="game">
      <Header/>
      <Board elements={elements()} />
    </div>
  );
};

export default App;