use glam::{Vec2, Vec3, Vec4, Mat4};


pub(crate) struct Shader {
    id: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

impl Shader {
    pub(crate) unsafe fn new(vertex_code: &str, fragment_code: &str) -> Self {
        let program_id = gl::CreateProgram();
        let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);

        gl::ShaderSource(vertex_id, 1, [vertex_code.as_ptr() as *const i8].as_ptr(), [vertex_code.len() as i32].as_ptr());
        gl::CompileShader(vertex_id);
        if get_shader_i(vertex_id, gl::COMPILE_STATUS) == 0 {
            panic!("Vertex:\n{}", get_shader_log(vertex_id));
        }

        gl::ShaderSource(fragment_id, 1, [fragment_code.as_ptr() as *const i8].as_ptr(), [fragment_code.len() as i32].as_ptr());
        gl::CompileShader(fragment_id);
        if get_shader_i(fragment_id, gl::COMPILE_STATUS) == 0 {
            panic!("Fragment:\n{}", get_shader_log(fragment_id));
        }

        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);

        gl::LinkProgram(program_id);
        if get_program_i(program_id, gl::LINK_STATUS) == 0 {
            panic!("Linking:\n{}", get_shader_log(program_id));
        }

        gl::DetachShader(program_id, vertex_id);
        gl::DetachShader(program_id, fragment_id);
        gl::DeleteShader(vertex_id);
        gl::DeleteShader(fragment_id);

        gl::ValidateProgram(program_id);
        if get_program_i(program_id, gl::VALIDATE_STATUS) == 0 {
            panic!("Validation:\n{}", get_shader_log(program_id));
        }
        
        Self { id: program_id }
    }

    pub(crate) unsafe fn bind(&mut self) {
        gl::UseProgram(self.id)
    }

    pub(crate) unsafe fn setf(&mut self, name: &str, value: f32) {
        gl::Uniform1f(self.get_location(name), value)
    }
    pub(crate) unsafe fn seti(&mut self, name: &str, value: i32) {
        gl::Uniform1i(self.get_location(name), value)
    }

    pub(crate) unsafe fn setv2(&mut self, name: &str, value: Vec2) {
        gl::Uniform2f(self.get_location(name), value.x, value.y)
    }
    pub(crate) unsafe fn setv3(&mut self, name: &str, value: Vec3) {
        gl::Uniform3f(self.get_location(name), value.x, value.y, value.z)
    }
    pub(crate) unsafe fn setv4(&mut self, name: &str, value: Vec4) {
        gl::Uniform4f(self.get_location(name), value.x, value.y, value.z, value.w)
    }
    pub(crate) unsafe fn setm4(&mut self, name: &str, value: Mat4) {
        gl::UniformMatrix4fv(self.get_location(name), 1, gl::FALSE, value.to_cols_array().as_ptr())
    }

    unsafe fn get_location(&self, name: &str) -> i32 {
        gl::GetUniformLocation(self.id, (name.to_owned() + "\0").as_ptr() as _)
    }
}


unsafe fn get_shader_log(id: u32) -> String {
    let mut max_length: i32 = 0;
    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut max_length);
    let mut c = vec![0i8; max_length as usize];
    let mut l: i32 = 0;
    gl::GetShaderInfoLog(id, max_length, &mut l, c.as_mut_ptr());
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(c.as_ptr() as *const u8, l as usize)).to_string()
}

unsafe fn get_shader_i(id: u32, param: u32) -> i32 {
    let mut p: i32 = 0;
    gl::GetShaderiv(id, param, &mut p);
    p
}

unsafe fn get_program_i(id: u32, param: u32) -> i32 {
    let mut p: i32 = 0;
    gl::GetProgramiv(id, param, &mut p);
    p
}