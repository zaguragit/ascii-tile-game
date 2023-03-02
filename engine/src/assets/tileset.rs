use std::{ptr::null};

use stb::image::{Channels, stbi_load_from_memory};

pub(crate) struct Tileset {
    id: u32,
}

impl Drop for Tileset {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

impl Tileset {
    pub(crate) unsafe fn new(
        data: &[u8],
        tile_size: (usize, usize),
    ) -> Self {
        let (tile_width, tile_height) = tile_size;
        let (info, data) = stbi_load_from_memory(data, Channels::RgbAlpha)
            .expect("Couldn't load image");
    
        let tiles_x = info.width as usize / tile_width;
        let tiles_y = info.height as usize / tile_height;
        let image_count = tiles_x * tiles_y;
        
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);
    
        gl::TexImage3D(
            gl::TEXTURE_2D_ARRAY,
            0,
            gl::RED as _,
            tile_width as _,
            tile_height as _,
            image_count as _,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            null(),
        );
    
        let tile_size_x = tile_width * 4;
        let row_length = tiles_x * tile_size_x;
        let mut tile_buffer = vec![0u8; tile_width * tile_height * 4];
    
        for y in 0..tiles_y {
            for x in 0..tiles_x {
                let i = y * row_length * tile_height + x * tile_size_x;
                for row_in_tile in 0..tile_height {
                    for si in 0..tile_size_x {
                        tile_buffer[row_in_tile * tile_size_x + si] = data.as_slice()[i + row_in_tile * row_length + si]
                    }
                }
                let zoffset = y * tiles_x + x;
                gl::TexSubImage3D(
                    gl::TEXTURE_2D_ARRAY, 0,
                    0, 0, zoffset as _,
                    tile_width as _, tile_height as _, 1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    tile_buffer.as_ptr() as _,
                )
            }
        }

        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        Self { id }
    }

    pub(crate) unsafe fn bind(&self, index: u32) {
        gl::ActiveTexture(gl::TEXTURE0 + index);
        gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
    }
}