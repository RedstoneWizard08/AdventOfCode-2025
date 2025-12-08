use macros::embed_input;

embed_input!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    None,
    Beam,
    Split,
}

pub fn main() {
    let mut grid = [[Cell::None; WIDTH]; HEIGHT];

    for (y, line) in INPUT.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            grid[y][x] = match *ch {
                'S' | '|' => Cell::Beam,
                '^' => Cell::Split,
                '.' => Cell::None,
                _ => panic!("Invalid character!"),
            };
        }
    }

    let mut hits = [0i64; WIDTH];

    hits[grid[0].iter().position(|it| *it == Cell::Beam).unwrap()] += 1;

    for y in 0..HEIGHT {
        let line = grid[y];
        let mut row = [0i64; WIDTH];

        for x in 0..WIDTH {
            let cell = line[x];

            match cell {
                Cell::Beam => {
                    row[x] = hits[x];
                }

                Cell::Split => {
                    if y > 0 && grid[y - 1][x] == Cell::Beam {
                        if x > 0 {
                            grid[y][x - 1] = Cell::Beam;
                            row[x - 1] += hits[x];
                        }

                        if x < WIDTH - 1 {
                            grid[y][x + 1] = Cell::Beam;
                            row[x + 1] += hits[x];
                        }
                    }
                }

                Cell::None => {
                    if y > 0 && grid[y - 1][x] == Cell::Beam {
                        grid[y][x] = Cell::Beam;
                    }

                    row[x] += hits[x];
                }
            }
        }

        hits = row;
    }

    let timelines = hits.into_iter().sum::<i64>();

    std::hint::black_box(timelines);

    #[cfg(feature = "cli")]
    println!("Timelines: {timelines}");
}
