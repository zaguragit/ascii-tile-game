use std::{ptr::null, ffi::c_void};

pub(crate) struct Mesh {
    id: u32,
    indices: u32,
    vertices: u32,
    vertex_count: i32,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.bind();
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        
            gl::DisableVertexAttribArray(0);
            gl::DeleteBuffers(2, [self.vertices, self.indices].as_ptr());
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl Mesh {
    pub(crate) unsafe fn make_quad() -> Self {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        let mut vertices_id: u32 = 0;
        gl::GenBuffers(1, &mut vertices_id);
        let mut indices_id = 0;
        gl::GenBuffers(1, &mut indices_id);

        gl::BindVertexArray(vao);

        let vertices: [f32; 8] = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
        ];

        let indices: [u32; 6] = [0, 1, 3, 3, 1, 2];

        gl::BindBuffer(gl::ARRAY_BUFFER, vertices_id);
        gl::BufferData(gl::ARRAY_BUFFER, 8 * 4, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);  

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_id);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 6 * 4, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        // vertex positions
        gl::EnableVertexAttribArray(0);	
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4, null());

        gl::BindVertexArray(0);
        Self {
            id: vao,
            indices: indices_id,
            vertices: vertices_id,
            vertex_count: 6,
        }
    }

    pub(crate) unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.indices);
    
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertices);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, null());
    }

    pub(crate) unsafe fn draw(&self) {
        gl::DrawElements(gl::TRIANGLES, self.vertex_count, gl::UNSIGNED_INT, null());
    }
}