use crate::{Numeral, Piece};
use crossterm::style::Color;

pub fn create_numerals() -> Vec<Numeral> {
    let mut numerals = Vec::new();

    // 0
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 0, 0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 1
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 2
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 1, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 3
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 4
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 5
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 6
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 7
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 8
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    // 9
    let shape: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let numeral: Numeral = Numeral {
        x: 0,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shape,
    };
    numerals.push(numeral);

    numerals
}

pub fn create_pieces() -> Vec<Piece> {
    let mut pieces = Vec::new();

    // J tetromino
    let mut shapes = Vec::new();
    let mut shape: [[u8; 4]; 4] = [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    let mut piece: Piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb { r: 0, g: 0, b: 255 },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // L tetromino
    shapes = Vec::new();
    shape = [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb {
            r: 115,
            g: 196,
            b: 144,
        },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // O tetromino
    shapes = Vec::new();
    shape = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb {
            r: 255,
            g: 255,
            b: 0,
        },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // I tetromino
    shapes = Vec::new();
    shape = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb {
            r: 0,
            g: 255,
            b: 255,
        },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // S tetromino
    shapes = Vec::new();
    shape = [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb { r: 0, g: 255, b: 0 },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // Z tetromino
    shapes = Vec::new();
    shape = [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb { r: 255, g: 0, b: 0 },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);

    // T tetromino
    shapes = Vec::new();
    shape = [[1, 0, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    shape = [[1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    shapes.push(shape);
    piece = Piece {
        x: 4,
        y: 0,
        color: Color::Rgb {
            r: 147,
            g: 112,
            b: 219,
        },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);
    pieces
}
