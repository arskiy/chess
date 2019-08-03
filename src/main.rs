pub mod ai;
pub mod chess;
pub mod emscripten_file;

fn main() -> Result<(), String> {
    // let's do this!
    chess::init()?;

    Ok(())
}
