const ROWS = 6;
const COLUMNS = 7;
const board = new Array(ROWS).fill(0).map(() => new Array(COLUMNS).fill(0));
const gradientDark = 0.6;

const boardHTML = document.getElementById('board');

class RGB {
    constructor(red, green, blue) {
        this.red = red;
        this.green = green;
        this.blue = blue;
    }
}

const colors = [
    new RGB(50, 50, 50),
    new RGB(255, 22, 22),
    new RGB(230, 225, 0),
];

let turnClock = true;
let gameOver = false;

createBoard();

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

    if (checkIfDraw(row)) {
        gameOver = true;
        animation.then(() => {
            alert('The game is a draw!');
        });
        return;
    }

    if (checkIfMoveWins(row, col, player)) {
        gameOver = true;
        animation.then(() => {
            alert(`Player ${player} wins!`);
        });
        return;
    }

    flipTurn();
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
    pieceDiv.style.backgroundImage = getPieceColor(player);

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

function getPieceColor(n) {
    const bright = `rgb(${colors[n].red}, ${colors[n].green}, ${colors[n].blue})`;
    const dark = `rgb(${colors[n].red * gradientDark}, ${colors[n].green * gradientDark}, ${colors[n].blue * gradientDark})`;
    return `linear-gradient(${bright}, ${dark})`;
}

function getCurrentPlayer() {
    return turnClock ? 1 : 2;
}

function flipTurn() {
    turnClock = !turnClock;
}

function checkIfDraw(row) {
    return row == 0 && !board[0].includes(0);
}

function checkIfMoveWins(row, col, piece) {
    // Horizontal, vertical, and diagonal checks
    for (const [run, rise] of [[1, 0], [0, 1], [1, -1], [1, 1]]) {
        let length = 1;

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

                length++;
                if (length == 4) return true;
            }
        }
    }

    return false;
}
