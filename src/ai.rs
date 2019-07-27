extern crate shakmaty;

use shakmaty::{Pieces, Role, Color, Chess, Move, Position, Setup};

use std::cmp::max;
use std::cmp::min;

// temporary for debugging
static mut POSITION_COUNT: u32 = 0;

// simplified evaluation arrays
const PAWN_EVAL_WHITE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5,  5,  5,  5,  5,  5,  5,  5],
    [1,  1,  2,  4,  4,  2,  1,  1],
    [0,  0,  1,  3,  3,  1,  0,  0],
    [0,  0,  0,  2,  2,  0,  0,  0],
    [0, -0, -1,  0,  0, -1, -0,  0],
    [1,  1,  1, -2, -2,  1,  1,  1],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const PAWN_EVAL_BLACK: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [1,  1,  1, -2, -2,  1,  1,  1],
    [0, -0, -1,  0,  0, -1, -0,  0],
    [0,  0,  0,  2,  2,  0,  0,  0],
    [0,  0,  1,  3,  3,  1,  0,  0],
    [1,  1,  2,  4,  4,  2,  1,  1],
    [5,  5,  5,  5,  5,  5,  5,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const KNIGHT_EVAL: [[i32; 8]; 8] = [
    [-5, -4, -3, -3, -3, -3, -4, -5],
    [-4, -2,  0,  0,  0,  0, -2, -4],
    [-3,  0,  1,  1,  1,  1,  0, -3],
    [-3,  1,  2,  2,  2,  1,  0, -3],
    [-3,  1,  2,  2,  2,  1,  0, -3],
    [-3,  0,  1,  1,  1,  1,  0, -3],
    [-4, -2,  0,  0,  0,  0, -2, -4],
    [-5, -4, -3, -3, -3, -3, -4, -5],
];

const BISHOP_EVAL_WHITE: [[i32; 8]; 8] = [
    [ -4, -2, -2, -2, -2, -2, -2, -4],
    [ -2,  0,  0,  0,  0,  0,  0, -2],
    [ -2,  0,  1,  2,  2,  2,  0, -2],
    [ -2,  1,  1,  2,  2,  2,  1, -2],
    [ -2,  0,  2,  2,  2,  2,  0, -2],
    [ -2,  2,  2,  2,  2,  2,  2, -2],
    [ -2,  1,  0,  0,  0,  0,  1, -2],
    [ -4, -2, -2, -2, -2, -2, -2, -4],
];

const BISHOP_EVAL_BLACK: [[i32; 8]; 8] = [
    [ -4, -2, -2, -2, -2, -2, -2, -4],
    [ -2,  1,  0,  0,  0,  0,  1, -2],
    [ -2,  2,  2,  2,  2,  2,  2, -2],
    [ -2,  0,  2,  2,  2,  2,  0, -2],
    [ -2,  1,  1,  2,  2,  2,  1, -2],
    [ -2,  0,  1,  2,  2,  2,  0, -2],
    [ -2,  0,  0,  0,  0,  0,  0, -2],
    [ -4, -2, -2, -2, -2, -2, -2, -4],
];

const ROOK_EVAL_WHITE: [[i32; 8]; 8] = [
    [  0,  0,  0,  0,  0,  0,  0,  0],
    [  1,  2,  2,  2,  2,  2,  2,  1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [  0,  0,  0,  1,  1,  0,  0,  0],
];

const ROOK_EVAL_BLACK: [[i32; 8]; 8] = [
    [  0,  0,  0,  1,  1,  0,  0,  0],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [ -1,  0,  0,  0,  0,  0,  0, -1],
    [  1,  2,  2,  2,  2,  2,  2,  1],
    [  0,  0,  0,  0,  0,  0,  0,  0],
];

const EVAL_QUEEN: [[i32; 8]; 8] = [
    [ -4, -2, -2, -1, -1, -2, -2, -4],
    [ -2,  0,  0,  0,  0,  0,  0, -2],
    [ -2,  0,  1,  1,  1,  1,  0, -2],
    [ -1,  0,  1,  1,  1,  1,  0, -1],
    [  0,  0,  1,  1,  1,  1,  0, -1],
    [ -2,  1,  1,  1,  1,  1,  0, -2],
    [ -2,  0,  1,  0,  0,  0,  0, -2],
    [ -4, -2, -2, -1, -1, -2, -2, -4],
];

const KING_EVAL_WHITE: [[i32; 8]; 8] = [
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -2, -3, -3, -4, -4, -3, -3, -2],
    [ -1, -2, -2, -2, -2, -2, -2, -1],
    [  2,  2,  0,  0,  0,  0,  2,  2],
    [  2,  3,  1,  0,  0,  1,  3,  2],
];

const KING_EVAL_BLACK: [[i32; 8]; 8] = [
    [  2,  3,  1,  0,  0,  1,  3,  2],
    [  2,  2,  0,  0,  0,  0,  2,  2],
    [ -1, -2, -2, -2, -2, -2, -2, -1],
    [ -2, -3, -3, -4, -4, -3, -3, -2],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
    [ -3, -4, -4, -5, -5, -4, -4, -3],
];

pub fn get_values(pieces: &Pieces) -> i32 {
    let mut total = 0;

    let get_value_from_eval = |pieces: &Pieces, eval: &[[i32; 8]; 8], index: usize| {
        eval[pieces.to_owned().nth(index).unwrap().0.rank().flip_vertical().char() as usize - '1' as usize]
            [pieces.to_owned().nth(index).unwrap().0.file().char() as usize - 'a' as usize]
    };

    for i in 0..pieces.len() {
        // kings
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::King)
            .map(|piece| 
                 if piece.1.color == Color::White { total += 900 + get_value_from_eval(&pieces, &KING_EVAL_WHITE, i) } 
                 else { total -= 900 + get_value_from_eval(&pieces, &KING_EVAL_BLACK, i) });

        // queens
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Queen)
            .map(|piece|
                 if piece.1.color == Color::White { total += 90 + get_value_from_eval(&pieces, &EVAL_QUEEN, i) } 
                 else { total -= 90 + get_value_from_eval(&pieces, &EVAL_QUEEN, i) });

        // rooks
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Rook)
            .map(|piece|
                 if piece.1.color == Color::White { total += 50 + get_value_from_eval(&pieces, &ROOK_EVAL_WHITE, i) } 
                 else { total -= 50 + get_value_from_eval(&pieces, &ROOK_EVAL_BLACK, i) });

        // bishops
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Bishop)
            .map(|piece|
                 if piece.1.color == Color::White { total += 30 + get_value_from_eval(&pieces, &BISHOP_EVAL_WHITE, i) } 
                 else { total -= 30 + get_value_from_eval(&pieces, &BISHOP_EVAL_BLACK, i) });

        // knights
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Knight)
            .map(|piece|
                 if piece.1.color == Color::White { total += 30 + get_value_from_eval(&pieces, &KNIGHT_EVAL, i) } 
                 else { total -= 30 + get_value_from_eval(&pieces, &KNIGHT_EVAL, i) });

        // pawns
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Pawn)
            .map(|piece|
                 if piece.1.color == Color::White { total += 10 + get_value_from_eval(&pieces, &PAWN_EVAL_WHITE, i) } 
                 else { total -= 10 + get_value_from_eval(&pieces, &PAWN_EVAL_BLACK, i) });
    }

    total
}

// special thanks to https://www.freecodecamp.org/news/simple-chess-ai-step-by-step-1d55a9266977/
//
// Recursive function to decide the best move based on the future
// (This does not gives us the *really* best move, it just sieves out the dumb moves
pub fn minimax(depth: u32, game: Chess, mut alpha: i32, mut beta: i32) -> i32 {
    unsafe { POSITION_COUNT += 1; }
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
    unsafe { POSITION_COUNT += 1; }
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

    unsafe { println!("positions evaluated: {}", POSITION_COUNT); POSITION_COUNT = 0; }
    best_move_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_values() {
        let game = Chess::default();
        assert_eq!(get_values(&game.board().pieces()), 0);
    }
}
