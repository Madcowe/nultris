use nultris::{create_frame, create_play_area, render_frame};
use std::io::{self};

fn main() -> io::Result<()> {
    let frame = create_frame(10, 20);
    render_frame(&frame)?;
    let play_area = create_play_area(10, 20, crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 });
    println!("{:?}", play_area);
    Ok(())
}
