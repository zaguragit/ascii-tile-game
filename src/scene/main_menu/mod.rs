use estrogen::{AsciiSprite, Context, Key, RGB, rgb, UpdateResult, UI};

use super::{world_loading::create_world_loading_scene};

#[repr(u8)]
#[derive(Debug)]
pub enum MenuOption {
    Play,
    Quit,
}

impl MenuOption {
    fn get_label(&self) -> &str {
        match self {
            MenuOption::Play => "Play",
            MenuOption::Quit => "Quit",
        }
    }
}

pub struct MenuState<const OPTION_COUNT: usize> {
    options: [MenuOption; OPTION_COUNT],
    selection: usize,
}

pub fn create_main_menu_scene() -> UI<MenuState<2>> {
    let state = MenuState {
        options: [MenuOption::Play, MenuOption::Quit],
        selection: 0,
    };
    UI::new(state, get_char_at, on_input)
}

fn get_char_at<const OPTION_COUNT: usize>(state: &MenuState<OPTION_COUNT>, x: usize, y: usize) -> AsciiSprite {
    const xoff: usize = 3;
    const yoff: usize = 3;
    const spacing: usize = 3;
    const bg: RGB = rgb(0.1, 0.1, 0.1);
    const color: RGB = rgb(0.8, 0.5, 0.5);
    const selected_color: RGB = rgb(1.0, 1.0, 0.5);

    let o = y as i32 - yoff as i32;
    if o < 0 || o as usize % (spacing + 1) != 0 {
        return AsciiSprite { bg, fg: bg, index: 0 }
    }
    let o = o as usize / (spacing + 1);
    if o >= OPTION_COUNT {
        return AsciiSprite { bg, fg: bg, index: 0 }
    }
    let option = state.options[o].get_label();
    let fg = if state.selection == o { selected_color } else { color };

    let o = x as i32 - xoff as i32;
    if o < 0 || o as usize >= option.len() {
        return AsciiSprite { bg, fg: bg, index: 0 }
    }
    AsciiSprite { bg, fg, index: option.as_bytes()[o as usize] }
}

fn on_input<const OPTION_COUNT: usize>(state: &mut MenuState<OPTION_COUNT>, context: &mut Context, key: Key) -> UpdateResult {
    match key {
        Key::Up => {
            state.selection = (state.selection as i32 - 1) as usize % OPTION_COUNT;
            UpdateResult::Update
        },
        Key::Down => {
            state.selection = (state.selection as i32 + 1) as usize % OPTION_COUNT;
            UpdateResult::Update
        },
        Key::Space | Key::Enter => on_selected(&state.options[state.selection]),
        _ => UpdateResult::NoChange
    }
}

fn on_selected(option: &MenuOption) -> UpdateResult {
    match option {
        MenuOption::Play => UpdateResult::SwitchScene(Box::new(create_world_loading_scene())),
        MenuOption::Quit => UpdateResult::Quit,
    }
}