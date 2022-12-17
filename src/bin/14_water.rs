use ndarray::prelude::*;
use ndarray::Array2;
use std::cmp::{max, min};
use std::env;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Line(Coord, Coord);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Dims2(usize, usize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct CanvasDims {
    origin: Coord, // top left point
    size: Dims2,   // width in x,y
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let data = fs::read_to_string(file_path).unwrap();
    let lines = parse_puzzle_input(&data);

    let dims = get_canvas_dims(&lines);
    let mut canvas = make_starting_canvas(&lines);
    let count = sand_it(&mut canvas, &dims);
    dbg!(count);

    // Add that super wide line just past the end
    let mut lines = parse_puzzle_input(&data);
    lines.push(Line(
        Coord {
            x: 300,
            y: dims.size.1 as i32 + 1,
        },
        Coord {
            x: 700,
            y: dims.size.1 as i32 + 1,
        },
    ));
    let mut canvas = make_starting_canvas(&lines);
    let dims = get_canvas_dims(&lines);
    let count = sand_it(&mut canvas, &dims);
    dbg!(count);
}

fn sand_it(canvas: &mut Array2<char>, dims: &CanvasDims) -> usize {
    let sand_origin = Coord { x: 500, y: 0 };

    dbg!(dims);

    let start_y = sand_origin.y - dims.origin.y;
    let start_x = sand_origin.x - dims.origin.x;

    // slice down starting at origin
    let mut count = 0;
    loop {
        let mut path = canvas.clone();
        let mut y = start_y;
        let mut x = start_x;

        loop {
            // is below out of bounds, have we won?
            if y as usize == dims.size.1 - 1 {
                println!("bottom! ({}, {})", x, y);
                draw_canvas(&path);
                return count;
            }

            path.slice_mut(s![y, x]).fill('~');

            // down
            if *canvas.slice(s![y + 1, x]).into_scalar() == '.' {
                y += 1;
                continue;
            }

            // left goes off?
            if x <= 0 {
                println!("left! ({}, {})", x, y);
                draw_canvas(&path);
                return count;
            }

            // try leftcanvas.slice_mut(s![y, x]).fill('o');
            if *canvas.slice(s![y + 1, x - 1]).into_scalar() == '.' {
                y += 1;
                x -= 1;
                continue;
            }

            // right goes off?
            if x as usize >= dims.size.0 - 1 {
                println!("right! ({}, {})", x, y);
                draw_canvas(&path);
                return count;
            }

            // try right
            if *canvas.slice(s![y + 1, x + 1]).into_scalar() == '.' {
                y += 1;
                x += 1;
                continue;
            }

            // sand is at rest, stop
            break;
        }

        // place the sand
        canvas.slice_mut(s![y, x]).fill('o');
        count += 1;

        if x == start_x && y == start_y {
            println!("full! ({}, {})", x, y);
            draw_canvas(&path);
            return count; // game over
        }
    }
}

fn make_starting_canvas(lines: &[Line]) -> Array2<char> {
    let dims = get_canvas_dims(lines);
    let mut canvas = Array2::from_elem((dims.size.1, dims.size.0), '.');
    let sand_origin = Coord { x: 500, y: 0 };

    canvas
        .slice_mut(s![
            sand_origin.y - dims.origin.y,
            sand_origin.x - dims.origin.x,
        ])
        .fill('+');

    for line in lines {
        if line.0.y == line.1.y {
            // line in y
            let a = line.0.x - dims.origin.x;
            let b = line.1.x - dims.origin.x;

            canvas
                .slice_mut(s![line.0.y - dims.origin.y, (min(a, b))..=(max(a, b))])
                .fill('#');
        } else {
            // line in x
            let a = line.0.y - dims.origin.y;
            let b = line.1.y - dims.origin.y;

            canvas
                .slice_mut(s![(min(a, b))..=(max(a, b)), line.0.x - dims.origin.x])
                .fill('#');
        }
    }

    canvas
}

fn draw_canvas(canvas: &Array2<char>) {
    for row in canvas.rows() {
        println!("{}", row.iter().collect::<String>());
    }
}

#[test]
fn test_make_starting_canvas() {
    let lines = parse_puzzle_input("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");
    let canvas = make_starting_canvas(&lines);
    draw_canvas(&canvas);
}

fn get_canvas_dims(lines: &[Line]) -> CanvasDims {
    let points = lines
        .iter()
        .flat_map(|line| vec![line.0, line.1])
        .chain(vec![Coord { x: 500, y: 0 }])
        .collect::<Vec<Coord>>();
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    CanvasDims {
        origin: Coord { x: min_x, y: min_y },
        size: Dims2((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1),
    }
}

fn parse_puzzle_input(text: &str) -> Vec<Line> {
    text.lines().flat_map(parse_line).collect::<Vec<_>>()
}

fn parse_line(text: &str) -> Vec<Line> {
    let text = text.split(" -> ").collect::<Vec<_>>();
    text.windows(2)
        .map(|pair| Line(parse_coord(pair[0]), parse_coord(pair[1])))
        .collect::<Vec<Line>>()
}

fn parse_coord(text: &str) -> Coord {
    let text = text.split(',').collect::<Vec<_>>();
    Coord {
        x: text[0].parse().unwrap(),
        y: text[1].parse().unwrap(),
    }
}

#[test]
fn test_get_canvas_dims() {
    let lines = parse_puzzle_input("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");
    assert_eq!(
        get_canvas_dims(&lines),
        CanvasDims {
            origin: Coord { x: 494, y: 0 },
            size: Dims2(10, 10)
        }
    );
}

#[test]
fn test_parse_puzzle_input() {
    assert_eq!(
        parse_puzzle_input("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9"),
        vec![
            Line(Coord { x: 498, y: 4 }, Coord { x: 498, y: 6 }),
            Line(Coord { x: 498, y: 6 }, Coord { x: 496, y: 6 }),
            Line(Coord { x: 503, y: 4 }, Coord { x: 502, y: 4 }),
            Line(Coord { x: 502, y: 4 }, Coord { x: 502, y: 9 }),
            Line(Coord { x: 502, y: 9 }, Coord { x: 494, y: 9 }),
        ]
    );
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("498,4 -> 498,6 -> 496,6"),
        vec![
            Line(Coord { x: 498, y: 4 }, Coord { x: 498, y: 6 }),
            Line(Coord { x: 498, y: 6 }, Coord { x: 496, y: 6 })
        ]
    );

    assert_eq!(
        parse_line("503,4 -> 502,4 -> 502,9 -> 494,9"),
        vec![
            Line(Coord { x: 503, y: 4 }, Coord { x: 502, y: 4 }),
            Line(Coord { x: 502, y: 4 }, Coord { x: 502, y: 9 }),
            Line(Coord { x: 502, y: 9 }, Coord { x: 494, y: 9 }),
        ]
    );
}

#[test]
fn test_parse_coord() {
    assert_eq!(parse_coord("498,4"), Coord { x: 498, y: 4 });
    assert_eq!(parse_coord("498,6"), Coord { x: 498, y: 6 });
    assert_eq!(parse_coord("496,6"), Coord { x: 496, y: 6 });
    assert_eq!(parse_coord("503,4"), Coord { x: 503, y: 4 });
    assert_eq!(parse_coord("502,4"), Coord { x: 502, y: 4 });
    assert_eq!(parse_coord("502,9"), Coord { x: 502, y: 9 });
}
