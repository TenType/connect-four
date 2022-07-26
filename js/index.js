const board = new Array(6).fill(0).map(() => new Array(7).fill(0));
const display = ['_', 'R', 'Y'];

let turnClock = true;
let gameOver = false;

document.addEventListener('DOMContentLoaded', () => {
    const game = document.getElementById('game');
    game.innerHTML = getBoard();
});

function makeMove() {
    if (gameOver) return;

    const input = document.getElementById('move').value;
    const game = document.getElementById('game');
    const feedback = document.getElementById('feedback');
    
    if (isNaN(input) || input < 1 || input > 7) {
        feedback.innerHTML = 'Invalid move';
        return;
    }

    const col = input - 1;
    
    if (board[0][col] != 0) {
        feedback.innerHTML = 'Column is full!';
        return;
    }

    let row = board.length - 1;
    while (board[row][col] != 0) {
        row--;
    }

    const piece = getCurrentPlayer();
    board[row][col] = piece;

    if (checkIfMoveWins(row, col, piece)) {
        gameOver = true;
        game.innerHTML = getBoard();
        feedback.innerHTML = `Player ${piece} wins!`;

        const input = document.getElementById('move');
        const button = document.getElementById('action-button');

        input.parentNode.removeChild(input);
        button.parentNode.removeChild(button);
        return;
    }

    flipTurn();

    game.innerHTML = getBoard();
    feedback.innerHTML = '';
}

function getBoard() {
    let text = '';
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[i].length; j++) {
            text += display[board[i][j]];
        }
        text += '<br>';
    }
    return text;
}

function getCurrentPlayer() {
    return turnClock ? 1 : 2;
}

function flipTurn() {
    turnClock = !turnClock;
}

function checkIfMoveWins(row, col, piece) {
    // Horizontal, vertical, and diagonal checks
    for (const [run, rise] of [[1, 0], [0, 1], [1, -1], [1, 1]]) {
        let length = 1;

        // Check positive and negative directions
        for (const direction of [1, -1]) {
            // The last piece in a vertical win is always on top
            if (run == 0 && direction == -1)
                continue;

            for (let step = 1; step < 4; step++) {
                const row_check = row + rise * step * direction;
                const col_check = col + run * step * direction;

                // Out of bounds or not matching
                if (row_check < 0 || row_check >= board.length
                    || col_check < 0 || col_check >= board[row].length
                    || board[row_check][col_check] != piece)
                    break;

                length++;
                if (length == 4)
                    return true;
            }
        }
    }

    return false;
}
