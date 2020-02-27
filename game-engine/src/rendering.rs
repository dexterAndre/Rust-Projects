#[allow(dead_code)]
pub mod open_gl {
    // Dependencies
    pub use gl::{ self, types::* };
    pub use glfw::{ self, Action, Context, Key, Window };
    use std::{ self, ffi::CString, ffi::CStr, ptr, mem, path::Path, os::raw::c_void, sync::mpsc::Receiver };
    pub use crate::mathematics::linalg::{ self, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4 };

    // Settings
    pub const scr_width: u32 = 800;
    pub const scr_height: u32 = 600;

    // Classes
    //      Program
    pub struct Program {
        id: GLuint,
    }
    impl Program {
        pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
            let program_id = unsafe { gl::CreateProgram() };

            for shader in shaders {
                unsafe { gl::AttachShader(program_id, shader.id()); }
            }

            unsafe { gl::LinkProgram(program_id); }

            let mut success: gl::types::GLint = 1;
            unsafe {
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            }

            if success == 0 {
                let mut len: gl::types::GLint = 0;
                unsafe {
                    gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = create_whitespace_cstring_with_len(len as usize);

                unsafe {
                    gl::GetProgramInfoLog(
                        program_id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }

                return Err(error.to_string_lossy().into_owned());
            }

            for shader in shaders {
                unsafe { gl::DetachShader(program_id, shader.id()); }
            }

            Ok(Program { id: program_id })
        }

        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }
    
        pub fn set_used(&self) {
            unsafe {
                gl::UseProgram(self.id);
            }
        }

        pub fn set_mat4(&self, name: &str, mat: *const Matrix4) {
            unsafe {
                let mat_loc = gl::GetUniformLocation(self.id(), name.as_ptr() as (*const i8));
                gl::UniformMatrix4fv(mat_loc, 1, gl::FALSE, mat as (*const gl::types::GLfloat));
            }
        }

        pub unsafe fn set_bool(&self, name: &str, value: bool) {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), value as i32);
        }
        pub unsafe fn set_int(&self, name: &str, value: i32) {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), value);
        }
        pub unsafe fn set_float(&self, name: &str, value: f32) {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), value);
        }
        pub unsafe fn set_vector3(&self, name: &str, value: &Vector3) {
            gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 1, value.as_ptr());
        }
        pub unsafe fn set_vector4(&self, name: &str, value: &Vector4) {
            gl::Uniform4fv(gl::GetUniformLocation(self.id, name.as_ptr() as *const i8), 1, value.as_ptr());
        }
    }
    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
    
    pub struct Shader {
        id: GLuint,
    }
    impl Shader {
        pub fn from_source(
            source: &CStr,
            kind: gl::types::GLenum
        ) -> Result<Shader, String> {
            let id = shader_from_source(source, kind)?;
            return Ok(Shader { id });
        }
    
        pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
            return Shader::from_source(source, gl::VERTEX_SHADER);
        }
        
        pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
            return Shader::from_source(source, gl::FRAGMENT_SHADER);
        }
    
        pub fn id(&self) -> gl::types::GLuint {
            return self.id;
        }
    }
    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }
    
    pub fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
    
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
    
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
    
            let error = create_whitespace_cstring_with_len(len as usize);
    
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
    
            return Err(error.to_string_lossy().into_owned());
        }
    
        Ok(id)
    }
    
    pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len));
        // convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
    }
}

// #[allow(dead_code)]
// pub mod open_gl {
//     pub use std::{ self, ffi::CString, ffi::CStr };
//     pub use gl::{ self, types::* };
//     pub use glfw::{ self, Action, Context, Key, Window };
//     pub use crate::mathematics::{
//         linalg, 
//         linalg::Vector2,
//         linalg::Vector3,
//         linalg::Vector4,
//         linalg::Matrix2, 
//         linalg::Matrix3,
//         linalg::Matrix4,
//     };

//     pub struct Program {
//         id: gl::types::GLuint,
//     }
    
//     impl Program {
//         pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
//             let program_id = unsafe { gl::CreateProgram() };
    
//             for shader in shaders {
//                 unsafe { gl::AttachShader(program_id, shader.id()); }
//             }
    
//             unsafe { gl::LinkProgram(program_id); }
    
//             let mut success: gl::types::GLint = 1;
//             unsafe {
//                 gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
//             }
    
//             if success == 0 {
//                 let mut len: gl::types::GLint = 0;
//                 unsafe {
//                     gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
//                 }
    
//                 let error = create_whitespace_cstring_with_len(len as usize);
    
//                 unsafe {
//                     gl::GetProgramInfoLog(
//                         program_id,
//                         len,
//                         std::ptr::null_mut(),
//                         error.as_ptr() as *mut gl::types::GLchar
//                     );
//                 }
    
//                 return Err(error.to_string_lossy().into_owned());
//             }
    
//             for shader in shaders {
//                 unsafe { gl::DetachShader(program_id, shader.id()); }
//             }
    
//             Ok(Program { id: program_id })
//         }
    
//         pub fn id(&self) -> gl::types::GLuint {
//             self.id
//         }
    
//         pub fn set_used(&self) {
//             unsafe {
//                 gl::UseProgram(self.id);
//             }
//         }

//         pub fn set_mat4(&self, name: &str, mat: *const Matrix4) {
//             unsafe {
//                 let mat_loc = gl::GetUniformLocation(self.id(), name.as_ptr() as (*const i8));
//                 gl::UniformMatrix4fv(mat_loc, 1, gl::FALSE, mat as (*const gl::types::GLfloat));
//             }
//         }
//     }
    
//     impl Drop for Program {
//         fn drop(&mut self) {
//             unsafe {
//                 gl::DeleteProgram(self.id);
//             }
//         }
//     }
    
//     pub struct Shader {
//         id: gl::types::GLuint,
//     }
    
//     impl Shader {
//         pub fn from_source(
//             source: &CStr,
//             kind: gl::types::GLenum
//         ) -> Result<Shader, String> {
//             let id = shader_from_source(source, kind)?;
//             return Ok(Shader { id });
//         }
    
//         pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
//             return Shader::from_source(source, gl::VERTEX_SHADER);
//         }
        
//         pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
//             return Shader::from_source(source, gl::FRAGMENT_SHADER);
//         }
    
//         pub fn id(&self) -> gl::types::GLuint {
//             return self.id;
//         }
//     }
    
//     impl Drop for Shader {
//         fn drop(&mut self) {
//             unsafe {
//                 gl::DeleteShader(self.id);
//             }
//         }
//     }
    
//     pub fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
//         let id = unsafe { gl::CreateShader(kind) };
//         unsafe {
//             gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
//             gl::CompileShader(id);
//         }
    
//         let mut success: gl::types::GLint = 1;
//         unsafe {
//             gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
//         }
    
//         if success == 0 {
//             let mut len: gl::types::GLint = 0;
//             unsafe {
//                 gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
//             }
    
//             let error = create_whitespace_cstring_with_len(len as usize);
    
//             unsafe {
//                 gl::GetShaderInfoLog(
//                     id,
//                     len,
//                     std::ptr::null_mut(),
//                     error.as_ptr() as *mut gl::types::GLchar,
//                 );
//             }
    
//             return Err(error.to_string_lossy().into_owned());
//         }
    
//         Ok(id)
//     }
    
//     pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
//         // allocate buffer of correct size
//         let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
//         // fill it with len spaces
//         buffer.extend([b' '].iter().cycle().take(len));
//         // convert buffer to CString
//         unsafe { CString::from_vec_unchecked(buffer) }
//     }
// }