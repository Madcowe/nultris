// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::{self, Color, Print, SetForegroundColor, Stylize},
    terminal,
};
use rand::prelude::*;
use std::{
    env::current_dir,
    io::{self, Write},
};
use std::{thread, time};

#[derive(PartialEq, Debug)]
enum NextGameAction {
    Move,
    NewPiece,
    GameOver,
    Quit,
}

#[derive(Debug)]
struct Bloxel {
    occupied: bool,
    color: Color,
}

#[derive(Debug, Clone)]
struct Piece {
    x: usize,
    y: usize,
    color: Color,
    shapes: Vec<[[u8; 4]; 4]>,
    orientation: usize,
}

pub fn main_loop() -> io::Result<()> {
    // setup, maybe move to own funciton later
    let mut play_area = create_play_area(10, 20, crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 });
    let pieces = create_pieces();
    let (mut current_piece, mut next_game_action) = create_current_piece(&play_area, &pieces);
    terminal::enable_raw_mode()?;
    let delay = time::Duration::from_millis(250);

    // When quit button is pressed quit the game
    loop {
        let frame = create_frame(&play_area, &current_piece);
        render_frame(&frame)?;
        if poll(delay)? {
            if let Event::Key(event) = read()? {
                if let KeyCode::Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        // When a game comes to an end start a new came
        match next_game_action {
            NextGameAction::NewPiece => {
                add_shape_to_play_area(&mut play_area, &mut current_piece);
                (current_piece, next_game_action) = create_current_piece(&play_area, &pieces);
            }
            NextGameAction::Move => {
                let legal_move = move_current_piece(
                    current_piece.x,
                    current_piece.y + 1,
                    &play_area,
                    &mut current_piece,
                );
                if legal_move && can_stop_falling(&play_area, &current_piece) {
                    next_game_action = NextGameAction::NewPiece;
                }
            }
            _ => (),
        }
        // When a piece stop moving create a new piece
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
// while next_game_action != NextGameAction::GameOver {
//     let frame = create_frame(&play_area, &current_piece);
//     render_frame(&frame)?;
//     if next_game_action == NextGameAction::NewPiece {
//         if next_game_action == NextGameAction::GameOver {
//             break;
//         }
//     }
//     // thread::sleep(delay);
//     loop {
//         if poll(delay)? {
//             match read()? {
//                 Event::Key(event) => match event.code {
//                     crossterm::event::KeyCode::Char(c) => {
//                         println!("{}", c);
//                         if c == 'q' {
//                             thread
//                         }
//                     }
//                     _ => (),
//                 },
//                 _ => (),
//             }
//         } else {
//             break;
//         }
//     }
//     println!("{:?}", next_game_action);
// }

fn create_pieces() -> Vec<Piece> {
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

fn create_current_piece(
    play_area: &Vec<Vec<Bloxel>>,
    pieces: &Vec<Piece>,
) -> (Piece, NextGameAction) {
    // let mut rng = thread_rng();
    // let len = pieces.len() - 1;
    // let i = rng.gen_range(0..len);
    let piece = pieces[0].clone();
    let shape = piece.shapes[piece.orientation];

    // if new shape overlaps occupied Bloxel in play area then game over
    for x in 0..shape.len() {
        for y in 0..shape[0].len() {
            if shape[x][y] > 0 && play_area[piece.x + x][piece.y + y].occupied == true {
                return (piece, NextGameAction::GameOver);
            }
        }
    }
    (piece, NextGameAction::Move)
}

fn create_play_area(x: u16, y: u16, bg_color: Color) -> Vec<Vec<Bloxel>> {
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

fn move_current_piece(
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

fn can_stop_falling(play_area: &Vec<Vec<Bloxel>>, current_piece: &Piece) -> bool {
    let mut can_stop_falling = false;

    let shape = current_piece.shapes[current_piece.orientation];
    for x in 0..shape.len() {
        for y in (0..shape[0].len()).rev() {
            // println!("{} {}", x, y);
            if shape[x][y] > 0
                && (
                    // at bottom of play area
                    current_piece.y + y == play_area[0].len() - 1
                    // bloxel below share is occupied
                    || play_area[current_piece.x + x][current_piece.y + y + 1].occupied
                )
            {
                can_stop_falling = true;
                break;
            }
        }
    }
    can_stop_falling
}

fn add_shape_to_play_area(play_area: &mut Vec<Vec<Bloxel>>, current_piece: &mut Piece) {
    let shape = current_piece.shapes[current_piece.orientation];
    for x in 0..shape.len() {
        for y in 0..shape[0].len() {
            if shape[x][y] > 0 {
                play_area[current_piece.x + x][current_piece.y + y].occupied = true;
                play_area[current_piece.x + x][current_piece.y + y].color = current_piece.color;
            }
        }
    }
}

fn create_frame(play_area: &Vec<Vec<Bloxel>>, current_piece: &Piece) -> Vec<Vec<Color>> {
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

fn render_frame(frame: &Vec<Vec<Color>>) -> io::Result<()> {
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
