export type BoardMove = 'left' | 'right' | 'up' | 'down';

export interface BoardState {
  elements: number[];
  score: number;
}

const URL = 'http://localhost:8080/board'

export async function moveBoard(action: BoardMove): Promise<BoardState> {
  const options: RequestInit = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ action })
  };

  const { rows, score } = await fetch(`${URL}/move`, options)
    .then((response) => response.json());

  return { elements: rows.flat(), score };
}

export async function loadBoard(): Promise<BoardState> {
  const { rows, score } = await fetch(URL)
    .then((response) => response.json());

    return { elements: rows.flat(), score };
}

export async function resetBoard(): Promise<BoardState> {
  const options: RequestInit = {
    method: 'POST'
  };

  const { rows, score } = await fetch(`${URL}/reset`, options)
    .then((response) => response.json());

    return { elements: rows.flat(), score };
}
