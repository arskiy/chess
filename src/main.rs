mod chess;
mod ai;

fn main() -> Result<(), String> {
    // let's do this!
    chess::init()?;

    Ok(())
}
