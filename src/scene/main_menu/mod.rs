use std::fmt::Display;

use engine::{AsciiSprite, Context, Key, RGB, rgb, UpdateResult, UI, util::draw_selection_list};

use super::{world_loading::create_world_loading_scene};

#[repr(u8)]
#[derive(Debug)]
pub enum MenuOption {
    Play,
    Quit,
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MenuOption::Play => "Play",
            MenuOption::Quit => "Quit",
        })
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

    let gui_char = draw_selection_list(&state.options, state.selection, x, y, xoff, yoff, spacing, bg, color, selected_color);
    match gui_char {
        Some(c) => c,
        None => AsciiSprite { bg, fg: bg, index: 0 },
    }
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