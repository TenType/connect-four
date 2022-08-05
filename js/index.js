const ROWS = 6;
const COLUMNS = 7;
const PIECE_COLORS = ['bg-red', 'bg-yellow'];
const FALLING_EASE = 'cubic-bezier(0.22, 0, 0.42, 0)';
const FALLING_OFFSET = 60;

// References to elements in the document
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
    // Initialize the board with zeroes
    board = new Array(ROWS).fill(0).map(() => new Array(COLUMNS).fill(0));

    // Reset all variables
    turnClock = true;
    gameOver = false;
    moves = [];

    // Reset HTML data
    menu.dataset.done = '';
    resetButton.dataset.primary = '';
    moveHistory.value = '';

    updateMenu();
}

function createBoard() {
    // Fill up the grid with tiles
    for (let row = 0; row < ROWS; row++) {
        for (let col = 0; col < COLUMNS; col++) {
            const tile = document.createElement('div');
            tile.classList.add('tile');
            grid.append(tile);

            // Drop a piece for the current player when a column is clicked
            tile.onclick = () => makeMove(col);
        }
    }
}

function makeMove(col) {
    // Check if game over or the column is full
    if (gameOver || board[0][col] != 0) return;

    // Animate dropping the piece
    const { row, player, animation } = dropPiece(col);

    // Save it to the move history
    moves.push(col + 1);
    moveHistory.value = moves.join('');

    // Draw game
    if (checkIfDraw(row)) {
        gameOver = true;
        animation.onfinish = () => {
            endGame();
            menu.classList = 'border-0';
            turn.innerHTML = 'Draw';
        };
        return;
    }

    // Connect Four
    let matchingPieces = checkIfMoveWins(row, col, player);
    if (matchingPieces.length) {
        gameOver = true;
        animation.onfinish = () => {
            endGame();
            turn.innerHTML = `Player ${player} wins!`;

            // Set a blinking animation for the pieces in the Connect Four
            for (const [row, col] of matchingPieces) {
                const piece = grid.children[row * 7 + col].firstChild;
                piece.dataset.win = true;
            }
        };
        return;
    }

    // Next turn
    flipTurn();
    updateMenu();
}

function dropPiece(col) {
    // Find row of dropped piece
    let row = board.length - 1;
    while (board[row][col] != 0) {
        row--;
    }

    // Update the board state
    const player = getCurrentPlayer();
    board[row][col] = player;

    // Create the piece
    const piece = document.createElement('div');
    piece.classList.add('piece');
    piece.classList.add(PIECE_COLORS[player - 1]);

    // Append the piece to the location in the grid
    const location = grid.children[row * 7 + col];
    location.appendChild(piece);

    // Calculate the distance needed for the piece to fall to its location
    const topY = grid.children[0].getBoundingClientRect().y;
    const pieceY = piece.getBoundingClientRect().y;
    const distance = topY - pieceY - FALLING_OFFSET;

    // Animate dropping the piece
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

    return { row, player, animation };
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
    // Get Y position of the bottom row
    const bottomY = grid.children[grid.children.length - 1].getBoundingClientRect().y;

    // Empty every piece on the grid
    for (let row = 0; row < ROWS; row++) {
        for (let col = 0; col < COLUMNS; col++) {
            // Get a location of the piece at the row and column
            const location = grid.children[row * 7 + col];
            const piece = location.firstElementChild;

            // Continue the loop if there is no piece on the tile
            if (!piece) continue;

            // Calculate the distance needed for the piece to fall off the board
            const pieceY = location.getBoundingClientRect().y;
            const distance = bottomY - pieceY + FALLING_OFFSET;

            // Animate dropping the piece
            const animation = piece.animate(
                {
                    transform: [
                        `translateY(${distance}px)`,
                    ],
                    easing: [FALLING_EASE],
                },
                500 - (row * 50)
            );

            // Remove the piece when the animation is done
            animation.onfinish = () => piece.remove();
        }
    }
}
