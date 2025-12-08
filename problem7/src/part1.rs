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

    let mut splits = 0;

    for y in 0..HEIGHT {
        let line = grid[y];

        for x in 0..WIDTH {
            let cell = line[x];

            match cell {
                Cell::Beam => {}

                Cell::Split => {
                    if y > 0 && grid[y - 1][x] == Cell::Beam {
                        splits += 1;

                        if x > 0 {
                            grid[y][x - 1] = Cell::Beam;
                        }

                        if x < WIDTH - 1 {
                            grid[y][x + 1] = Cell::Beam;
                        }
                    }
                }

                Cell::None => {
                    if y > 0 && grid[y - 1][x] == Cell::Beam {
                        grid[y][x] = Cell::Beam;
                    }
                }
            }
        }
    }

    std::hint::black_box(splits);

    #[cfg(feature = "cli")]
    println!("Splits: {splits}");
}
