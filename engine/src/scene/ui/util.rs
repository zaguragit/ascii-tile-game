use std::fmt::Display;

use crate::{RGB, AsciiSprite};

pub fn draw_text(
    text: &str,
    x: usize,
    y: usize,
    xoff: usize,
    yoff: usize,
    bg: RGB,
    fg: RGB,
) -> Option<AsciiSprite> {
    if y != yoff {
        return None
    }
    let o = x as i32 - xoff as i32;
    if o < 0 || o as usize >= text.len() {
        return None
    }
    Some(AsciiSprite { bg, fg, index: text.as_bytes()[o as usize] })
}

pub fn draw_selection_list<const OPTION_COUNT: usize, T : Display>(
    items: &[T; OPTION_COUNT],
    selection: usize,
    x: usize,
    y: usize,
    xoff: usize,
    yoff: usize,
    spacing: usize,
    bg: RGB,
    color: RGB,
    selected_color: RGB,
) -> Option<AsciiSprite> {
    let o = y as i32 - yoff as i32;
    if o < 0 || o as usize % (spacing + 1) != 0 {
        return None
    }
    let o = o as usize / (spacing + 1);
    if o >= OPTION_COUNT {
        return None
    }
    let option = items[o].to_string();
    let fg = if selection == o { selected_color } else { color };

    draw_text(option.as_str(), x, 0, xoff, 0, bg, fg)
}