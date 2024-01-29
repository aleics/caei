export type BoardAction = 'init' | 'reset' | BoardMove;
export type BoardMove = 'left' | 'right' | 'up' | 'down';

const URL = 'http://localhost:8080/board'

export async function moveBoard(action: BoardMove): Promise<number[]> {
  const options: RequestInit = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ action })
  };

  const { rows } = await fetch(`${URL}/move`, options)
    .then((response) => response.json());

  return rows.flat();
}

export async function loadBoard(): Promise<number[]> {
  const { rows } = await fetch(URL)
    .then((response) => response.json());

  return rows.flat();
}

export async function resetBoard(): Promise<number[]> {
  const options: RequestInit = {
    method: 'POST'
  };

  const { rows } = await fetch(`${URL}/reset`, options)
    .then((response) => response.json());

  return rows.flat();
}
