extern crate sdl2;
extern crate shakmaty;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::{thread, time};

use shakmaty::{Chess, Role, Setup, Square, Rank, File, Move, Position, Board};

use std::path::Path;
use std::collections::HashSet;

use std::cmp::max;
use std::cmp::min;

mod ai;

const SCR_WIDTH: u32 = 600;

const SQR_SIZE: u32 = SCR_WIDTH / 8;

// temporary for debugging
static mut POSITION_COUNT: u32 = 0;

fn main() -> Result<(), String> {
    // sdl things
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = match video
        .window("Chess", SCR_WIDTH, SCR_WIDTH)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = context.event_pump()?;

    canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    // define standard board
    let mut game = Chess::default();

    // load white pieces' sprites. (This is using FEN notation.)
    // credits for sprites: Wikimedia Commons
    // (https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces)
    let mut w_b: Texture;
    let mut w_k: Texture;
    let mut w_n: Texture;
    let mut w_p: Texture;
    let mut w_q: Texture;
    let mut w_r: Texture;

    // black's
    let mut b_b: Texture;
    let mut b_k: Texture;
    let mut b_n: Texture;
    let mut b_p: Texture;
    let mut b_q: Texture;
    let mut b_r: Texture;

    // completely white texture
    let mut nothing: Texture;

    if cfg!(not(feature = "windows")) {
        // load white pieces' sprites.        
        // credits for sprites: Wikimedia Commons
        // (https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces)
        w_b = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/b_white.png"))?;
        w_k = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/k_white.png"))?;
        w_n = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/n_white.png"))?;
        w_p = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/p_white.png"))?;
        w_q = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/q_white.png"))?;
        w_r = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/r_white.png"))?;

        // black's
        b_b = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/b_black.png"))?;
        b_k = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/k_black.png"))?;
        b_n = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/n_black.png"))?;
        b_p = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/p_black.png"))?;
        b_q = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/q_black.png"))?;
        b_r = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/r_black.png"))?;

        // completely transparent texture
        nothing = texture_creator.load_texture(Path::new("/usr/share/chess.d/sprites/nothing.png"))?;
    } else {
        w_b = texture_creator.load_texture(Path::new("sprites/b_white.png"))?;
        w_k = texture_creator.load_texture(Path::new("sprites/k_white.png"))?;
        w_n = texture_creator.load_texture(Path::new("sprites/n_white.png"))?;
        w_p = texture_creator.load_texture(Path::new("sprites/p_white.png"))?;
        w_q = texture_creator.load_texture(Path::new("sprites/q_white.png"))?;
        w_r = texture_creator.load_texture(Path::new("sprites/r_white.png"))?;

        b_b = texture_creator.load_texture(Path::new("sprites/b_black.png"))?;
        b_k = texture_creator.load_texture(Path::new("sprites/k_black.png"))?;
        b_n = texture_creator.load_texture(Path::new("sprites/n_black.png"))?;
        b_p = texture_creator.load_texture(Path::new("sprites/p_black.png"))?;
        b_q = texture_creator.load_texture(Path::new("sprites/q_black.png"))?;
        b_r = texture_creator.load_texture(Path::new("sprites/r_black.png"))?;

        nothing = texture_creator.load_texture(Path::new("sprites/nothing.png"))?;
    }

    // This will parse and draw all pieces currently on the game to the window.
    let draw_pieces = |canvas: &mut Canvas<Window>, game: &Board| {
        for i in 0..game.pieces().len() {
            match game.pieces().nth(i).unwrap().1.color {
                shakmaty::Color::White =>
                    match game.pieces().nth(i).unwrap().1.role {
                        Role::Pawn   => draw_piece(canvas, &game, &w_p, i),
                        Role::Queen  => draw_piece(canvas, &game, &w_q, i),
                        Role::Bishop => draw_piece(canvas, &game, &w_b, i),
                        Role::Rook   => draw_piece(canvas, &game, &w_r, i),
                        Role::Knight => draw_piece(canvas, &game, &w_n, i),
                        Role::King   => draw_piece(canvas, &game, &w_k, i),
                    },
                shakmaty::Color::Black =>
                    match game.pieces().nth(i).unwrap().1.role {
                        Role::Pawn   => draw_piece(canvas, &game, &b_p, i),
                        Role::Queen  => draw_piece(canvas, &game, &b_q, i),
                        Role::Bishop => draw_piece(canvas, &game, &b_b, i),
                        Role::Rook   => draw_piece(canvas, &game, &b_r, i),
                        Role::Knight => draw_piece(canvas, &game, &b_n, i),
                        Role::King   => draw_piece(canvas, &game, &b_k, i),
                    },
            }
        }
    };

    // We need to set this before the render loop to avoid undefined behaviour,
    // so we just set an arbritary texture to this by now.
    let mut curr_texture: &Texture = &nothing;

    let mut curr_click_pos: Square;

    // arbitrary to avoid undefined behaviour
    let mut prev_click_pos: Square = Square::A1;

    let mut prev_role_click: Role = Role::Pawn;
    let mut curr_role_click: Option<Role>;

    let mut prev_mouse_buttons = HashSet::new();

    'render_loop: loop {
        for event in events.poll_iter() {
            // if esc is pressed, exit main loop
            // (consequently ending the program)
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'render_loop,
                _ => {}
            }
        }

        if let Some(outcome) = game.outcome() {
            match outcome.winner() {
                Some(color) =>
                    if color == shakmaty::Color::Black {
                        println!("You lost.");
                        break 'render_loop;
                    }
                    else {
                        println!("You won! Congratulations!!!");
                        break 'render_loop;
                    }

                None => { println!("Draw!"); break 'render_loop; }
            }
        }

        let mouse_state = events.mouse_state();
        let curr_mouse_buttons: HashSet<_> = mouse_state.pressed_mouse_buttons().collect();


        canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0xFF, 0xCE, 0x9E));
        draw_grid(&mut canvas);

        draw_check(&game, &mut canvas);

        draw_pieces(&mut canvas, game.board());

        // AI

        if game.turn() == shakmaty::Color::Black {
            game = game.to_owned().play(&minimax_root(3, game)).unwrap();
            unsafe { println!("Positions evaluated: {}", POSITION_COUNT); }
            unsafe { POSITION_COUNT = 0; }
        }

        // Abandon all hope, ye who enter here.
        // while a mouse button is pressed, it will fall into this conditional
        let get_texture = |game: &Board| {
            // TODO: use filter here
            // match is more readable than if let.     
            match game.color_at(Square::from_coords(
                    File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                    Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical())) {
                Some(color) => if color == shakmaty::Color::White {
                    match game.role_at(Square::from_coords(
                            File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                            Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical())) {
                        Some(role) => match role {
                            Role::King   => &w_k,
                            Role::Knight => &w_n,
                            Role::Rook   => &w_r,
                            Role::Bishop => &w_b,
                            Role::Queen  => &w_q,
                            Role::Pawn   => &w_p,
                        }

                        None => &nothing,
                    }
                }
                else { &nothing }

                None => &nothing,
            }
        };

        // necessary to make the borrow checker happy.
        if curr_mouse_buttons.is_empty() {
            curr_texture = get_texture(game.board());
        }

        if game.turn() == shakmaty::Color::White {
            let is_mouse_released = &prev_mouse_buttons - &curr_mouse_buttons;
            if !is_mouse_released.is_empty() {
                curr_role_click = game.board().role_at(Square::from_coords(
                        File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                        Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical()));
                curr_click_pos = Square::from_coords(File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical());
                match game.to_owned().play(&Move::Normal {
                    role: prev_role_click,
                    from: prev_click_pos,
                    to: curr_click_pos,
                    capture: curr_role_click,
                    promotion: None}) {

                    Ok(game_wrap) => game = game_wrap,

                    Err(_) => draw_error(((curr_click_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32,
                    ((curr_click_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE) as i32,
                    &mut canvas),
                }
            }
        }

        if curr_mouse_buttons.is_empty() {
            prev_role_click = game.board().role_at(Square::from_coords(
                    File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                    Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical())).unwrap_or(Role::Knight);

            prev_click_pos = Square::from_coords(File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
            Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical());
        } else {

            canvas.copy(curr_texture, None, Rect::new(
                    (mouse_state.x() / SQR_SIZE as i32) * SQR_SIZE as i32,
                    (mouse_state.y() / SQR_SIZE as i32) * SQR_SIZE as i32,
                    SQR_SIZE,
                    SQR_SIZE))?;
        }

        canvas.present();

        prev_mouse_buttons = curr_mouse_buttons;

        // if you don't do this cpu usage will skyrocket to 100%
        events.wait_event_timeout(10);
        // events.poll_event();
    }


    Ok(())
}

//-----------------------------------------------------------------------------------

fn draw_piece(canvas: &mut Canvas<Window>, game: &Board, texture: &Texture, i: usize) {
    canvas
        .copy(
            texture,
            None,
            Rect::new(
                ((game.pieces().nth(i).unwrap().0.file().char() as u32 - 'a' as u32)
                 * SQR_SIZE) as i32,
                 ((game.pieces().nth(i).unwrap().0.rank().flip_vertical().char() as u32 - '1' as u32)
                  * SQR_SIZE) as i32,
                  SQR_SIZE,
                  SQR_SIZE,
                  )).unwrap();
}

// from: https://www.libsdl.org/tmp/SDL/test/testdrawchessboard.c
fn draw_grid(canvas: &mut Canvas<Window>) {
    let mut row = 0;

    while row < 8 {
        let mut x = row % 2;

        for _ in (row % 2)..(4 + (row % 2)) {
            let rect = Rect::new(
                x * SQR_SIZE as i32,
                row * SQR_SIZE as i32,
                SQR_SIZE,
                SQR_SIZE,
                );
            x += 2;

            let _ = canvas.fill_rect(rect);
        }

        row += 1;
    }
}

//----------------------------------------------------------------
// TODO: make this actually work as expected

fn draw_error(x: i32, y: i32, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 5, 5));
    let _ = canvas.fill_rect(Rect::new(
            x,
            y,
            SQR_SIZE,
            SQR_SIZE));
    thread::sleep(time::Duration::from_millis(100));
}

