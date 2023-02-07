use std::collections::HashMap;

use glfw::Action;

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    A, B, C, D, E,
    F, G, H, I, J,
    K, L, M, N, O,
    P, Q, R, S, T,
    U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    Up, Down, Left, Right,
    LeftShift, RightShift,
    Space, Enter, Tab,
}

const KEY_COUNT: usize = Key::Tab as usize + 1;


pub(crate) struct KeyBuffer {
    pub(crate) data: HashMap<Key, bool>,
    pub(crate) pressed_key_counter: usize,
}

impl KeyBuffer {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new(),
            pressed_key_counter: 0,
        }
    }

    pub(crate) fn on_event(
        &mut self,
        key: Key,
        action: Action,
    ) {
        match action {
            Action::Press => {
                self.data.insert(key, true);
                self.pressed_key_counter = self.pressed_key_counter + 1;
            },
            Action::Release => {
                self.data.insert(key, false);
                self.pressed_key_counter = self.pressed_key_counter - 1;
            },
            Action::Repeat => {},
        }
    }
}