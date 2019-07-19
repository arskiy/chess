extern crate shakmaty;

use shakmaty::{Pieces, Role, Color};

pub struct AI {

}

impl AI {
    pub fn new() -> AI {
        AI {}
    }

    pub fn get_values(pieces: &Pieces) -> i32 {
        let mut total = 0;

        // kings
        // let _ = pieces.to_owned().filter(|piece| piece.1.role == Role::King)
            // .map(|piece| if piece.1.color == Color::White {total += 900} else {total -= 900});

        // queens
        // let _ = pieces.to_owned().filter(|piece| piece.1.role == Role::Queen)
            // .map(|piece| if piece.1.color == Color::White {total += 90} else {total -= 90});

        // rooks
        // let _ = pieces.to_owned().filter(|piece| piece.1.role == Role::Rook)
            // .map(|piece| if piece.1.color == Color::White {total += 50} else {total -= 50});
 
        // bishops and knights
        // let _ = pieces.to_owned().filter(|piece| piece.1.role == Role::Bishop || piece.1.role == Role::Knight)
            // .map(|piece| if piece.1.color == Color::White {total += 30} else {total -= 30});

        // pawns
        // let _ = pieces.to_owned().filter(|piece| piece.1.role == Role::Pawn)
            // .map(|piece| if piece.1.color == Color::White {total += 10} else {total -= 10});
        
        for i in 0..pieces.len() {
            match pieces.to_owned().nth(i).unwrap().1.role {
                Role::King => match pieces.to_owned().nth(i).unwrap().1.color {
                    Color::Black => total += 900,
                    Color::White => total -= 900,
                }

                Role::Queen => match pieces.to_owned().nth(i).unwrap().1.color {
                    Color::Black => total += 90,
                    Color::White => total -= 90,
                }

                Role::Rook => match pieces.to_owned().nth(i).unwrap().1.color {
                    Color::Black => total += 50,
                    Color::White => total -= 50,
                }

                Role::Knight | Role::Bishop => match pieces.to_owned().nth(i).unwrap().1.color {
                    Color::Black => total += 30,
                    Color::White => total -= 30,
                }
                
                Role::Pawn => match pieces.to_owned().nth(i).unwrap().1.color {
                    Color::Black => total += 10,
                    Color::White => total -= 10,
                }
            }
        }

        total
    }
}
