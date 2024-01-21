use nultris::*;
use std::io::{self};

fn main() -> io::Result<()> {
    let play_area = create_play_area(10, 20, crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 });
    let pieces = create_pieces();
    let current_piece = create_current_piece(&pieces);
    let frame = create_frame(&play_area, &current_piece);
    // println!("{:?}", play_area);
    render_frame(&frame)?;
    Ok(())
}
