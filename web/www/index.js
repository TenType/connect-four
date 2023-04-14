import init, { App } from '../pkg/connect_four_website.js';

const board = document.getElementById('board');
const WIDTH = 7;
const HEIGHT = 6;

init().then(async () => {
  const bytes = await loadOpeningBook();
  const app = new App(bytes);
  setup(app);
  console.log(app.evaluate());
});

async function loadOpeningBook() {
  const file = await fetch('../../database/opening_book.bin');
  const buffer = await file.arrayBuffer();
  return new Uint8Array(buffer);
}

/**
 * @param {App} app
 */
function setup(app) {
  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const tile = document.createElement('div');
      tile.classList.add('tile');
      board.appendChild(tile);
      tile.onclick = () => makeMove(app, col);
    }
  }
}

/**
 * @param {App} app
 * @param {number} col
 */
function makeMove(app, col) {
  if (app.is_game_over()) return;

  const row = HEIGHT - app.play(col) - 1;
  if (row < 0) {
    // Move cannot be played
    return;
  }

  const piece = document.createElement('div');
  piece.classList.add('piece');

  if (!app.first_player_turn()) {
    piece.dataset.color = 'red';
  } else {
    piece.dataset.color = 'yellow';
  }

  const index = row * 7 + col;
  const gridCell = board.children[index];
  gridCell.appendChild(piece);

  const topY = board.children[0].getBoundingClientRect().y;
  const pieceY = piece.getBoundingClientRect().y;
  const distance = topY - pieceY - 60;

  const animation = piece.animate(
    {
      transform: [
        `translateY(${distance}px)`,
        'translateY(0px)',
        `translateY(${distance / 10}px)`,
      ],
      offset: [0, 0.5, 0.7],
      easing: ['cubic-bezier(0.22, 0, 0.42, 0)', 'ease', 'ease'],
    },
    500
  );

  const winner = app.winner();
  if (winner != 0) {
    animation.onfinish = () => alert(`Player ${winner} won!`);
  } else if (app.is_draw()) {
    animation.onfinish = () => alert('The game is a draw!');
  } else {
    console.log(app.evaluate());
  }
}
