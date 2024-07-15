// handles terminal input and output

use std::io;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

pub fn print_grid(grid_values: &Vec<String>) {
    if grid_values.len() != 9 {
        println!("Not enough values to make the grid, {}", grid_values.len());
        return;
    }
    println!("Raw value of grid is {:?}", grid_values);
}