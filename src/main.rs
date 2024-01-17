use nultris::{create_frame, create_play_area, render_frame};
use std::io::{self};

fn main() -> io::Result<()> {
    let play_area = create_play_area(10, 20, crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 });
    let frame = create_frame(play_area);
    // println!("{:?}", play_area);
    render_frame(&frame)?;
    Ok(())
}
