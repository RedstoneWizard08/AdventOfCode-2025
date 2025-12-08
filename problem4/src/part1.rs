use macros::embed_input;

embed_input!("../input.txt");

fn get_cell(cells: &[[bool; WIDTH]; HEIGHT], y: isize, x: isize) -> bool {
    if x < 0 || x > WIDTH as isize - 1 || y < 0 || y > WIDTH as isize - 1 {
        false
    } else {
        cells[y as usize][x as usize]
    }
}

pub fn main() {
    // bool => Is there paper in the cell?
    let mut cells = [[false; WIDTH]; HEIGHT];

    for (y, line) in INPUT.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            cells[y][x] = *ch == '@';
        }
    }

    let mut count = 0;

    for y in 0..HEIGHT as isize {
        for x in 0..WIDTH as isize {
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

    std::hint::black_box(count);

    #[cfg(feature = "cli")]
    println!("Answer: {count}");
}
