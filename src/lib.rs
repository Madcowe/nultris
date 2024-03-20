use crate::sprites::create_pieces;
use crossterm::{
    cursor::{self},
    event::{poll, read, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use gilrs::{Axis, EventType, Gilrs};
use rand::prelude::*;
use std::{
    io::{self, Write},
    isize,
    ops::AddAssign,
    time::Duration,
};
use std::{thread, time};

mod sprites;

#[derive(Debug, Clone, Copy)]
struct Bloxel {
    occupied: bool,
    color: Color,
}

#[derive(Debug, Clone)]
pub struct Piece {
    x: isize,
    y: isize,
    color: Color,
    shapes: Vec<[[u8; 4]; 4]>,
    orientation: usize,
}

pub fn main_loop() -> io::Result<()> {
    let bg_color = Color::Rgb { r: 0, g: 0, b: 0 };
    let mut play_area = create_play_area(10, 20, bg_color);
    terminal::enable_raw_mode()?;
    let pieces = create_pieces();
    let (mut current_piece, mut game_over) = create_current_piece(&play_area, &pieces);
    let mut delay = time::Duration::from_millis(250);
    // clear what is currently showing in terminal as render_frame doesn't do this
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    let mut gilrs = Gilrs::new().unwrap();
    let (mut joy_x, mut joy_y); // = (0f32, 0f32);
    let (mut up_pressed, mut down_pressed, mut left_pressed, mut right_pressed) =
        (false, false, false, false);

    // When quit button is pressed quit the game
    loop {
        let frame = create_frame(&play_area, &current_piece);
        render_frame(&frame)?;
        let (mut x, mut y, mut orientation) =
            (current_piece.x, current_piece.y, current_piece.orientation);
        // Keyboard controls
        // get rid of all previous events so only keys pressed in active play get applied
        while poll(time::Duration::from_secs(0))? {
            read()?;
        }
        if poll(delay)? {
            if let Event::Key(event) = read()? {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        KeyCode::Left => {
                            x = current_piece.x - 1;
                        }
                        KeyCode::Right => {
                            x = current_piece.x + 1;
                        }
                        KeyCode::Up => {
                            // if not at end of shapes then orientation + 1 other wise set to 0
                            orientation = 0;
                            if current_piece.orientation < current_piece.shapes.len() - 1 {
                                orientation = current_piece.orientation + 1;
                            }
                        }
                        KeyCode::Down => {
                            y = current_piece.y + 1;
                        }
                        _ => (),
                    }
                }
            }
        }
        // joystick controls
        while let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
            if let EventType::AxisChanged(axis, position, _) = event {
                // eprintln!("{:?} {} joy_x: {} joy_y: {}", axis, position, joy_x, joy_y);
                match axis {
                    Axis::LeftStickX => {
                        joy_x = position;
                        if joy_x > 0.5 && !right_pressed {
                            x = current_piece.x + 1;
                            right_pressed = true
                        } else if joy_x <= 0.5 && right_pressed {
                            right_pressed = false;
                        } else if joy_x < -0.5 && !left_pressed {
                            x = current_piece.x - 1;
                            left_pressed = true;
                        } else if joy_x >= -0.5 && left_pressed {
                            left_pressed = false;
                        }
                    }
                    Axis::LeftStickY => {
                        joy_y = position;
                        if joy_y > 0.5 && !up_pressed {
                            orientation = 0;
                            if current_piece.orientation < current_piece.shapes.len() - 1 {
                                orientation = current_piece.orientation + 1;
                            }
                            up_pressed = true;
                        } else if joy_y <= 0.5 && up_pressed {
                            up_pressed = false;
                        } else if joy_y < -0.5 && !down_pressed {
                            y = current_piece.y + 1;
                            down_pressed = true;
                        } else if joy_y >= -0.5 && down_pressed {
                            down_pressed = false;
                        }
                    }
                    _ => (),
                }
            }
        }
        let can_stop = move_current_piece(x, y, orientation, &play_area, &mut current_piece);
        // if a piece stops moving create a new piece else move down
        if can_stop {
            add_shape_to_play_area(&mut play_area, &mut current_piece);
            let speed_up = remove_complete_rows(&mut play_area, bg_color);
            if speed_up && delay.as_millis() > 20 {
                delay = time::Duration::from_millis(delay.as_millis() as u64 - 5);
            }
            (current_piece, game_over) = create_current_piece(&play_area, &pieces);
        } else {
            _ = move_current_piece(
                current_piece.x,
                current_piece.y + 1,
                current_piece.orientation,
                &play_area,
                &mut current_piece,
            );
        }
        // When a game comes to an end start a new game
        if game_over {
            // game over animation
            play_area = create_play_area(10, 20, bg_color);
            let frame = create_blank_frame(bg_color);
            render_frame(&frame)?;
            down_pressed = false;
            (current_piece, game_over) = create_current_piece(&play_area, &pieces);
            // let restart_delay = time::Duration::from_millis(1000);
            // thread::sleep(restart_delay);
            while poll(time::Duration::from_secs(0))? {
                read()?;
            }
            loop {
                if poll(delay)? {
                    if let Event::Key(event) = read()? {
                        match event.code {
                            KeyCode::Down => {
                                break ();
                            }
                            _ => (),
                        }
                    }
                }
                // joystick controls
                while let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
                    if let EventType::AxisChanged(axis, position, _) = event {
                        // eprintln!("{:?} {} joy_x: {} joy_y: {}", axis, position, joy_x, joy_y);
                        match axis {
                            Axis::LeftStickY => {
                                joy_y = position;
                                if joy_y < -0.5 && !down_pressed {
                                    down_pressed = true;
                                } else if joy_y >= -0.5 && down_pressed {
                                    down_pressed = false;
                                }
                            }
                            _ => (),
                        }
                    }
                }
                if down_pressed {
                    break;
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

fn create_current_piece(play_area: &Vec<Vec<Bloxel>>, pieces: &Vec<Piece>) -> (Piece, bool) {
    let mut rng = thread_rng();
    let len = pieces.len();
    let i = rng.gen_range(0..len);
    let piece = pieces[i].clone();
    let shape = piece.shapes[piece.orientation];

    // if new shape overlaps occupied Bloxel in play area then game over
    for x in 0..shape.len() {
        for y in 0..shape[0].len() {
            // if shape is occpied and within play area then test against play area
            if shape[x][y] > 0 && piece.x + x as isize >= 0 && piece.y + y as isize >= 0 {
                let (x, y) = (piece.x as usize + x as usize, piece.y as usize + y as usize);
                if play_area[x][y].occupied == true {
                    return (piece, true);
                }
            }
        }
    }
    (piece, false)
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
    x: isize,
    y: isize,
    orientation: usize,
    play_area: &Vec<Vec<Bloxel>>,
    current_piece: &mut Piece,
) -> bool {
    let mut can_move = true;
    let max_x = play_area.len();
    let max_y = play_area[0].len();
    let (origin_x, origin_y) = (x, y);
    let shape = current_piece.shapes[orientation];
    for x in 0..shape.len() {
        for y in 0..shape[0].len() {
            let occupied = shape[x][y];
            let (x, y) = (origin_x + x as isize, origin_y + y as isize);
            // if occupied bit of shape outside of play area can_move is false
            if occupied > 0 && (x < 0 || y < 0 || x >= max_x as isize || y >= max_y as isize) {
                can_move = false;
                break;
            // if occupied bit of shape within play area
            } else if occupied > 0 && x >= 0 && y >= 0 && x < max_x as isize && y < max_y as isize {
                let (x, y) = (x as usize, y as usize);
                // and that bit of play area is already occupied then can_move is false
                if play_area[x][y].occupied {
                    can_move = false;
                    break;
                }
            }
        }
        if can_move == false {
            break;
        }
    }
    if can_move {
        current_piece.x = x;
        current_piece.y = y;
        current_piece.orientation = orientation;
    }
    can_stop_falling(&play_area, &current_piece)
}

fn can_stop_falling(play_area: &Vec<Vec<Bloxel>>, current_piece: &Piece) -> bool {
    let mut can_stop_falling = false;
    let max_x = play_area.len();
    let max_y = play_area[0].len();

    let shape = current_piece.shapes[current_piece.orientation];
    for x in 0..shape.len() {
        for y in (0..shape[0].len()).rev() {
            // if shape bit occupied and bit within play_area
            if shape[x][y] > 0
                && current_piece.x + x as isize >= 0
                && current_piece.y + y as isize >= 0
                && (current_piece.x + x as isize) < max_x as isize
                && (current_piece.y + y as isize) < max_y as isize
            {
                let (x, y) = (
                    (current_piece.x + x as isize) as usize,
                    (current_piece.y + y as isize) as usize,
                );
                if y == max_y - 1 || play_area[x][y + 1].occupied {
                    // at bottom of play area or bloxel below shape is occupied
                    can_stop_falling = true;
                    break;
                }
            }
        }
    }
    can_stop_falling
}

fn add_shape_to_play_area(play_area: &mut Vec<Vec<Bloxel>>, current_piece: &mut Piece) {
    let shape = current_piece.shapes[current_piece.orientation];
    for x in 0..shape.len() {
        for y in 0..shape[0].len() {
            if shape[x][y] > 0
                && current_piece.x + x as isize >= 0
                && current_piece.y + y as isize >= 0
            {
                // occupied piece is not negatively ouside playing area
                let (x, y) = (
                    (current_piece.x + x as isize) as usize,
                    (current_piece.y + y as isize) as usize,
                );
                play_area[x][y].occupied = true;
                play_area[x][y].color = current_piece.color;
            }
        }
    }
}

fn remove_complete_rows(play_area: &mut Vec<Vec<Bloxel>>, bg_color: Color) -> bool {
    let mut complete_rows = Vec::new();
    for y in 0..play_area[0].len() {
        let mut total_occupied = 0;
        for x in 0..play_area.len() {
            if play_area[x][y].occupied {
                total_occupied += 1;
            }
            if total_occupied == play_area.len() {
                complete_rows.push(y);
            }
        }
    }
    let rows_removed = !complete_rows.is_empty();
    for row in complete_rows.into_iter() {
        for y in (0..row + 1).rev() {
            for x in 0..play_area.len() {
                // if not top most row copy row from above
                if y > 0 {
                    play_area[x][y] = play_area[x][y - 1];
                // if top row just blank out as none to copy from above
                } else {
                    play_area[x][y].occupied = false;
                    play_area[x][y].color = bg_color;
                }
            }
        }
    }
    rows_removed
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
        for occupied in column {
            if occupied > 0 && x >= 0 && y >= 0 && x < max_x as isize && y < max_y as isize {
                // If x, y co-odrinate  occupied and within play area
                let (x, y) = (x as usize, y as usize);
                // if y co-ordinate wihtin frame and occupied
                frame[x][y] = color;
            }
            y += 1;
        }
        y = current_piece.y;
        x += 1;
    }
    frame
}

fn create_blank_frame(bg_color: Color) -> Vec<Vec<Color>> {
    let column: Vec<Color> = vec![bg_color; 20];
    let frame: Vec<Vec<Color>> = vec![column; 10];
    frame
}

fn render_frame(frame: &Vec<Vec<Color>>) -> io::Result<()> {
    // should check and throw error if terminal is smaller than frame
    let mut stdout = io::stdout();

    // purge all the history so user can't scroll back this doesn't get rid of stuff initally
    execute!(stdout, terminal::Clear(terminal::ClearType::Purge))?;

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
