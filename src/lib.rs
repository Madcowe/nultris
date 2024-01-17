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

pub fn create_frame(play_area: Vec<Vec<Bloxel>>) -> Vec<Vec<Color>> {
    let mut frame = Vec::new();

    for x in 0..play_area.len() {
        let mut row = Vec::new();
        for y in 0..play_area[x].len() {
            row.push(play_area[x][y].color);
            println!("{} {}", x, y);
        }
        frame.push(row);
    }

    // add code to render current shape

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
