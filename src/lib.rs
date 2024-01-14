// An attribute to hide warnings for unused code.
#![allow(dead_code)]

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
