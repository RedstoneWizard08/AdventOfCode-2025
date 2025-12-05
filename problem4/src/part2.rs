use macros::embed_input;

embed_input!("../input.txt");

const WIDTH_I: isize = WIDTH as isize;
const HEIGHT_I: isize = HEIGHT as isize;
const SIZE: usize = WIDTH * HEIGHT;

#[inline(always)]
fn rp(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn get_cell(cells: &[bool; SIZE], y: isize, x: isize) -> bool {
    if x < 0 || x > WIDTH_I - 1 || y < 0 || y > WIDTH_I - 1 {
        false
    } else {
        cells[rp(x as usize, y as usize)]
    }
}

#[inline(always)]
fn check(cells: &[bool; SIZE]) -> usize {
    let mut count = 0;

    for y in 0..HEIGHT_I {
        for x in 0..WIDTH_I {
            if !get_cell(&cells, y, x) {
                continue;
            }

            let mut filled = 0;

            filled += get_cell(&cells, y, x - 1) as u8; // left
            filled += get_cell(&cells, y, x + 1) as u8; // right
            filled += get_cell(&cells, y - 1, x) as u8; // up
            filled += get_cell(&cells, y + 1, x) as u8; // down
            filled += get_cell(&cells, y - 1, x - 1) as u8; // top left
            filled += get_cell(&cells, y - 1, x + 1) as u8; // top right
            filled += get_cell(&cells, y + 1, x - 1) as u8; // bottom left
            filled += get_cell(&cells, y + 1, x + 1) as u8; // bottom right

            if filled < 4 {
                count += 1;
            }
        }
    }

    count
}

#[cfg_attr(not(feature = "cli"), allow(unused))]
pub fn main() {
    // bool => Is there paper in the cell?
    let mut cells = [false; SIZE];

    // I *could* do this step at compile time, but that would be cheating for the benchmarks.
    for (y, line) in INPUT.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            cells[rp(x, y)] = *ch == '@';
        }
    }

    let mut count = check(&cells);
    let mut removed = 0;

    while count > 0 {
        for y in 0..HEIGHT_I {
            for x in 0..WIDTH_I {
                if !get_cell(&cells, y, x) {
                    continue;
                }

                let mut filled = 0;

                filled += get_cell(&cells, y, x - 1) as u8; // left
                filled += get_cell(&cells, y, x + 1) as u8; // right
                filled += get_cell(&cells, y - 1, x) as u8; // up
                filled += get_cell(&cells, y + 1, x) as u8; // down
                filled += get_cell(&cells, y - 1, x - 1) as u8; // top left
                filled += get_cell(&cells, y - 1, x + 1) as u8; // top right
                filled += get_cell(&cells, y + 1, x - 1) as u8; // bottom left
                filled += get_cell(&cells, y + 1, x + 1) as u8; // bottom right

                if filled < 4 {
                    cells[rp(x as usize, y as usize)] = false;
                    removed += 1;
                }
            }
        }

        count = check(&cells);
    }

    #[cfg(feature = "cli")]
    println!("Removed: {removed}");
}
