use macros::embed_input;

embed_input!("../input.txt");

fn get_cell(cells: &[[bool; WIDTH]; HEIGHT], y: isize, x: isize) -> bool {
    if x < 0 || x > WIDTH as isize - 1 || y < 0 || y > WIDTH as isize - 1 {
        false
    } else {
        cells[y as usize][x as usize]
    }
}

fn check(cells: &[[bool; WIDTH]; HEIGHT]) -> usize {
    let mut count = 0;

    for y in 0..HEIGHT as isize {
        for x in 0..WIDTH as isize {
            if !get_cell(&cells, y, x) {
                continue;
            }

            let values = [
                get_cell(&cells, y, x - 1),     // left
                get_cell(&cells, y, x + 1),     // right
                get_cell(&cells, y - 1, x),     // up
                get_cell(&cells, y + 1, x),     // down
                get_cell(&cells, y - 1, x - 1), // top left
                get_cell(&cells, y - 1, x + 1), // top right
                get_cell(&cells, y + 1, x - 1), // bottom left
                get_cell(&cells, y + 1, x + 1), // bottom right
            ];

            if values.into_iter().filter(|it| *it).count() < 4 {
                count += 1;
            }
        }
    }

    count
}

#[cfg_attr(not(feature = "cli"), allow(unused))]
pub fn main() {
    // bool => Is there paper in the cell?
    let mut cells = [[false; WIDTH]; HEIGHT];

    for (y, line) in INPUT.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            cells[y][x] = *ch == '@';
        }
    }

    let mut count = check(&cells);
    let mut removed = 0;

    while count > 0 {
        for y in 0..HEIGHT as isize {
            for x in 0..WIDTH as isize {
                if !get_cell(&cells, y, x) {
                    continue;
                }

                let values = [
                    get_cell(&cells, y, x - 1),     // left
                    get_cell(&cells, y, x + 1),     // right
                    get_cell(&cells, y - 1, x),     // up
                    get_cell(&cells, y + 1, x),     // down
                    get_cell(&cells, y - 1, x - 1), // top left
                    get_cell(&cells, y - 1, x + 1), // top right
                    get_cell(&cells, y + 1, x - 1), // bottom left
                    get_cell(&cells, y + 1, x + 1), // bottom right
                ];

                if values.into_iter().filter(|it| *it).count() < 4 {
                    cells[y as usize][x as usize] = false;
                    removed += 1;
                }
            }
        }

        count = check(&cells);
    }

    #[cfg(feature = "cli")]
    println!("Removed: {removed}");
}
