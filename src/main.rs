use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::{self, Rng};
use std::{error::Error, f64, io::{stdout, Write}, sync::{atomic::AtomicBool, Arc}};
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
fn initialize_grid(initial_grid_probability: f64) -> Result<(Vec<Vec<bool>>, ConsoleSize), Box<dyn Error>> {
    // Get the current terminal size.
    let size = termsize::get().ok_or("Failed to get terminal size")?;

    // Create a random number generator.
    let mut rng = rand::thread_rng();

    // Create a 2D vector with the correct dimensions
    // and initialize all cells to `false`.
    let mut grid = vec![vec![false; size.cols as usize]; size.rows as usize];

    // Set randomly generated live cells in the grid.
    for i in 0..size.cols as usize {
        for j in 0..size.rows as usize {
            grid[j][i] = rng.gen_bool(initial_grid_probability); // Reduced the probability to make the grid less crowded.
        }
    }

    Ok((
        grid,
        ConsoleSize {
            rows: size.rows as usize,
            cols: size.cols as usize,
        },
    ))
}

/// Prints the grid to the console.
///
/// # Arguments
///
/// * `grid` - The grid to be printed.
/// * `prev_grid` - The previous grid state.
fn display_grid(grid: &[Vec<bool>], prev_grid: &[Vec<bool>]) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != prev_grid[y][x] {
                stdout.execute(cursor::MoveTo(x as u16, y as u16))?;
                if cell {
                    stdout
                        .execute(Print("#"))?;
                } else {
                    stdout
                        .execute(Print(" "))?;
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
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
fn main() -> Result<(), Box<dyn Error>> {
    let mut initial_grid_probability = 0.2;

    // The first argument to the program is a float value that controls the randomness of the initial grid.
    // let initial_grid_probability = std::env::args().nth(1).unwrap_or("0.2".to_string()).parse::<f64>().unwrap_or(0.2);
    if let Some(arg) = std::env::args().nth(1) {
        if let Some(val) = arg.parse::<f64>().ok() {
            initial_grid_probability = val;
            println!("Initial grid probability: {}", initial_grid_probability);
        }
    } else {
        println!("Default initial grid probability: {}", initial_grid_probability);
        println!("To change the initial grid probability, pass it as an argument to the program.");
        println!("Example: <program_name> 0.5");
    }
    thread::sleep(Duration::from_millis(2000));

    // Create an atomic flag to track if the user has requested to exit the program.
    let should_exit = Arc::new(AtomicBool::new(false));
    let should_exit_clone = Arc::clone(&should_exit);

    // Initialize the ctrlc handler.
    ctrlc::set_handler(move || {
        // Clear the screen before exiting.
        execute!(stdout(), Clear(ClearType::All)).expect("Error clearing screen");
        println!();
        println!("Exiting...");

        // Set the atomic flag to true to indicate that the user has requested to exit the program.
        should_exit_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        
    }).expect("Error setting Ctrl-C handler");

    // Initialize the grid with a random pattern of live and dead cells and get the size of the console.
    let (mut grid, console_size) = initialize_grid(initial_grid_probability)?;
    let mut prev_grid = grid.clone();

    // Clear the screen before starting the loop.
    execute!(stdout(), Clear(ClearType::All))?;

    // Enter an infinite loop to continuously update and display the grid.
    while !should_exit.load(std::sync::atomic::Ordering::Relaxed) {
        // Display the current state of the grid to the console.
        display_grid(&grid, &prev_grid)?;

        // Update the grid by applying the Game of Life rules.
        prev_grid = grid.clone();
        grid = update_grid(&mut grid, &console_size);

        // Sleep for a short duration to control the speed of the simulation.
        thread::sleep(Duration::from_millis(100));

        // If the user has requested to exit the program, break out of the loop.
        if should_exit.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}
