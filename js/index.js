const ROWS = 6;
const COLUMNS = 7;
const pieceColors = ['bg-red', 'bg-yellow'];

const boardHTML = document.getElementById('board');
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
            boardHTML.append(tile);

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
        animation.then(() => {
            endGame();
            menu.classList = 'border-0';
            turn.innerHTML = 'Draw';
        });
        return;
    }

    let matchingPieces = checkIfMoveWins(row, col, player);
    if (matchingPieces.length) {
        gameOver = true;
        animation.then(() => {
            endGame();
            turn.innerHTML = `Player ${player} wins!`;

            for (const [row, col] of matchingPieces) {
                const piece = boardHTML.children[row * 7 + col].firstChild;
                piece.dataset.win = true;
            }
        });
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

    const pieceDiv = document.createElement('div');
    pieceDiv.classList.add('piece');
    pieceDiv.classList.add(pieceColors[player - 1]);

    const location = boardHTML.children[row * 7 + col];
    location.appendChild(pieceDiv);

    const topY = boardHTML.children[col].getBoundingClientRect().y;
    const targetY = pieceDiv.getBoundingClientRect().y;
    const length = topY - targetY - 60;

    const animation = pieceDiv.animate(
        {
            transform: [
                `translateY(${length}px)`,
                'translateY(0px)',
                `translateY(${length / 10}px)`,
            ],
            offset: [0, 0.5, 0.7],
            easing: ['cubic-bezier(0.22, 0, 0.42, 0)', 'ease', 'ease'],
        },
        500
    );

    return { row, player, animation: animation.finished };
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
                const row_check = row + rise * step * direction;
                const col_check = col + run * step * direction;

                // Out of bounds or not matching
                if (
                    row_check < 0 || row_check >= board.length ||
                    col_check < 0 || col_check >= board[row].length ||
                    board[row_check][col_check] != piece
                )
                    break;

                matchingPieces.push([row_check, col_check]);
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
    const children = boardHTML.children;
    const bottomY = children[children.length - 1].getBoundingClientRect().y;

    for (let row = 0; row < ROWS; row++) {
        for (let col = 0; col < COLUMNS; col++) {
            const location = children[row * 7 + col];
            const piece = location.firstElementChild;
            if (!piece) continue;

            const pieceY = location.getBoundingClientRect().y;
            const distance = bottomY - pieceY + 60;

            const animation = piece.animate(
                {
                    transform: [
                        `translateY(${distance}px)`,
                    ],
                    easing: ['cubic-bezier(0.22, 0, 0.42, 0)'],
                },
                500 - (row * 50)
            );
            animation.onfinish = () => piece.remove();
        }
    }
}
