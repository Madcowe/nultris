// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Print, SetForegroundColor, Stylize},
    terminal,
};
use rand::prelude::*;
use std::io::{self, Write};
use std::{thread, time};

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

pub fn main_loop() -> io::Result<()> {
    // setup, maybe move to own funciton later
    let play_area = create_play_area(10, 20, crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 });
    let pieces = create_pieces();
    let mut current_piece = create_current_piece(&pieces);
    let mut legal_move = true;
    let frame = create_frame(&play_area, &current_piece);

    // terminal::enable_raw_mode()?;
    render_frame(&frame)?;

    while legal_move == true {
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        legal_move = move_current_piece(
            current_piece.x,
            current_piece.y + 1,
            &play_area,
            &mut current_piece,
        );
        println!("{:}", legal_move);
        let frame = create_frame(&play_area, &current_piece);
        render_frame(&frame)?;
    }
    // terminal::disable_raw_mode()?;
    Ok(())
}

pub fn create_pieces() -> Vec<Piece> {
    let mut pieces = Vec::new();

    let mut shapes = Vec::new();
    let shape: [[u8; 4]; 4] = [[1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    let piece: Piece = Piece {
        x: 4,
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

pub fn move_current_piece(
    x: usize,
    y: usize,
    play_area: &Vec<Vec<Bloxel>>,
    current_piece: &mut Piece,
) -> bool {
    let mut legal_move = true;
    let max_x = play_area.len();
    let max_y = play_area[0].len();
    {
        let (mut x, mut y) = (x, y);
        let shape = current_piece.shapes[current_piece.orientation];
        for column in shape {
            for occupied in column {
                if ((x >= max_x || y >= max_y) && occupied > 0)
                    || (occupied > 0 && play_area[x][y].occupied == true)
                {
                    // if ocupied co-ordinate outisde play_area or both play area and shape
                    // occupied, move is not legal leave loop as no need to check rest.
                    legal_move = false;
                    break;
                }
                y += 1;
            }
            if legal_move == false {
                break;
            }
            y = current_piece.y;
            x += 1;
        }
    }
    if legal_move {
        current_piece.x = x;
        current_piece.y = y;
    }
    legal_move
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

pub fn create_frame(play_area: &Vec<Vec<Bloxel>>, current_piece: &Piece) -> Vec<Vec<Color>> {
    let mut frame = Vec::new();

    for x in 0..play_area.len() {
        let mut column = Vec::new();
        for y in 0..play_area[x].len() {
            column.push(play_area[x][y].color);
        }
        frame.push(column);
    }

    // note this just renders the shape it doesn't check for collision as will be handled beforehand
    let max_x = frame.len();
    let max_y = frame[0].len();
    let (mut x, mut y, color) = (current_piece.x, current_piece.y, current_piece.color);
    let shape = current_piece.shapes[current_piece.orientation];
    for column in shape {
        if x < max_x {
            // if x co-oridnate is wihtin frame
            for occupied in column {
                if y < max_y && occupied > 0 {
                    // if y co-ordinate wihtin frame and occupied
                    frame[x][y] = color;
                }
                y += 1;
            }
        }
        y = current_piece.y;
        x += 1;
    }

    // frame[0][2] = Color::Green;

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
