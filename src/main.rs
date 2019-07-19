extern crate sdl2;
extern crate shakmaty;
extern crate rand;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use rand::Rng;

use shakmaty::{Chess, Role, Setup, Square, Rank, File, Move, Position, Board};

use std::path::Path;
use std::collections::HashSet;

mod ai;
use ai::AI;

const SCR_WIDTH: u32 = 600;

const SQR_SIZE: u32 = SCR_WIDTH / 8;

fn main() -> Result<(), String> {
    // sdl things
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let minimax_ai = AI::new();
    
    let mut rng = rand::thread_rng();

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
    let mut board = Chess::default();

    // load white pieces' sprites. (This is using FEN notation.)
    // credits for sprites: Wikimedia Commons
    // (https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces)
    let w_b = texture_creator.load_texture(Path::new("sprites/B.png"))?;
    let w_k = texture_creator.load_texture(Path::new("sprites/K.png"))?;
    let w_n = texture_creator.load_texture(Path::new("sprites/N.png"))?;
    let w_p = texture_creator.load_texture(Path::new("sprites/P.png"))?;
    let w_q = texture_creator.load_texture(Path::new("sprites/Q.png"))?;
    let w_r = texture_creator.load_texture(Path::new("sprites/R.png"))?;

    // black's
    let b_b = texture_creator.load_texture(Path::new("sprites/b.png"))?;
    let b_k = texture_creator.load_texture(Path::new("sprites/k.png"))?;
    let b_n = texture_creator.load_texture(Path::new("sprites/n.png"))?;
    let b_p = texture_creator.load_texture(Path::new("sprites/p.png"))?;
    let b_q = texture_creator.load_texture(Path::new("sprites/q.png"))?;
    let b_r = texture_creator.load_texture(Path::new("sprites/r.png"))?;

    // completely transparent texture
    let nothing = texture_creator.load_texture(Path::new("sprites/nothing.png"))?;

    let mut prev_count: i32 = 0;

    // This will parse and draw all pieces currently on the game to the window.
    let draw_pieces = |canvas: &mut Canvas<Window>, board: &Board| {
        for i in 0..board.pieces().len() {
            match board.pieces().nth(i).unwrap().1.color {
                shakmaty::Color::White =>
                    match board.pieces().nth(i).unwrap().1.role {
                        Role::Pawn   => draw_piece(canvas, &board, &w_p, i),
                        Role::Queen  => draw_piece(canvas, &board, &w_q, i),
                        Role::Bishop => draw_piece(canvas, &board, &w_b, i),
                        Role::Rook   => draw_piece(canvas, &board, &w_r, i),
                        Role::Knight => draw_piece(canvas, &board, &w_n, i),
                        Role::King   => draw_piece(canvas, &board, &w_k, i),
                    },
                shakmaty::Color::Black =>
                    match board.pieces().nth(i).unwrap().1.role {
                        Role::Pawn   => draw_piece(canvas, &board, &b_p, i),
                        Role::Queen  => draw_piece(canvas, &board, &b_q, i),
                        Role::Bishop => draw_piece(canvas, &board, &b_b, i),
                        Role::Rook   => draw_piece(canvas, &board, &b_r, i),
                        Role::Knight => draw_piece(canvas, &board, &b_n, i),
                        Role::King   => draw_piece(canvas, &board, &b_k, i),
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
    let mut curr_role_click: Option<Role> = None;

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

        let mouse_state = events.mouse_state();
        let curr_mouse_buttons: HashSet<_> = mouse_state.pressed_mouse_buttons().collect();


        canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0xFF, 0xCE, 0x9E));
        draw_grid(&mut canvas);

        draw_pieces(&mut canvas, board.board());

        // highest possible value
        let mut best_count: i32 = prev_count;

        // AI
        if board.turn() == shakmaty::Color::Black {
            let mut best_board: Chess = board.clone();


            for movement in 0..board.legals().len() {
                let new_board = board.to_owned().play(&board.legals()[movement]).unwrap();
                let count = ai::AI::get_values(&new_board.board().pieces());

                println!("mov: {}, current: {}, best: {}", movement, count, best_count);

                // the lesser, the better
                if best_count > count {
                    best_count = count;
                    best_board = new_board;
                }
            }

            if best_count == prev_count {
                best_board = best_board.to_owned().play(&best_board.legals()
                                             [rng.gen_range(0, best_board.legals().len())])
                                             .unwrap();
                println!("random move");
            }

            board = best_board;
            prev_count = best_count;
        }

        // Abandon all hope, ye who enter here.
        // while a mouse button is pressed, it will fall into this conditional
        let get_texture = |board: &Board| {
            // TODO: use filter here
            // match is more readable than if let.     
            match board.color_at(Square::from_coords(
                    File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                    Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical())) {
                Some(color) => if color == shakmaty::Color::White {
                    match board.role_at(Square::from_coords(
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
            curr_texture = get_texture(board.board());
        }

        if board.turn() == shakmaty::Color::White {
            let is_mouse_released = &prev_mouse_buttons - &curr_mouse_buttons;
            if !is_mouse_released.is_empty() {
                curr_role_click = board.board().role_at(Square::from_coords(
                    File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                    Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical()));
                curr_click_pos = Square::from_coords(File::new((mouse_state.x() / SQR_SIZE as i32) as u32),
                Rank::new((mouse_state.y() / SQR_SIZE as i32) as u32).flip_vertical());
                match board.to_owned().play(&Move::Normal {
                    role: prev_role_click,
                    from: prev_click_pos,
                    to: curr_click_pos,
                    capture: curr_role_click,
                    promotion: None}) {

                    Ok(board_wrap) => board = board_wrap,

                    Err(_) => draw_error(((curr_click_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32,
                    ((curr_click_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE) as i32,
                    &mut canvas,
                    0),
                }
            }
        }

        if curr_mouse_buttons.is_empty() {
            prev_role_click = board.board().role_at(Square::from_coords(
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

fn draw_piece(canvas: &mut Canvas<Window>, board: &Board, texture: &Texture, i: usize) {
    canvas
        .copy(
            texture,
            None,
            Rect::new(
                ((board.pieces().nth(i).unwrap().0.file().char() as u32 - 'a' as u32)
                 * SQR_SIZE) as i32,
                 ((board.pieces().nth(i).unwrap().0.rank().flip_vertical().char() as u32 - '1' as u32)
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

fn draw_error(x: i32, y: i32, canvas: &mut Canvas<Window>, counter: u8) {
    // if counter == 255 {
    // return
    // }

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let _ = canvas.fill_rect(Rect::new(
            x,
            y,
            SQR_SIZE,
            SQR_SIZE));
    // println!("X: {} | Y: {} | {}", x, y, counter);

    // std::thread::sleep(std::time::Duration::from_millis(2));
    // draw_error(x, y, canvas, counter + 1);
}
