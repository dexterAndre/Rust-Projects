// Simple Rust game engine by Dexter André Osiander (2019)
#![allow(dead_code)]
#![allow(warnings)]


// Help
//      Rust + GLFW + OpenGL small sample program: https://www.reddit.com/r/rust_gamedev/comments/e03qwz/help_how_to_multithreading_glfw_cursorpos_callback/


// pub use crate::lib::Time::{self, TimerScoped};
mod system_time;
use crate::system_time::time;
mod rendering;
use crate::rendering::open_gl::{ Action, Context, Key, Window };
// mod render_gl;
mod mathematics;



/*

fn main() {
    // Initialization
    {
        // Engine setup
        // Window setup
        // Input setup
        // Renderer setup
    }

    // Game loop
    {
        // Log time
        // Handle input
        // Game logic
        // Rendering
        // Loop control
    }
}

*/


fn main() {
    test_math();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        // glfw::WindowEvent::Key(Key::Enter, _, Action::Release, _) => {
        //     println!("Hello, world!");
        // }
        _ => {}
    }
}

fn test_rendering() {
    // Time
    let mut time_current: u128 = time::now_ms();
    let mut time_delta: u128 = 0;
    
    // Window & OpenGL
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    //      Setting lowest OpenGL version
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    //      Setting profile
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core,));
    let (mut window, events) = glfw
    .create_window(800, 600, "Game Engine", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    window.make_current();
    
    //      Setting up OpenGL
    //          Loading gl functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }
    
    //      Setting up shaders
    use std::ffi::CString;

    let vert_shader = rendering::open_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();
    let frag_shader = rendering::open_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    //      Setting up shader program
    let shader_program = rendering::open_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();
    shader_program.set_used();

    //      Setting up triangle
    let vertices: Vec<f32> = vec![
        // positions            // colors
        -0.5,   -0.5,    0.0,   1.0,    0.0,    0.0,    // bottom right
         0.5,   -0.5,    0.0,   0.0,    1.0,    0.0,    // bottom left
         0.0,    0.5,    0.0,   0.0,    0.0,    1.0     // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    // Data buffers
    unsafe {
        // Binding buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        // Feeding buffer data
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Unbind buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // Data layout
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        // Binding buffer
        gl::BindVertexArray(vao);

        // Feeding buffer data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        //      (same as "layout = 0" in vertex shader)
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,  // index of the generic vertex attribute ("layout (location = 0)")
            3,  // number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,   // stride (byte offset between consecutive attributes)
            std::ptr::null()    // offset of the first component
        );
        //      we put color information at (location = 1)
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,  // index of the generic vertex attribute ("layout (location = 0)")
            3,  // RGB = 3
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,   // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        // Unbinding vbo and vao
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }


    
    //      Render loop
    while !window.should_close() {
        // Logging time
        time_delta = time::now_ms() - time_current;
        time_current = time::now_ms();
        
        // Handling input
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        
        // let t = TimerScoped::new();
        // {
        //     for i in 0..10_000 {
        //         let a = 771415334 - 350105234;
        //         // println!("Hello, world! {}", i);
        //     }
        // }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }

        
        // Drawing geometry
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        
        window.swap_buffers();
        
        
        // Setting max fps
        // let t = Time::now_ms();
        // while (Time::now_ms() - t) < (1000 / 10) {}
        
        // println!("Time::now_ms(): {}", Time::now_ms());
        // println!("time_current: {}", time_current);
        // println!("time_delta: {}", time_delta);
    }
}

fn test_math() {
    pub use mathematics::linalg::vector::Vector2;

    let a = Vector2::from_floats(3.0, 4.0);
    let b = Vector2::from_floats(1.0, 5.0);

    println!("a: {:?}, b: {:?}", a, b);
}