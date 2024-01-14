// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn create_frame(x: u32, y: u32) -> Vec<Vec<Colour>> {
    let mut frame = Vec::new();

    for _i in 0..x {
        let mut row = Vec::new();
        for _y in 0..y {
            let colour = Colour {
                red: 255,
                blue: 0,
                green: 0,
            };
            row.push(colour);
        }
        frame.push(row);
    }
    frame
}

pub fn render_frame(frame: &Vec<Vec<Colour>>) -> io::Result<()> {
    // should check and throw error if terminal is smaller than framei
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for x in 0..frame.len() {
        let row = &frame[x];
        for y in 0..row.len() {
            queue!(
                stdout,
                cursor::MoveTo(x as u16, y as u16),
                style::PrintStyledContent("█".blue())
            )?;
        }
    }

    Ok(())
}
