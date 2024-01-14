use nultris::{create_frame, render_frame};
use std::io::{self};

fn main() -> io::Result<()> {
    let frame = create_frame(10, 20);
    render_frame(&frame)?;
    Ok(())
}
