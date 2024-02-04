import { type Component, createSignal, onMount } from 'solid-js';
import { BoardMove, BoardState, loadBoard, moveBoard, resetBoard } from './services/board';
import Game from './components/Game';

type BoardAction = 'init' | 'reset' | BoardMove;

async function sendAction(action: BoardAction): Promise<BoardState> {
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

export default () => {
  const [game, setGame] = createSignal<BoardState>({ elements: [], score: 0 });

  const applyMove = async (action: BoardAction) => {
    const game = await sendAction(action);
    setGame(game);
  }

  document.addEventListener('keydown', async (event) => {
    switch (event.code) {
      case 'ArrowUp':
      case 'KeyW':
        await applyMove('up');
        break;
      case 'ArrowDown':
      case 'KeyS':
        await applyMove('down');
        break;
      case 'ArrowLeft':
      case 'KeyA':
        await applyMove('left');
        break;
      case 'ArrowRight':
      case 'KeyD':
        await applyMove('right');
        break;
      case 'KeyN':
        await applyMove('reset');
        break;
    }
  });

  onMount(async () => {
    await applyMove('init');
  });

  return <Game board={game()} onReset={() => applyMove('reset')}/>;
};