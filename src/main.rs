extern crate sdl2;
extern crate shakmaty;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use shakmaty::{Chess, Role, Setup};

use std::path::Path;

const SCR_WIDTH: u32 = 600;

const SQR_SIZE: u32 = SCR_WIDTH / 8;

fn main() -> Result<(), String> {
    // sdl things
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = match video
        .window("Chess", SCR_WIDTH, SCR_WIDTH)
        .position_centered()
        .opengl()
        .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = context.event_pump()?;

    canvas.set_draw_color(Color::RGB(0xFF, 0xCE, 0x9E));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    // define standard board
    let board = Chess::default();

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

    // Abandon all hope, ye who enter here.
    // This will parse and draw all pieces currently on the game to the window.
    let draw_pieces = |canvas: &mut Canvas<Window>| {
        for i in 0..board.board().pieces().len() {
            match board.board().pieces().nth(i).unwrap().1.color {
                shakmaty::Color::White =>
                    match board.board().pieces().nth(i).unwrap().1.role {
                        Role::Pawn   => draw_piece(canvas, &board, &w_p, i),
                        Role::Queen  => draw_piece(canvas, &board, &w_q, i),
                        Role::Bishop => draw_piece(canvas, &board, &w_b, i),
                        Role::Rook   => draw_piece(canvas, &board, &w_r, i),
                        Role::Knight => draw_piece(canvas, &board, &w_n, i),
                        Role::King   => draw_piece(canvas, &board, &w_k, i),
                    },
                shakmaty::Color::Black =>
                    match board.board().pieces().nth(i).unwrap().1.role {
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


    // debug
    println!("{:#?}", board.board());
    println!("{:?}", board.board().pieces().len());

    /*
       for i in 0..30 {
       println!("{:?}", board.board().pieces().nth(i));

       println!("{:?}", board.board().pieces().nth(i).unwrap().0.file().char() as u32 - 'a' as u32);
       println!("{:?}", board.board().pieces().nth(i).unwrap().1.color);

       println!("{:?}", board.board().pieces().nth(i).unwrap().0.rank().char() as u32 - '1' as u32);
       println!("{:?}", board.board().pieces().nth(i).unwrap().0.rank().char());

       println!("{:#?}", board.turn());
       }*/

    canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
    draw_grid(&mut canvas);

    draw_pieces(&mut canvas);

    canvas.present();

    'render_loop: loop {
        for event in event_pump.poll_iter() {
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

        // if you don't do this cpu usage will skyrocket to 100%
        event_pump.wait_event_timeout(10);
        // event_pump.poll_event();
    }

    Ok(())
}

//-----------------------------------------------------------------------------------

fn draw_piece(canvas: &mut Canvas<Window>, board: &Chess, texture: &Texture, i: usize) {
    canvas
        .copy(
            texture,
            None,
            Rect::new(
                ((board.board().pieces().nth(i).unwrap().0.file().char() as u32 - 'a' as u32)
                 * SQR_SIZE) as i32,
                 ((board
                   .board()
                   .pieces()
                   .nth(i)
                   .unwrap()
                   .0
                   .rank()
                   .flip_vertical()
                   .char() as u32
                   - '1' as u32) * SQR_SIZE) as i32,
                  SQR_SIZE,
                  SQR_SIZE,
                  )).unwrap();
}

// ----------------------------------------------------------------------------------

// from: https://www.libsdl.org/tmp/SDL/test/testdrawchessboard.c
fn draw_grid(canvas: &mut Canvas<Window>) {
    let mut row = 0;

    while row < 9 {
        let mut x = row % 2 - 1;

        for _ in (row % 2)..(5 + (row % 2)) {
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

//------------------------------------------------------
// white = capitalized letters
//
// example board:
// r n b q k b n r
// p p p p p p p p
// . . . . . . . .
// . . . . . . . .
// . . . . P . . .
// . . . . . . . .
// P P P P . P P P
// R N B Q K B N R
