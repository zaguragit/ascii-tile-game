use crate::{AsciiSprite, RGB, Scene, assets::texture::TextureBuffer};

pub(crate) struct CharBuffer {
    char_buffer: TextureBuffer,
    bg_buffer: TextureBuffer,
    fg_buffer: TextureBuffer,
}

impl CharBuffer {
    pub(crate) fn new(
        text_width: usize,
        text_height: usize,
        scene: &dyn Scene,
    ) -> Self {
        let mut char_buffer = vec![0u8; text_width * text_height];
        let mut bg_buffer = vec![0u32; text_width * text_height];
        let mut fg_buffer = vec![0u32; text_width * text_height];
        for y in 0..text_height {
            for x in 0..text_width {
                let i = y * text_width + x;
                let AsciiSprite { fg, bg, index } = scene.get_char_at(x, y);
                char_buffer[i] = index;
                bg_buffer[i] = bg.to_abgr();
                fg_buffer[i] = fg.to_abgr();
            }
        }
        unsafe {
            Self {
                char_buffer: TextureBuffer::new_r(&char_buffer.as_slice(), text_width, text_height),
                bg_buffer: TextureBuffer::new_rgba(bg_buffer.as_slice(), text_width, text_height),
                fg_buffer: TextureBuffer::new_rgba(fg_buffer.as_slice(), text_width, text_height),
            }
        }
    }

    pub(crate) fn bind(&mut self) {
        unsafe {
            self.char_buffer.bind(1);
            self.bg_buffer.bind(2);
            self.fg_buffer.bind(3);
        }
    }
}