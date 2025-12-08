use macros::embed_input;

embed_input!("../input.txt");

const WIDTH_B: usize = WIDTH - 1;
const HEIGHT_B: usize = HEIGHT - 1;

const SIZE: usize = WIDTH * HEIGHT;

fn rp(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

pub fn main() {
    // bool => Is there paper in the cell?
    let mut cells = [false; SIZE];

    // I *could* do this step at compile time, but that would be cheating for the benchmarks.
    for (y, line) in INPUT.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            cells[rp(x, y)] = *ch == '@';
        }
    }

    let mut removed = 0;

    loop {
        let mut removed_local = 0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if !cells[rp(x, y)] {
                    continue;
                }

                let mut filled = 0;

                filled += (x > 0 && cells[rp(x - 1, y)]) as u8; // left
                filled += (x < WIDTH_B && cells[rp(x + 1, y)]) as u8; // right
                filled += (y > 0 && cells[rp(x, y - 1)]) as u8; // up
                filled += (y < HEIGHT_B && cells[rp(x, y + 1)]) as u8; // down
                filled += (x > 0 && y > 0 && cells[rp(x - 1, y - 1)]) as u8; // top left
                filled += (x > 0 && y < HEIGHT_B && cells[rp(x - 1, y + 1)]) as u8; // top right
                filled += (x < WIDTH_B && y > 0 && cells[rp(x + 1, y - 1)]) as u8; // bottom left
                filled += (x < WIDTH_B && y < HEIGHT_B && cells[rp(x + 1, y + 1)]) as u8; // bottom right

                if filled < 4 {
                    cells[rp(x, y)] = false;
                    removed_local += 1;
                }
            }
        }

        if removed_local <= 0 {
            break;
        }

        removed += removed_local;
    }

    std::hint::black_box(removed);

    #[cfg(feature = "cli")]
    println!("Removed: {removed}");
}
