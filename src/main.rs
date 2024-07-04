use std::thread;
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;


fn initialize_grid() -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            grid[j][i] = rand::random();
        }
    }
    for i in 0..WIDTH {
        grid[0][i] = true;
        grid[HEIGHT - 1][i] = true;
    }
    for i in 0..HEIGHT {
        grid[i][0] = true;
        grid[i][WIDTH - 1] = true;
    }
    grid
}

fn display_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell { '#' } else { ' ' });
        }
        println!();
    }
}

fn live_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if let Some(&cell) = grid.get((y as i32 + i) as usize)
                                     .and_then(|row| row.get((x as i32 + j) as usize)) {
                count += cell as usize;
            }
        }
    }
    count
}

fn update_grid(grid: &mut [Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_grid = initialize_grid();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let live_neighbors = live_neighbors(grid, j, i);
            if grid[i][j] {
                new_grid[i][j] = live_neighbors == 2 || live_neighbors == 3;
            } else {
                new_grid[i][j] = live_neighbors == 3;
            }
        }
    }
    new_grid
}

fn main() {
    let mut grid = initialize_grid();
    loop {
        display_grid(&grid);
        grid = update_grid(&mut grid);
        thread::sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
    }
}
