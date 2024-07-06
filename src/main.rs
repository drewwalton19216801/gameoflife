use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::{self, Rng};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use termsize;

/// Represents the size of the console.
///
/// This struct contains the number of rows and columns in the console.
struct ConsoleSize {
    /// The number of rows in the console.
    rows: usize,
    /// The number of columns in the console.
    cols: usize,
}

/// Generates an initial grid for the Game of Life.
///
/// The grid is initialized with a random pattern of live and dead cells.
///
/// # Returns
///
/// * `grid` - The initial grid.
/// * `console_size` - The size of the console.
fn initialize_grid() -> (Vec<Vec<bool>>, ConsoleSize) {
    // Get the current terminal size.
    let size = termsize::get().unwrap();

    // Create a random number generator.
    let mut rng = rand::thread_rng();

    // Create a 2D vector with the correct dimensions
    // and initialize all cells to `false`.
    let mut grid = vec![vec![false; size.cols as usize]; size.rows as usize];

    // Set randomly generated live cells in the grid.
    for i in 0..size.cols as usize {
        for j in 0..size.rows as usize {
            grid[j][i] = rng.gen_bool(0.2); // Reduced the probability to make the grid less crowded.
        }
    }

    (
        grid,
        ConsoleSize {
            rows: size.rows as usize,
            cols: size.cols as usize,
        },
    )
}

/// Prints the grid to the console.
///
/// # Arguments
///
/// * `grid` - The grid to be printed.
/// * `prev_grid` - The previous grid state.
fn display_grid(grid: &[Vec<bool>], prev_grid: &[Vec<bool>]) {
    let mut stdout = stdout();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != prev_grid[y][x] {
                stdout.execute(cursor::MoveTo(x as u16, y as u16)).unwrap();
                if cell {
                    stdout
                        .execute(SetForegroundColor(Color::Green))
                        .unwrap()
                        .execute(Print("#"))
                        .unwrap();
                } else {
                    stdout
                        .execute(SetForegroundColor(Color::Black))
                        .unwrap()
                        .execute(Print(" "))
                        .unwrap();
                }
            }
        }
    }
    stdout.flush().unwrap();
}

/// Calculates the number of live neighbors of a cell in the grid.
///
/// # Arguments
///
/// * `grid` - The grid containing the cells.
/// * `x` - The x-coordinate of the cell.
/// * `y` - The y-coordinate of the cell.
///
/// # Returns
///
/// The number of live neighbors.
fn live_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
    // Initialize a count for the live neighbors.
    let mut count = 0;

    // Iterate over the neighbors of the cell.
    for i in -1..=1 {
        for j in -1..=1 {
            // Skip the cell itself.
            if i == 0 && j == 0 {
                continue;
            }

            // Check if the neighbor is within the grid bounds.
            if let Some(&cell) = grid
                .get((y as isize + i) as usize)
                .and_then(|row| row.get((x as isize + j) as usize))
            {
                // Increment the count if the neighbor is live.
                count += cell as usize;
            }
        }
    }

    // Return the count of live neighbors.
    count
}

/// Updates the grid by applying the Game of Life rules.
///
/// # Arguments
///
/// * `grid` - The grid to be updated.
/// * `size` - The size of the grid.
///
/// # Returns
///
/// The updated grid.
fn update_grid(grid: &mut [Vec<bool>], size: &ConsoleSize) -> Vec<Vec<bool>> {
    // Create a new grid with the same dimensions as the input grid.
    let mut new_grid = vec![vec![false; size.cols]; size.rows];

    // Iterate over each cell in the grid.
    for i in 0..size.rows {
        for j in 0..size.cols {
            // Calculate the number of live neighbors of the cell.
            let live_neighbors = live_neighbors(grid, j, i);

            // Apply the Game of Life rules to determine the next state of the cell.
            if grid[i][j] {
                // If the cell is alive:
                // - If it has 2 or 3 live neighbors, it remains alive.
                // - Otherwise, it dies.
                new_grid[i][j] = live_neighbors == 2 || live_neighbors == 3;
            } else {
                // If the cell is dead:
                // - If it has exactly 3 live neighbors, it becomes alive.
                // - Otherwise, it remains dead.
                new_grid[i][j] = live_neighbors == 3;
            }
        }
    }

    // Return the updated grid.
    new_grid
}

/// The main entry point of the program.
///
/// This function initializes a grid, enters an infinite loop where it updates and displays the grid,
/// and sleeps for a short duration to control the speed of the simulation.
///
/// # Returns
///
/// This function does not return anything.
fn main() {
    // Initialize the grid with a random pattern of live and dead cells and get the size of the console.
    let (mut grid, console_size) = initialize_grid();
    let mut prev_grid = grid.clone();

    // Clear the screen before starting the loop.
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    // Enter an infinite loop to continuously update and display the grid.
    loop {
        // Display the current state of the grid to the console.
        display_grid(&grid, &prev_grid);

        // Update the grid by applying the Game of Life rules.
        prev_grid = grid.clone();
        grid = update_grid(&mut grid, &console_size);

        // Sleep for a short duration to control the speed of the simulation.
        thread::sleep(Duration::from_millis(100));
    }
}
