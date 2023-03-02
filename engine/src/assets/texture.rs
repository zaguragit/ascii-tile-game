
pub(crate) struct TextureBuffer {
    id: u32,
}

impl Drop for TextureBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

impl TextureBuffer {
    pub(crate) unsafe fn new_r(
        data: &[u8],
        text_width: usize,
        text_height: usize,
    ) -> Self {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
    
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as _,
            text_width as _,
            text_height as _,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as _,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        Self { id }
    }

    pub(crate) unsafe fn new_rgba(
        data: &[u32],
        text_width: usize,
        text_height: usize,
    ) -> Self {
        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as _,
            text_width as _,
            text_height as _,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as _,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        Self { id }
    }

    pub(crate) unsafe fn bind(&self, index: u32) {
        gl::ActiveTexture(gl::TEXTURE0 + index);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}