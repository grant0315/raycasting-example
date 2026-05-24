use raylib::prelude::*;

pub type Map = Vec<Vec<i8>>;

// Generates sample map with just boundary walls
// 0 = empty, 1 = wall
pub fn generate_map(m_width: i32, m_height: i32) -> Map {
    let mut map: Map = vec![];
    for i in 0..m_width {
        let mut row = Vec::new();
        for j in 0..m_height {
            if i == 0 || i == m_height - 1 || j == 0 || j == m_width - 1 {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        map.push(row);
    }
    map
}

pub fn print_map(map: &Map) {
    for (_, row) in map.iter().enumerate() {
        for (_, value) in row.iter().enumerate() {
            print!("{} ", value);
        }
        print!("\n");
    }
}

pub fn draw_map(d: &mut RaylibDrawHandle, map: &Map, cell_size: i32) {
    for (row_index, row) in map.iter().enumerate() {
        for (col, value) in row.iter().enumerate() {
            // If wall, draw square (i.e. 1)
            if *value == 1 {
                d.draw_rectangle(
                    row_index as i32 * cell_size,
                    col as i32 * cell_size,
                    cell_size,
                    cell_size,
                    Color::GREEN,
                );
            }

            // Draw blank square if empty (i.e. 0)
            d.draw_rectangle_lines(
                row_index as i32 * cell_size,
                col as i32 * cell_size,
                cell_size,
                cell_size,
                Color::GRAY,
            );
        }
    }
}
