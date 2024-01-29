import { createResource, type Component, createSignal, createEffect } from 'solid-js';
import Header from './components/Header';
import Board from './components/Board';
import "./App.css";
import { BoardAction, loadBoard, moveBoard, resetBoard } from './services/board';
import Footer from './components/Footer';


async function applyMove({ action }: { action: BoardAction }): Promise<number[]> {
  switch (action) {
    case 'init':
      return loadBoard();

    case 'reset':
      return resetBoard();

    case 'left':
    case 'right':
    case 'up':
    case 'down':
      return moveBoard(action);
  }
}

const App: Component = () => {
  const [move, setMove] = createSignal<{ action: BoardAction }>({ action: 'init' });
  const [elements] = createResource(move, applyMove);
  const onReset = () => setMove({ action: 'reset' });

  document.addEventListener('keydown', (event) => {
    switch (event.code) {
      case 'ArrowUp':
      case 'KeyW':
        setMove({ action: 'up' });
        break;
      case 'ArrowDown':
      case 'KeyS':
        setMove({ action: 'down' });
        break;
      case 'ArrowLeft':
      case 'KeyA':
        setMove({ action: 'left' });
        break;
      case 'ArrowRight':
      case 'KeyD':
        setMove({ action: 'right' });
        break;
      case 'KeyN':
        onReset();
        break;
    }
  });

  return (
    <div class="game">
      <Header onReset={onReset} />
      <Board elements={elements()} />
      <Footer />
    </div>
  );
};

export default App;