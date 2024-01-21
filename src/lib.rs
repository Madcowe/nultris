// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Print, SetForegroundColor, Stylize},
    terminal,
};
use rand::prelude::*;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Bloxel {
    occupied: bool,
    color: Color,
}

#[derive(Debug, Clone)]
pub struct Piece {
    x: usize,
    y: usize,
    color: Color,
    shapes: Vec<[[u8; 4]; 4]>,
    orientation: usize,
}

pub fn create_pieces() -> Vec<Piece> {
    let mut pieces = Vec::new();

    let mut shapes = Vec::new();
    let shape: [[u8; 4]; 4] = [[1, 1, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    let piece: Piece = Piece {
        x: 0,
        y: 0,
        color: Color::Rgb { r: 255, g: 0, b: 0 },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    pieces
}

pub fn create_current_piece(pieces: &Vec<Piece>) -> Piece {
    // let mut rng = thread_rng();
    // let len = pieces.len() - 1;
    // let i = rng.gen_range(0..len);
    pieces[0].clone()
}

pub fn create_play_area(x: u16, y: u16, bg_color: Color) -> Vec<Vec<Bloxel>> {
    let mut play_area = Vec::new();

    for _x in 0..x {
        let mut row = Vec::new();
        for _y in 0..y {
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

pub fn create_frame(play_area: &Vec<Vec<Bloxel>>, current_shape: &Piece) -> Vec<Vec<Color>> {
    let mut frame = Vec::new();

    for x in 0..play_area.len() {
        let mut row = Vec::new();
        for y in 0..play_area[x].len() {
            row.push(play_area[x][y].color);
        }
        frame.push(row);
    }

    // note this just renders the shape it doesn't check for collision as will be handled beforehand
    let max_x = frame.len();
    let max_y = frame[0].len();
    let (mut x, mut y, color) = (current_shape.x, current_shape.y, current_shape.color);
    let shape = current_shape.shapes[current_shape.orientation];
    for row in shape {
        if x <= max_x {
            // if x co-oridnate is wihtin frame
            for occupied in row {
                println!("{},{}", x, y);
                if y <= max_y && occupied > 0 {
                    // if y co-ordinate wihtin frame and occupied
                    frame[x][y] = color;
                }
                y += 1;
            }
        }
        x += 1;
    }

    frame[9][10] = Color::Green;

    frame
}

pub fn render_frame(frame: &Vec<Vec<Color>>) -> io::Result<()> {
    // should check and throw error if terminal is smaller than frame
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for x in 0..frame.len() {
        let row = &frame[x];
        for y in 0..row.len() {
            let color = row[y];
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_random() {
//         let r: u8 = random();
//         assert!(r > 1);
//     }
// }
