extern crate shakmaty;

use shakmaty::{Pieces, Role, Color, Chess, Move, Position, Setup};

use std::cmp::max;
use std::cmp::min;

pub struct AI {

}

impl AI {
    pub fn new() -> AI {
        AI {}
    }

}

pub fn get_values(pieces: &Pieces) -> i32 {
    let mut total = 0;

    for i in 0..pieces.len() {
        // kings
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::King)
            .map(|piece| if piece.1.color == Color::White {total += 900} else {total -= 900});

        // queens
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Queen)
            .map(|piece| if piece.1.color == Color::White {total += 90} else {total -= 90});

        // rooks
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Rook)
            .map(|piece| if piece.1.color == Color::White {total += 50} else {total -= 50});

        // bishops and knights
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Bishop || piece.1.role == Role::Knight)
            .map(|piece| if piece.1.color == Color::White {total += 30} else {total -= 30});

        // pawns
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Pawn)
            .map(|piece| if piece.1.color == Color::White {total += 10} else {total -= 10});
    }

    total
}

// special thanks to https://www.freecodecamp.org/news/simple-chess-ai-step-by-step-1d55a9266977/
//
// Recursive function to decide the best move based on the future
// (This does not gives us the *really* best move, it just sieves out the dumb moves
pub fn minimax(depth: u32, game: Chess, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return -get_values(&game.board().pieces());
    }

    let new_game_moves = game.legals();

    if game.turn() == shakmaty::Color::Black {
        let mut best_move = -9999;
        for i in 0..new_game_moves.len() {
            let temp_board = game.to_owned().play(&new_game_moves[i]);
            best_move = max(best_move, minimax(depth - 1, temp_board.unwrap(), alpha, beta));

            alpha = max(alpha, best_move);
            if alpha >= beta {
                return best_move;
            }
        }
        best_move
    }
    else {
        let mut best_move = 9999;
        for i in 0..new_game_moves.len() {
            let temp_board = game.to_owned().play(&new_game_moves[i]);
            best_move = min(best_move, minimax(depth - 1, temp_board.unwrap(), alpha, beta));

            beta = min(beta, best_move);
            if alpha >= beta {
                return best_move;
            }
        }
        best_move
    }
}

pub fn minimax_root(depth: u32, game: Chess) -> Move {
    let new_game_moves = game.legals();
    let mut best_value = -9999;

    // arbitrary value to avoid undefined behaviour
    let mut best_move_found: Move = new_game_moves[0].clone();

    for i in 0..new_game_moves.len() {
        let new_game_move = &new_game_moves[i];

        let temp_board = game.to_owned().play(&new_game_move);

        let curr_value = minimax(depth - 1, temp_board.unwrap(), -10000, 10000);

        if curr_value >= best_value {
            best_value = curr_value;
            best_move_found = new_game_move.clone();
        }
    }

    best_move_found
}
