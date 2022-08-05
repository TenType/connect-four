const ROWS = 6;
const COLUMNS = 7;
const PIECE_COLORS = ['bg-red', 'bg-yellow'];
const FALLING_EASE = 'cubic-bezier(0.22, 0, 0.42, 0)';
const FALLING_OFFSET = 60;

const grid = document.getElementById('board');
const menu = document.getElementById('menu');
const turn = document.getElementById('turn');
const counter = document.getElementById('turn-counter');
const moveHistory = document.getElementById('history');
const resetButton = document.getElementById('reset');

let board;
let turnClock;
let gameOver;
let moves;

createBoard();
newGame();

function newGame() {
    board = new Array(ROWS).fill(0).map(() => new Array(COLUMNS).fill(0));
    turnClock = true;
    gameOver = false;
    moves = [];

    menu.dataset.done = '';
    resetButton.dataset.primary = '';
    moveHistory.value = '';
    updateMenu();
}

function createBoard() {
    for (let row = 0; row < ROWS; row++) {
        for (let col = 0; col < COLUMNS; col++) {
            const tile = document.createElement('div');
            tile.classList.add('tile');
            grid.append(tile);

            tile.onclick = () => makeMove(col);
        }
    }
}

function makeMove(col) {
    if (gameOver || board[0][col] != 0) return;

    const { row, player, animation } = dropPiece(col);

    moves.push(col + 1);
    moveHistory.value = moves.join('');

    if (checkIfDraw(row)) {
        gameOver = true;
        animation.onfinish = () => {
            endGame();
            menu.classList = 'border-0';
            turn.innerHTML = 'Draw';
        };
        return;
    }

    let matchingPieces = checkIfMoveWins(row, col, player);
    if (matchingPieces.length) {
        gameOver = true;
        animation.onfinish = () => {
            endGame();
            turn.innerHTML = `Player ${player} wins!`;

            for (const [row, col] of matchingPieces) {
                const piece = grid.children[row * 7 + col].firstChild;
                piece.dataset.win = true;
            }
        };
        return;
    }

    flipTurn();
    updateMenu();
}

function dropPiece(col) {
    let row = board.length - 1;
    while (board[row][col] != 0) {
        row--;
    }

    const player = getCurrentPlayer();
    board[row][col] = player;

    const piece = document.createElement('div');
    piece.classList.add('piece');
    piece.classList.add(PIECE_COLORS[player - 1]);

    const location = grid.children[row * 7 + col];
    location.appendChild(piece);

    const topY = grid.children[0].getBoundingClientRect().y;
    const pieceY = piece.getBoundingClientRect().y;
    const distance = topY - pieceY - FALLING_OFFSET;

    const animation = piece.animate(
        {
            transform: [
                `translateY(${distance}px)`,
                'translateY(0px)',
                `translateY(${distance / 10}px)`,
            ],
            offset: [0, 0.5, 0.7],
            easing: [FALLING_EASE, 'ease', 'ease'],
        },
        500
    );

    return { row, player, animation};
}

function getCurrentPlayer() {
    return turnClock ? 1 : 2;
}

function flipTurn() {
    turnClock = !turnClock;
}

function updateMenu() {
    const player = getCurrentPlayer();

    menu.classList = `border-${player}`;
    turn.innerHTML = `Player ${player}'s Turn`;

    counter.innerHTML = `Turn ${Math.ceil((moves.length + 1) / 2)}, Move ${moves.length + 1}`;
}

function endGame() {
    menu.dataset.done = 'true';
    resetButton.dataset.primary = 'true';
}

function checkIfDraw(row) {
    return row == 0 && !board[0].includes(0);
}

function checkIfMoveWins(row, col, piece) {
    // Horizontal, vertical, and diagonal checks
    for (const [run, rise] of [[1, 0], [0, 1], [1, -1], [1, 1]]) {
        let matchingPieces = [[row, col]];

        // Check positive and negative directions
        for (const direction of [1, -1]) {
            // The last piece in a vertical win is always on top
            if (run == 0 && direction == -1) continue;

            for (let step = 1; step < 4; step++) {
                const rowCheck = row + rise * step * direction;
                const colCheck = col + run * step * direction;

                // Out of bounds or not matching
                if (
                    rowCheck < 0 || rowCheck >= board.length ||
                    colCheck < 0 || colCheck >= board[row].length ||
                    board[rowCheck][colCheck] != piece
                )
                    break;

                matchingPieces.push([rowCheck, colCheck]);
                if (matchingPieces.length == 4) return matchingPieces;
            }
        }
    }

    return [];
}

function resetGame() {
    emptyBoard();
    newGame();
}

function emptyBoard() {
    const bottomY = grid.children[grid.children.length - 1].getBoundingClientRect().y;

    for (let row = 0; row < ROWS; row++) {
        for (let col = 0; col < COLUMNS; col++) {
            const location = grid.children[row * 7 + col];
            const piece = location.firstElementChild;
            if (!piece) continue;

            const pieceY = location.getBoundingClientRect().y;
            const distance = bottomY - pieceY + FALLING_OFFSET;

            const animation = piece.animate(
                {
                    transform: [
                        `translateY(${distance}px)`,
                    ],
                    easing: [FALLING_EASE],
                },
                500 - (row * 50)
            );
            animation.onfinish = () => piece.remove();
        }
    }
}
