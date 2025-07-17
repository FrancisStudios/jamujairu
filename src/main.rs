/**
 *  ------ [ Property of Francis Studios ] ------
 * ==============================================
 * .. github:https://github.com/francisstudios ..
 * ____ ©2025 Francis Studios Softwares by L. ___
 */

/**
 * This is my first project in rust, just started
 * learning and getting a feel for this language.
 * The code might look rough, but this is the first
 * public rust app I ever developed, so that's my
 * excuse!
 */
use colored::Colorize;
use terminal_size::{terminal_size, Height, Width};

struct Terminal {
    width: usize,
    height: usize,
    is_size: bool,
}

fn main() {
    let termos: Terminal = get_terminal_size();

    jamujairu_standard_header(termos);
}

fn jamujairu_standard_header(term: Terminal) {
    let name_signature: &str = " [JAMUJAIRU] ";
    let spaceholder_char: &str = "=";
    let spaceholder_limit: usize = term.width - name_signature.chars().count();

    let signage_limit: usize = if spaceholder_limit % 2 == 0 {
        spaceholder_limit / 2
    } else {
        spaceholder_limit - 1 / 2
    };

    cls();

    let mut counter: usize = 0;

    loop {
        counter += 1;

        if counter > signage_limit / 2 {
            break;
        }

        print!("{}", spaceholder_char.white().bold().on_blue());
    }
    //let space_holder: u16 = term.width - name_signature.len();
}

fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/**
 * Getting terminal W & H for proper spacing
 * if !is_size -> default values please!!! :)
*/
fn get_terminal_size() -> Terminal {
    let mut terminal_width: u16 = 0;
    let mut terminal_height: u16 = 0;

    if let Some(wdth) = get_terminal_width() {
        terminal_width = wdth;
    }

    if let Some(hght) = get_terminal_height() {
        terminal_height = hght;
    }

    return Terminal {
        width: terminal_width as usize,
        height: terminal_height as usize,
        is_size: (terminal_height != 0 && terminal_width != 0),
    };
}

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