fn draw_check(game: &Chess, canvas: &mut Canvas<Window>) {
    let pieces = game.board().pieces();
    let mut white_king_pos: Square = Square::new(0);
    let mut black_king_pos: Square = Square::new(0);

    for i in 0..pieces.len() {
        pieces.to_owned().nth(i).filter(|piece| piece.1.role == Role::King).map(|piece| if piece.1.color == shakmaty::Color::White {
            white_king_pos = piece.to_owned().0;
        }
        else {
            black_king_pos = piece.to_owned().0;
        });
    }

    if game.is_check() {
        let x: i32;
        let y: i32;

        if game.turn() == shakmaty::Color::White {
            x = ((white_king_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32;
            y = ((white_king_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE) as i32;
        }
        else {
            x = ((black_king_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32;
            y = ((black_king_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE) as i32;
        }

        canvas.set_draw_color(Color::RGB(255, 5, 5));
        let _ = canvas.fill_rect(Rect::new(
                x,
                y,
                SQR_SIZE,
                SQR_SIZE));
    }
}

fn minimax(depth: u32, game: Chess, mut alpha: i32, mut beta: i32) -> i32 {
    // this is actually safe since we aren't using concurrency
    unsafe { POSITION_COUNT += 1; }
    if depth == 0 {
        return -ai::get_values(&game.board().pieces());
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

fn minimax_root(depth: u32, game: Chess) -> Move {
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
