use crate::Piece;
use crossterm::style::Color;

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
        color: Color::Rgb {
            r: 205,
            g: 152,
            b: 211,
        },
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
            r: 221,
            g: 77,
            b: 182,
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
            r: 23,
            g: 21,
            b: 41,
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
        color: Color::Rgb {
            r: 82,
            g: 155,
            b: 129,
        },
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
        color: Color::Rgb {
            r: 136,
            g: 73,
            b: 128,
        },
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
            r: 255,
            g: 167,
            b: 235,
        },
        shapes,
        orientation: 0,
    };
    pieces.push(piece);
    pieces
}
