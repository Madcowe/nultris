// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Print, SetForegroundColor, Stylize},
    terminal,
};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Bloxel {
    occupied: bool,
    color: Color,
}

pub fn create_play_area(x: u16, y: u16, bg_color: Color) -> Vec<Vec<Bloxel>> {
    let mut play_area = Vec::new();

    for _y in 0..y {
        let mut row = Vec::new();
        for _x in 0..x {
            let bloxel = Bloxel {
                occupied: false,
                color: bg_color,
            };
            row.push(bloxel);
        }
        play_area.push(row);
    }
    play_area
}

pub fn create_frame(x: u16, y: u16) -> Vec<Vec<Color>> {
    let mut frame = Vec::new();
    let mut color = Color::Rgb { r: 255, g: 0, b: 0 };

    for _y in 0..y {
        let mut row = Vec::new();
        for _x in 0..x {
            row.push(color);
        }
        frame.push(row);
        if color == (Color::Rgb { r: 255, g: 0, b: 0 }) {
            color = Color::Rgb { r: 0, g: 0, b: 255 };
        } else {
            color = Color::Rgb { r: 255, g: 0, b: 0 };
        }
    }
    frame
}

pub fn render_frame(frame: &Vec<Vec<Color>>) -> io::Result<()> {
    // should check and throw error if terminal is smaller than framei
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for y in 0..frame.len() {
        let row = &frame[y];
        for x in 0..row.len() {
            let color = row[x];
            let x = x as u16 * 2; // as inserting double blocks for squares
            queue!(
                stdout,
                cursor::MoveTo(x, y as u16),
                SetForegroundColor(color),
                Print("██".to_string())
            )?;
        }
    }

    stdout.flush()?;

    Ok(())
}
