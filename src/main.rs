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
    width: u16,
    height: u16,
    is_size: bool,
}

fn main() {
    let termos: Terminal = get_terminal_size();

    jamujairu_standard_header(termos);
}

fn jamujairu_standard_header(term: Terminal) {
    let name_signature: &str = " [JAMUJAIRU] ";
    let mut leading_eqs: &str = "=";
    //let space_holder: u16 = term.width - name_signature.len();
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
        width: terminal_width,
        height: terminal_height,
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
