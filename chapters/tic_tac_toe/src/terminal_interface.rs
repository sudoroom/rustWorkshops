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
    let mut grid_output_string = String::new();
    let line = "\n-------------\n";

    grid_output_string.push_str(&format!("{line}"));

    for (i, value) in grid_values.iter().enumerate() {
        grid_output_string.push_str(&format!("|{:^3}", value));
        
        if i%3 == 2 {
            grid_output_string.push_str(&format!("|\n-------------\n"));
        }
    }

    println!("{grid_output_string}");
}