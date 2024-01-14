// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Print, SetForegroundColor, Stylize},
    terminal,
};
use std::io::{self, Write};

pub fn create_frame(x: u16, y: u16) -> Vec<Vec<Color>> {
    let mut frame = Vec::new();

    for _i in 0..x {
        let mut row = Vec::new();
        for _y in 0..y {
            let colour = Color::Rgb { r: 255, g: 0, b: 0 };
            row.push(colour);
        }
        frame.push(row);
    }
    frame
}

pub fn render_frame(frame: &Vec<Vec<Color>>) -> io::Result<()> {
    // should check and throw error if terminal is smaller than framei
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for x in 0..frame.len() {
        let row = &frame[x];
        for y in 0..row.len() {
            let color = row[y];
            queue!(
                stdout,
                cursor::MoveTo(x as u16, y as u16),
                SetForegroundColor(color),
                Print("█".to_string()) // style::PrintStyledContent("█".magenta())
            )?;
        }
    }

    stdout.flush()?;

    Ok(())
}
