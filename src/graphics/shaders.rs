pub fn compile_shader(src: &'static str, kind: u32) -> Result<u32, String> {
    unsafe {
        let shader = gl::CreateShader(kind);
        gl::ShaderSource(shader, 1, &(src.as_ptr() as * const i8), &(src.len() as i32));
        gl::CompileShader(shader);

        let mut ok = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
            gl::DeleteShader(shader);
            return Err(String::from_utf8_lossy(&buf).into_owned());
        }
        Ok(shader)
    }
}

pub fn link_program(vs: u32, fs: u32) -> Result<u32, String> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut ok = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
            gl::DeleteProgram(program);
            return Err(String::from_utf8_lossy(&buf).into_owned());
        }
        Ok(program)
    }
}

pub fn get_attrib_location(program: u32, name: &str) -> Result<u32, String> {
    let c_str = std::ffi::CString::new(name).unwrap();
    unsafe { 
        let location = gl::GetAttribLocation(program, c_str.as_ptr());
        if location < 0 {
            return Err(format!("Attribute {name} was not found or is inactive"))
        }
        Ok(location as u32)
    }
}

pub fn get_uniform_location(program: u32, name: &str) -> Result<u32, String> {
    let c_str = std::ffi::CString::new(name).unwrap();
    unsafe { 
        let location = gl::GetUniformLocation(program, c_str.as_ptr());
        if location < 0 {
            return Err(format!("Uniform {name} was not found"))
        }
        Ok(location as u32)
    }
}