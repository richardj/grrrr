use colored::*;

pub fn vertical() -> String {
    let vertical = "┃  ".red();
    vertical.to_string()
}

pub fn top_bar(width:usize) {
    let bar = "━";
    println!("{} {} {}", top_left_corner(), bar.repeat(width + 2).red(), top_right_corner());
}

pub fn bottom_bar(width:usize) {
    let bar = "━";
    println!("{} {} {}", bottom_left_corner(), bar.repeat(width + 2).red(), bottom_right_corner());
}

/* corners */

pub fn top_left_corner() -> String {
    let corner = "┏".red();
    corner.to_string()
}

pub fn top_right_corner() -> String {
    let corner = "┓".red();
    corner.to_string()
}

pub fn bottom_left_corner() -> String {
    let corner = "┗".red();
    corner.to_string()
}

pub fn bottom_right_corner() -> String {
    let corner = "┛".red();
    corner.to_string()
}
