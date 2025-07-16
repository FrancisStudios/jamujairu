use colored::Colorize;
use terminal_size::{terminal_size, Height, Width};

struct Terminal {
    width: u16,
    height: u16,
}

fn main() {
    println!("{} {} !", "it".green(), "works".blue().bold());
    //print!("{} {}");

    let mut terminal_width: u16 = 0;
    let mut terminal_height: u16 = 0;

    if let Some(wdth) = get_terminal_width() {
        terminal_width = wdth;
    }

    if let Some(hght) = get_terminal_height() {
        terminal_height = hght;
    }

    let term = Terminal {
        width: terminal_width,
        height: terminal_height,
    };

    println!("{} {}", term.width, term.height);
}

fn jamujairu_standard_header() {}

fn get_terminal_width() -> Option<u16> {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w)
    } else {
        None
    }
}

fn get_terminal_height() -> Option<u16> {
    if let Some((_, Height(h))) = terminal_size() {
        Some(h)
    } else {
        None
    }
}
