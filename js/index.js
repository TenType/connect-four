const board = new Array(6).fill('_').map(() => new Array(7).fill('_'));
let turnClock = true;

document.addEventListener('DOMContentLoaded', () => {
    const game = document.getElementById('game');
    game.innerHTML = getBoard();
});

function makeMove() {
    const input = document.getElementById('move').value;
    const game = document.getElementById('game');
    const feedback = document.getElementById('feedback');
    
    if (isNaN(input) || input < 1 || input > 7) {
        feedback.innerHTML = 'Invalid move';
        return;
    }
    
    if (board[0][input - 1] != '_') {
        feedback.innerHTML = 'Column is full!';
        return;
    }

    let row = board.length - 1;
    while (board[row][input - 1] != '_') {
        row--;
    }

    board[row][input - 1] = getCurrentPlayer();
    flipTurn();

    game.innerHTML = getBoard();
    feedback.innerHTML = '';
}

function getBoard() {
    let text = '';
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[i].length; j++) {
            text += board[i][j];
        }
        text += '<br>';
    }
    return text;
}

function getCurrentPlayer() {
    return turnClock ? 'R' : 'Y';
}

function flipTurn() {
    turnClock = !turnClock;
}
