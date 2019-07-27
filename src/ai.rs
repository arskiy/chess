extern crate shakmaty;

use shakmaty::{Pieces, Role, Color, Chess, Move, Position, Setup};

use std::cmp::max;
use std::cmp::min;


pub fn get_values(pieces: &mut Pieces) -> i32 {

    // simplified evaluation arrays
    let pawn_eval_white: Vec<Vec<i32>> = vec!{
        vec![0,  0,  0,  0,  0,  0,  0,  0],
        vec![5,  5,  5,  5,  5,  5,  5,  5],
        vec![1,  1,  2,  4,  4,  2,  1,  1],
        vec![0,  0,  1,  3,  3,  1,  0,  0],
        vec![0,  0,  0,  2,  2,  0,  0,  0],
        vec![0, -0, -1,  0,  0, -1, -0,  0],
        vec![1,  1,  1, -2, -2,  1,  1,  1],
        vec![0,  0,  0,  0,  0,  0,  0,  0],
    };

    let pawn_eval_black: Vec<Vec<i32>> = reverse_array(&pawn_eval_white);

    let knight_eval: Vec<Vec<i32>> = vec!{
        vec![-5, -4, -3, -3, -3, -3, -4, -5],
        vec![-4, -2,  0,  0,  0,  0, -2, -4],
        vec![-3,  0,  1,  1,  1,  1,  0, -3],
        vec![-3,  1,  2,  2,  2,  1,  0, -3],
        vec![-3,  1,  2,  2,  2,  1,  0, -3],
        vec![-3,  0,  1,  1,  1,  1,  0, -3],
        vec![-4, -2,  0,  0,  0,  0, -2, -4],
        vec![-5, -4, -3, -3, -3, -3, -4, -5],
    };

    let bishop_eval_white: Vec<Vec<i32>> = vec!{
        vec![ -4, -2, -2, -2, -2, -2, -2, -4],
        vec![ -2,  0,  0,  0,  0,  0,  0, -2],
        vec![ -2,  0,  1,  2,  2,  2,  0, -2],
        vec![ -2,  1,  1,  2,  2,  2,  1, -2],
        vec![ -2,  0,  2,  2,  2,  2,  0, -2],
        vec![ -2,  2,  2,  2,  2,  2,  2, -2],
        vec![ -2,  1,  0,  0,  0,  0,  1, -2],
        vec![ -4, -2, -2, -2, -2, -2, -2, -4],
    };

    let bishop_eval_black: Vec<Vec<i32>> = reverse_array(&bishop_eval_white);

    let rook_eval_white: Vec<Vec<i32>> = vec!{
        vec![  0,  0,  0,  0,  0,  0,  0,  0],
        vec![  1,  2,  2,  2,  2,  2,  2,  1],
        vec![ -1,  0,  0,  0,  0,  0,  0, -1],
        vec![ -1,  0,  0,  0,  0,  0,  0, -1],
        vec![ -1,  0,  0,  0,  0,  0,  0, -1],
        vec![ -1,  0,  0,  0,  0,  0,  0, -1],
        vec![ -1,  0,  0,  0,  0,  0,  0, -1],
        vec![  0,  0,  0,  1,  1,  0,  0,  0],
    };

    let rook_eval_black: Vec<Vec<i32>> = reverse_array(&rook_eval_white);

    let eval_queen: Vec<Vec<i32>> = vec!{
        vec![ -4, -2, -2, -1, -1, -2, -2, -4],
        vec![ -2,  0,  0,  0,  0,  0,  0, -2],
        vec![ -2,  0,  1,  1,  1,  1,  0, -2],
        vec![ -1,  0,  1,  1,  1,  1,  0, -1],
        vec![  0,  0,  1,  1,  1,  1,  0, -1],
        vec![ -2,  1,  1,  1,  1,  1,  0, -2],
        vec![ -2,  0,  1,  0,  0,  0,  0, -2],
        vec![ -4, -2, -2, -1, -1, -2, -2, -4],
    };

    let king_eval_white: Vec<Vec<i32>> = vec!{
        vec![ -3, -4, -4, -5, -5, -4, -4, -3],
        vec![ -3, -4, -4, -5, -5, -4, -4, -3],
        vec![ -3, -4, -4, -5, -5, -4, -4, -3],
        vec![ -3, -4, -4, -5, -5, -4, -4, -3],
        vec![ -2, -3, -3, -4, -4, -3, -3, -2],
        vec![ -1, -2, -2, -2, -2, -2, -2, -1],
        vec![  2,  2,  0,  0,  0,  0,  2,  2],
        vec![  2,  3,  1,  0,  0,  1,  3,  2],
    };

    let king_eval_black: Vec<Vec<i32>> = reverse_array(&king_eval_white);
    let mut total = 0;

    let get_value_from_eval = |pieces: &Pieces, eval: &Vec<Vec<i32>>, index: usize| {
        eval[pieces.to_owned().nth(index).unwrap().0.rank().flip_vertical().char() as usize - '1' as usize]
            [pieces.to_owned().nth(index).unwrap().0.file().char() as usize - 'a' as usize]
    };

    for i in 0..pieces.len() {
        // kings
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::King)
            .map(|piece| 
                 if piece.1.color == Color::White { total += 900 + get_value_from_eval(&pieces, &king_eval_white, i) } 
                 else { total -= 900 + get_value_from_eval(&pieces, &king_eval_black, i) });

        // queens
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Queen)
            .map(|piece|
                 if piece.1.color == Color::White { total += 90 + get_value_from_eval(&pieces, &eval_queen, i) } 
                 else { total -= 90 + get_value_from_eval(&pieces, &eval_queen, i) });

        // rooks
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Rook)
            .map(|piece|
                 if piece.1.color == Color::White { total += 50 + get_value_from_eval(&pieces, &rook_eval_white, i) } 
                 else { total -= 50 + get_value_from_eval(&pieces, &rook_eval_black, i) });

        // bishops
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Bishop)
            .map(|piece|
                 if piece.1.color == Color::White { total += 30 + get_value_from_eval(&pieces, &bishop_eval_white, i) } 
                 else { total -= 30 + get_value_from_eval(&pieces, &bishop_eval_black, i) });

        // knights
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Knight)
            .map(|piece|
                 if piece.1.color == Color::White { total += 30 + get_value_from_eval(&pieces, &knight_eval, i) } 
                 else { total -= 30 + get_value_from_eval(&pieces, &knight_eval, i) });

        // pawns
        let _ = pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::Pawn)
            .map(|piece|
                 if piece.1.color == Color::White { total += 10 + get_value_from_eval(&pieces, &pawn_eval_white, i) } 
                 else { total -= 10 + get_value_from_eval(&pieces, &pawn_eval_black, i) });
    }

    total
}

// special thanks to https://www.freecodecamp.org/news/simple-chess-ai-step-by-step-1d55a9266977/
//
// Recursive function to decide the best move based on the future
// (This does not gives us the *really* best move, it just sieves out the dumb moves
pub fn minimax(depth: u32, game: Chess, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return -get_values(&mut game.board().pieces());
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

fn reverse_array(arr: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut final_vec: Vec<Vec<i32>> = Vec::new();

    for i in 0..8 {
        let mut temp_vec: Vec<i32> = Vec::new();

        arr.iter().
            flatten().
            rev().
            cloned().
            skip(i * 8).
            take(8).
            for_each(|x| temp_vec.push(x));

        final_vec.push(temp_vec);
    }

    final_vec
}
