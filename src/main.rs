use macroquad::prelude::*;
use std::io;
use std::time::Duration;

const CELL_SIZE: f32 = 10.0;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Sand,
}

#[macroquad::main("sand_dropper")]
async fn main() -> io::Result<()> {
    println!("testing token");
    let mut grid_width = (screen_width() / CELL_SIZE) as usize;
    let mut grid_height = (screen_height() / CELL_SIZE) as usize;
    let mut grid = vec![vec![Cell::Empty; grid_width]; grid_height];
    loop {
        clear_background(BLACK);
        let new_grid_width = (screen_width() / CELL_SIZE) as usize;
        let new_grid_height = (screen_height() / CELL_SIZE) as usize;
        if new_grid_height != grid_height || new_grid_width != grid_width {
            grid_height = new_grid_height;
            grid_width = new_grid_width;

            grid = vec![vec![Cell::Empty; grid_width]; grid_height];
        }
        mouse_drop(&mut grid, &grid_width, &grid_height);
        physics(&mut grid, &grid_width, &grid_height);
        render(&grid, &grid_width, &grid_height);
        next_frame().await;
    }
}

fn mouse_drop(grid: &mut Vec<Vec<Cell>>, grid_width: &usize, grid_height: &usize) {
    if is_mouse_button_down(MouseButton::Left) {
        let (mx, my) = mouse_position();

        let gx = (mx / CELL_SIZE) as usize;
        let gy = (my / CELL_SIZE) as usize;

        if gx < *grid_width && gy < *grid_height {
            grid[gy][gx] = Cell::Sand;
        }
    }
}
fn physics(grid: &mut Vec<Vec<Cell>>, grid_width: &usize, grid_height: &usize) {
    for y in (0..*grid_height - 1).rev() {
        for x in 0..*grid_width {
            if grid[y][x] == Cell::Sand {
                if grid[y + 1][x] == Cell::Empty {
                    grid[y + 1][x] = Cell::Sand;
                    grid[y][x] = Cell::Empty;
                } else if x > 0 && grid[y + 1][x - 1] == Cell::Empty {
                    grid[y + 1][x - 1] = Cell::Sand;
                    grid[y][x] = Cell::Empty;
                } else if x < *grid_width - 1 && grid[y + 1][x + 1] == Cell::Empty {
                    grid[y + 1][x + 1] = Cell::Sand;
                    grid[y][x] = Cell::Empty;
                }
            }
        }
    }
}
fn render(grid: &Vec<Vec<Cell>>, grid_width: &usize, grid_height: &usize) {
    let mut colour: Color = YELLOW;
    for y in 0..*grid_height {
        for x in 0..*grid_width {
            if grid[y][x] == Cell::Sand {
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    colour,
                );
            }
        }
    }
}
