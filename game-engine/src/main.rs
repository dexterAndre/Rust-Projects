// Simple Rust game engine by Dexter André Osiander (2020)
#![allow(dead_code)]
#![allow(warnings)]


// Help
//      Rust + GLFW + OpenGL small sample program: https://www.reddit.com/r/rust_gamedev/comments/e03qwz/help_how_to_multithreading_glfw_cursorpos_callback/


mod system_time;
use crate::system_time::time;
mod rendering;
use crate::rendering::open_gl::{ Action, Context, Key, Window };
// mod render_gl;
mod mathematics;

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

pub struct entity {
    id: i32
}

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


    // test_vector2();
    // test_math_profiling();
    test_rendering();
    // test_array();
}

fn process_input(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::Enter, _, Action::Release, _) => {
            println!("Hello, world!");
        }
        _ => {}
    }
}



fn render() {

}

fn test_array() {
    let mut arr = [[0.0; 4]; 4];
    arr =  [[1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]];
    
    for i in 0..4 {
        println!("row[{}]: [{}, {}, {}, {}]", i, arr[i][0], arr[i][1], arr[i][2], arr[i][3]);
    }

    use mathematics::linalg::Matrix2;
    use mathematics::linalg::Matrix3;
    use mathematics::linalg::Matrix4;
    let mut mat = Matrix4::new(
        1.0, 2.0, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0);
    
    println!("mat: {}", mat.to_string());
    println!("mat*: {}", (-mat).to_string());
    for i in 0..4 {
        println!("mat[{}]: {}", i, mat.column(i).to_string());
    }
    println!("mat.dia(): {}", mat.diagonal().to_string());

    let mat2 = Matrix2::new(
        2.0, 5.0,
        7.0, 8.0);
    println!("mat2.triangular_upper(): {}", mat2.triangular_upper().to_string());
    println!("mat2.det: {}", mat2.determinant().to_string());
    println!("mat2.det2: {}", mat2.determinant2().to_string()); 

    let mat3 = Matrix3::new(
        1.0, 2.0, -6.0,
        -3.0, 4.0, -1.0,
        0.0, -5.0, 7.0);
    println!("mat3: {}", mat3.to_string()); 
    println!("mat3.triangular_upper(): {}", mat3.triangular_upper().to_string());
    println!("mat3.det: {}", mat3.determinant().to_string());
    println!("mat3.det2: {}", mat3.determinant2().to_string());

    println!("mat3.minor(0, 0): {}", mat3.minor(0, 0).to_string());
    println!("mat3.minor(0, 0).det(): {}", mat3.minor(0, 0).determinant().to_string());
    println!("mat3.cofactor(0, 0): {}", mat3.cofactor(0, 0).to_string());
    println!("mat3.minor(0, 1): {}", mat3.minor(0, 1).to_string());
    println!("mat3.minor(0, 1).det(): {}", mat3.minor(0, 1).determinant().to_string());
    println!("mat3.cofactor(0, 1): {}", mat3.cofactor(0, 1).to_string());
    println!("mat3.minor(0, 2): {}", mat3.minor(0, 2).to_string());
    println!("mat3.minor(0, 2).det(): {}", mat3.minor(0, 2).determinant().to_string());
    println!("mat3.cofactor(0, 2): {}", mat3.cofactor(0, 2).to_string());
    
    println!("mat3.minor(1, 0): {}", mat3.minor(1, 0).to_string());
    println!("mat3.minor(1, 0).det(): {}", mat3.minor(1, 0).determinant().to_string());
    println!("mat3.cofactor(1, 0): {}", mat3.cofactor(1, 0).to_string());
    println!("mat3.minor(1, 1): {}", mat3.minor(1, 1).to_string());
    println!("mat3.minor(1, 1).det(): {}", mat3.minor(1, 1).determinant().to_string());
    println!("mat3.cofactor(1, 1): {}", mat3.cofactor(1, 1).to_string());
    println!("mat3.minor(1, 2): {}", mat3.minor(1, 2).to_string());
    println!("mat3.minor(1, 2).det(): {}", mat3.minor(1, 2).determinant().to_string());
    println!("mat3.cofactor(1, 2): {}", mat3.cofactor(1, 2).to_string());
    
    println!("mat3.minor(2, 0): {}", mat3.minor(2, 0).to_string());
    println!("mat3.minor(2, 0).det(): {}", mat3.minor(2, 0).determinant().to_string());
    println!("mat3.cofactor(2, 0): {}", mat3.cofactor(2, 0).to_string());
    println!("mat3.minor(2, 1): {}", mat3.minor(2, 1).to_string());
    println!("mat3.minor(2, 1).det(): {}", mat3.minor(2, 1).determinant().to_string());
    println!("mat3.cofactor(2, 1): {}", mat3.cofactor(2, 1).to_string());
    println!("mat3.minor(2, 2): {}", mat3.minor(2, 2).to_string());
    println!("mat3.minor(2, 2).det(): {}", mat3.minor(2, 2).determinant().to_string());
    println!("mat3.cofactor(2, 2): {}", mat3.cofactor(2, 2).to_string());

    println!("mat3.cofactor_matrix: {}", mat3.cofactor_matrix().to_string());

    let mat4 = Matrix4::new(
        2.0, 4.0, 5.0, 8.0,
        -1.0, 7.0, -2.0, 7.0,
        0.0, 11.0, -1.0, 6.0,
        3.0, -9.0, -3.0, 2.0);
    println!("mat4: {}", mat4.to_string());
    println!("mat4.triangular_upper(): {}", mat4.triangular_upper().to_string());
    println!("mat4.det: {}", mat4.determinant().to_string());
    println!("mat4.det2: {}", mat4.determinant2().to_string());
}

fn test_rendering() {
    // Dependencies
    pub use mathematics::num;
    pub use mathematics::num::constants as constants;
    pub use mathematics::linalg::*;
    pub use rendering::open_gl;
    pub use gl::{ self, types::* };
    use std::{ ffi::CStr, ffi::CString, os::raw::c_void, path::Path, mem, ptr };
    use image;
    use image::GenericImage;
    // use cgmath::{Matrix4, Vector3, vec3,  Deg, Rad, perspective};
    use cgmath::prelude::*;

    // Time
    let mut time_current: u128 = time::now_ms();
    let mut time_delta: u128 = 0;
    
    // Window & OpenGL
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    //      Setting lowest OpenGL version
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    //      Setting profile
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    //      Window creation
    let (mut window, events) = glfw
        .create_window(
            open_gl::scr_width, 
            open_gl::scr_height, 
            "Game Engine", 
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
    
    //      Setting up OpenGL
    //          Loading gl functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }
    
    //      Setting up shaders
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
    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
   ];
   // world space positions of our cubes
   let cubePositions: [Vector3; 10] = [
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(2.0, 5.0, -15.0),
        Vector3::new(-1.5, -2.2, -2.5),
        Vector3::new(-3.8, -2.0, -12.3),
        Vector3::new(2.4, -0.4, -3.65),
        Vector3::new(-1.7, 3.0, -7.35),
        Vector3::new(1.3, -2.0, -2.25),
        Vector3::new(1.5, 2.0, -2.15),
        Vector3::new(1.5, 0.2, -1.35),
        Vector3::new(-1.3, 1.0, -1.25)
    ];

    // Generating, binding, and filling buffers
    //      1. Creating empty buffer variables
    let mut VAO: gl::types::GLuint = 0;
    let mut VBO: gl::types::GLuint = 0;
    let mut EBO: gl::types::GLuint = 0;
    let (mut tex_1, mut tex_2) = (0, 0);

    unsafe {
        //      2. Generating buffers in OpenGL
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        //      3.  Binding the vertex array
        gl::BindVertexArray(VAO);

        //      4.  Bind and set vertex buffers
        //      4.1 Vertex buffer objct
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );
        // gl::BufferData(
        //     gl::ARRAY_BUFFER,
        //     (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        //     &vertices[0] as *const f32 as *const c_void,
        //     gl::STATIC_DRAW,
        // );
        //      4.2 Element buffer object
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER,
        //     (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        //     &indices[0] as *const i32 as *const c_void,
        //     gl::STATIC_DRAW
        // );

        //      5.1 Configure vertex attributes
        //          Same as "layout = 0" in vertex shader
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,                                                  // index of the generic vertex attribute ("layout (location = 0)")
            3,                                                  // number of components per generic vertex attribute
            gl::FLOAT,                                          // data type
            gl::FALSE,                                          // normalized (int-to-float conversion)
            (5 * std::mem::size_of::<GLfloat>()) as GLsizei,    // stride (byte offset between consecutive attributes)
            std::ptr::null()                                    // offset of the first component
        );

        //      5.2 Setting up texture coordinates
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void
        );

        //      5.3 Texture 1
        gl::GenTextures(1, &mut tex_1);
        gl::BindTexture(gl::TEXTURE_2D, tex_1);
        //      5.3.1 Wrapping mode
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        //      5.3.2 Filtering mode
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        //      5.3.3 Reading image into texture
        let img = image::open(&Path::new("resources/textures/wall.jpg")).expect("Failed to load texture.");
        let img_data = img.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &img_data[0] as *const u8 as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        //      5.4 Texture 2
        gl::GenTextures(1, &mut tex_2);
        gl::BindTexture(gl::TEXTURE_2D, tex_2);
        //      5.4.1 Wrapping mode
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        //      5.4.2 Filtering mode
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        //      5.4.3 Reading image into texture
        let img = image::open(&Path::new("resources/textures/awesomeface.png")).expect("Failed to load texture.");
        let img_data = img.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &img_data[0] as *const u8 as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader_program.set_int(c_str!("tex_1"), 0);
        shader_program.set_int(c_str!("tex_2"), 1);
        //      6. Unbinding buffers
        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        // gl::BindVertexArray(0);
    }
    
    //      Render loop
    while !window.should_close() {
        // Logging time
        time_delta = time::now_ms() - time_current;
        time_current = time::now_ms();
        
        // Handling input
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            process_input(&mut window, event);
        }

        // Clearing color
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        // Setting up textures
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex_1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, tex_2);
        }

        // Drawing geometry
        shader_program.set_used();

        // Transformations
        let model = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        // let model: Matrix4 = Matrix4::identity();
        let view = Matrix4::translation(Vector3::new(0.0, 0.0, -5.0));
        // let projection = Matrix4::identity();
        let projection = Matrix4::perspective(45.0, (open_gl::scr_width as f32) / (open_gl::scr_height as f32), 0.1, 100.0);
        // let projection = Matrix4::perspective(60.0, 1.0, 1.0, 100.0);
        // let projection = Matrix4::new(
        //     0.2, 0.0, 0.0, 0.0,
        //     0.0, 0.2, 0.0, 0.0,
        //     0.0, 0.0, 0.0101, 1.0101,
        //     0.0, 0.0, 0.0, 1.0
        // );
        // let projection: cgmath::Matrix4<f32> = cgmath::perspective(cgmath::Deg(45.0), open_gl::scr_width as f32 / open_gl::scr_height as f32, 0.1, 100.0);
        {
            // println!("{}", projection.to_string());
        }
        
        let mut model_loc: GLint = 0; 
        let mut view_loc: GLint = 0;
        let mut proj_loc: GLint = 0;

        unsafe {
            model_loc = gl::GetUniformLocation(shader_program.id(), c_str!("model").as_ptr() as *const GLchar);
            view_loc = gl::GetUniformLocation(shader_program.id(), c_str!("view").as_ptr() as *const GLchar);
            proj_loc = gl::GetUniformLocation(shader_program.id(), c_str!("projection").as_ptr() as *const GLchar);
                        
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());
        }
            
        // shader_program.set_mat4("Model", transform);
        // shader_program.set_mat4("View", camera);

        // Drawing geometry
        unsafe {
            gl::BindVertexArray(VAO);
            // gl::DrawArrays(
            //     gl::TRIANGLES, // mode
            //     0, // starting index in the enabled arrays
            //     3 // number of indices to be rendered
            // );
            // gl::DrawElements(
            //     gl::TRIANGLES,      // mode
            //     36,                  // ?
            //     gl::UNSIGNED_INT,  // ?
            //     0 as *const gl::types::GLvoid
            // );
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);

            for (i, position) in cubePositions.iter().enumerate() {
                let mut model = Matrix4::translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::scale_uniform(1.0) * Matrix4::rotation(angle, Vector3::new(1.0, 0.3, 0.5).normalization());
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        
        window.swap_buffers();
        
        
        // Setting max fps
        // let t = Time::now_ms();
        // while (Time::now_ms() - t) < (1000 / 10) {}
        
        // println!("Time::now_ms(): {}", Time::now_ms());
        // println!("time_current: {}", time_current);
        // println!("time_delta: {}", time_delta);
    }

    // Deallocating resources
    unsafe {
        gl::DeleteVertexArrays(1, &VAO);
        gl::DeleteBuffers(1, &VBO);
        gl::DeleteBuffers(1, &EBO);
    }
}

fn test_math_profiling() {
    pub use mathematics::linalg::Vector2;

    let a = Vector2::new(3.0, 4.0);
    let b = Vector2::new(1.0, 5.0);

    println!("a: {:?}, b: {:?}", a, b);

    let iterations = 100_000_000;
    
    println!("Vector angle unit: ");
    {
        let t = time::TimerScoped::new();
        for i in 0..iterations {
            let test_angle_unit = Vector2::angle_unit(&a, &b);
        }
    }
    
    println!("Vector angle: ");
    {
        let t = time::TimerScoped::new();
        for i in 0..iterations {
            let test_angle = Vector2::angle(&a, &b);
        }
    }

    println!("Vector angle safe: ");
    {
        let t = time::TimerScoped::new();
        for i in 0..iterations {
            let test_angle_safe = Vector2::angle_safe(&a, &b);
        }
    }
}

fn test_vector2() {
    use mathematics::linalg::Vector2;
    use mathematics::linalg::Matrix2;
    use mathematics::num::constants as num;

    // Construction
    println!("===== CONSTRUCTION =====");
    let mut a = Vector2::new(3.0, 4.0);
    println!("a = Vector2::new(3.0, 4.0) -> {}", a.to_string());
    let mut b = Vector2::new(5.0, 1.0);
    println!("b = Vector2::new(5.0, 1.0) -> {}", b.to_string());
    let mut c = Vector2::from_vector2(&a);
    println!("c = Vector2::from_vector(&a) -> {}", c.to_string());
    let mut d = Vector2::from_polar(num::PIOVER3, 1.0);
    println!("d = Vector2::from_polar(num::PIOVER3, 1.0) -> {}", d.to_string());
    
    // Prefabrication
    println!("===== PREFABRICATION =====");
    //      Mutable for later usage
    let mut e = Vector2::one();
    println!("e = Vector2::one() -> {}, mag: {}", e.to_string(), e.magnitude());
    let f = Vector2::zero();
    println!("f = Vector2::zero() -> {}, mag: {}", f.to_string(), f.magnitude());
    let g = Vector2::right();
    println!("g = Vector2::right() -> {}, mag: {}", g.to_string(), g.magnitude());
    let h = Vector2::left();
    println!("h = Vector2::left() -> {}, mag: {}", h.to_string(), h.magnitude());
    let i = Vector2::forth();
    println!("i = Vector2::forth() -> {}, mag: {}", i.to_string(), i.magnitude());
    let j = Vector2::back();
    println!("j = Vector2::back() -> {}, mag: {}", j.to_string(), j.magnitude());
    let k = Vector2::Q1();
    println!("k = Vector2::Q1() -> {}, mag: {}", k.to_string(), k.magnitude());
    let l = Vector2::Q1n();
    println!("l = Vector2::Q1n() -> {}, mag: {}", l.to_string(), l.magnitude());
    let m = Vector2::Q2();
    println!("m = Vector2::Q2() -> {}, mag: {}", m.to_string(), m.magnitude());
    let n = Vector2::Q2n();
    println!("n = Vector2::Q2n() -> {}, mag: {}", n.to_string(), n.magnitude());
    let o = Vector2::Q3();
    println!("o = Vector2::Q3() -> {}, mag: {}", o.to_string(), o.magnitude());
    let p = Vector2::Q3n();
    println!("p = Vector2::Q3n() -> {}, mag: {}", p.to_string(), p.magnitude());
    let q = Vector2::Q4();
    println!("q = Vector2::Q4() -> {}, mag: {}", q.to_string(), q.magnitude());
    let r = Vector2::Q4n();
    println!("r = Vector2::Q4n() -> {}, mag: {}", r.to_string(), r.magnitude());
    
    // Swizzling
    println!("===== SWIZZLING =====");
    println!("a = {}", a.to_string());
    let s = a.xx();
    println!("s = a.xx() -> {}", s.to_string());
    let t = a.xy();
    println!("t = a.xy() -> {}", t.to_string());
    let u = a.yx();
    println!("u = a.yx() -> {}", u.to_string());
    let v = a.yy();
    println!("v = a.yy() -> {}", v.to_string());
    
    // Vector arithmetic
    println!("===== VECTOR ARITHMETIC =====");
    println!("a = {}", a.to_string());
    println!("b = {}", b.to_string());
    println!("c = {}", c.to_string());
    println!("d = {}", d.to_string());
    
    println!("=== VECTOR ADDITION ===");
    println!("a + b = {}", (a + b).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    println!("b = {}", (b).to_string());
    a += b;
    println!("a += b {}", (a).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    println!("b = {}", (b).to_string());
    a = Vector2::new(3.0, 4.0);
    println!("Resetting: a = {}", a.to_string());
    
    println!("=== VECTOR SUBTRACTION ===");
    println!("a - b = {}", (a - b).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    println!("b = {}", (b).to_string());
    a -= b;
    println!("a -= b {}", (a).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    println!("b = {}", (b).to_string());
    a = Vector2::new(3.0, 4.0);
    println!("Resetting: a = {}", a.to_string());
    println!("-a = {}", (-a).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    
    println!("=== SCALAR-VECTOR MULTIPLICATION ===");
    println!("a * 2.0 = {}", (a * 2.0).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    a *= 2.0;
    println!("a *= 2.0 = {}", a.to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    a = Vector2::new(3.0, 4.0);
    println!("Resetting: a = {}", a.to_string());
    println!("2.0 * a = {}", (2.0 * a).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    
    println!("=== VECTOR-VECTOR MULTIPLICATION ===");
    println!("a = {}", a.to_string());
    println!("b = {}", b.to_string());
    println!("a * b (scalar; dot) = {}", (a * b));
    println!("a / b (vector; cross) = {}", (a / b));
    // println!("a ^ b (geometric; wedge) = {}", (a ^ b));
    
    println!("=== SCALAR-VECTOR DIVISION ===");
    println!("a / 2.0 = {}", (a / 2.0).to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    a /= 2.0;
    println!("a /= 2.0 = {}", a.to_string());
    println!("(Ensuring ownership is correct...");
    println!("a = {}", (a).to_string());
    a = Vector2::new(3.0, 4.0);
    println!("Resetting: a = {}", a.to_string());
    
    println!("=== MAGNITUDE ===");
    println!("a = {}", (a).to_string());
    println!("a.mag(): {}", a.magnitude());
    println!("a.mag_sqr(): {}", a.magnitude_sqr());
    println!("a.normalization(): {}", a.normalization().to_string());
    println!("a = {}", (a).to_string());
    // a.normalize();
    // println!("a.normalize(): {}", a.to_string());
    a = Vector2::new(3.0, 4.0);
    println!("Resetting: a = {}", a.to_string());
    
    println!("=== LERP ===");
    println!("a = {}, b = {}", a.to_string(), b.to_string());
    println!("Vector2::lerp(a, b, 0.25): {}", Vector2::lerp(&a, &b, 0.25).to_string());
    // println!("Vector2::slerp(a, b, 0.25): {}", Vector2::slerp(&a, &b, 0.25).to_string());
    
    println!("=== ANGLE ===");
    a = Vector2::new(3.0, 4.0);
    b = Vector2::new(5.0, 1.0);
    c = Vector2::new(-4.0, 3.0);
    d = Vector2::new(-3.0, -4.0);
    e = Vector2::new(-1.0, -3.0);

    println!("Resetting: a = {}, b = {}, c = {}, d = {}, e = {}", a.to_string(), b.to_string(), c.to_string(), d.to_string(), e.to_string());
    println!("Vector2::angle(x, a): {}r, {}d, {}ds, Vector2::angle(a, x) = {}r, {}d, {}ds", 
        Vector2::angle(&Vector2::right(), &a), 
        Vector2::angle(&Vector2::right(), &a) * num::RAD2DEG, 
        Vector2::angle_signed(&Vector2::right(), &a) * num::RAD2DEG, 
        Vector2::angle(&a, &Vector2::right()), 
        Vector2::angle(&a, &Vector2::right()) * num::RAD2DEG,
        Vector2::angle_signed(&a, &Vector2::right()) * num::RAD2DEG);
    println!("Vector2::angle(a, b): {}r, {}d, {}ds, Vector2::angle(b, a) = {}r, {}d, {}ds", 
        Vector2::angle(&a, &b), 
        Vector2::angle(&a, &b) * num::RAD2DEG, 
        Vector2::angle_signed(&a, &b) * num::RAD2DEG, 
        Vector2::angle(&b, &a), 
        Vector2::angle(&b, &a) * num::RAD2DEG,
        Vector2::angle_signed(&b, &a) * num::RAD2DEG);
    println!("Vector2::angle(a, c): {}r, {}d, {}ds, Vector2::angle(c, a) = {}r, {}d, {}ds", 
        Vector2::angle(&a, &c), 
        Vector2::angle(&a, &c) * num::RAD2DEG, 
        Vector2::angle_signed(&a, &c) * num::RAD2DEG, 
        Vector2::angle(&c, &a), 
        Vector2::angle(&c, &a) * num::RAD2DEG, 
        Vector2::angle_signed(&c, &a) * num::RAD2DEG);
    println!("Vector2::angle(a, d): {}r, {}d, {}ds, Vector2::angle(d, a) = {}r, {}d, {}ds", 
        Vector2::angle(&a, &d), 
        Vector2::angle(&a, &d) * num::RAD2DEG, 
        Vector2::angle_signed(&a, &d) * num::RAD2DEG, 
        Vector2::angle(&d, &a), 
        Vector2::angle(&d, &a) * num::RAD2DEG,
        Vector2::angle_signed(&d, &a) * num::RAD2DEG);
    println!("Vector2::angle(a, e): {}r, {}d, {}ds, Vector2::angle(e, a) = {}r, {}d, {}ds", 
        Vector2::angle(&a, &e), 
        Vector2::angle(&a, &e) * num::RAD2DEG, 
        Vector2::angle_signed(&a, &e) * num::RAD2DEG, 
        Vector2::angle(&e, &a), 
        Vector2::angle(&e, &a) * num::RAD2DEG,
        Vector2::angle_signed(&e, &a) * num::RAD2DEG);
    
    println!("Vector2::angle_unit(x, a.normalization()): {}r, {}d, {}ds, Vector2::angle(a.normalization(), x) = {}r, {}d, {}ds", 
        Vector2::angle(&Vector2::right(), &a.normalization()), 
        Vector2::angle(&Vector2::right(), &a.normalization()) * num::RAD2DEG, 
        Vector2::angle_signed(&Vector2::right(), &a.normalization()) * num::RAD2DEG,
        Vector2::angle(&a.normalization(), &Vector2::right()), 
        Vector2::angle(&a.normalization(), &Vector2::right()) * num::RAD2DEG,
        Vector2::angle_signed(&a.normalization(), &Vector2::right()) * num::RAD2DEG);
    println!("Vector2::angle_unit(a.normalization(), b.normalization()): {}r, {}d, {}ds, Vector2::angle_unit(b.normalization(), a.normalization()) = {}r, {}d, {}ds", 
        Vector2::angle_unit(&a.normalization(), &b.normalization()), 
        Vector2::angle_unit(&a.normalization(), &b.normalization()) * num::RAD2DEG, 
        Vector2::angle_signed(&a.normalization(), &b.normalization()) * num::RAD2DEG, 
        Vector2::angle_unit(&b.normalization(), &a.normalization()), 
        Vector2::angle_unit(&b.normalization(), &a.normalization()) * num::RAD2DEG,
        Vector2::angle_signed(&b.normalization(), &a.normalization()) * num::RAD2DEG);
    println!("Vector2::angle_unit(a.normalization(), c.normalization()): {}r, {}d, {}ds, Vector2::angle_unit(c.normalization(), a.normalization()) = {}r, {}d, {}ds", 
        Vector2::angle_unit(&a.normalization(), &c.normalization()), 
        Vector2::angle_unit(&a.normalization(), &c.normalization()) * num::RAD2DEG, 
        Vector2::angle_signed(&a.normalization(), &c.normalization()) * num::RAD2DEG, 
        Vector2::angle_unit(&c.normalization(), &a.normalization()), 
        Vector2::angle_unit(&c.normalization(), &a.normalization()) * num::RAD2DEG,
        Vector2::angle_signed(&c.normalization(), &a.normalization()) * num::RAD2DEG);
    println!("Vector2::angle_unit(a.normalization(), d.normalization()): {}r, {}d, {}ds, Vector2::angle_unit(d.normalization(), a.normalization()) = {}r, {}d, {}ds", 
        Vector2::angle_unit(&a.normalization(), &d.normalization()), 
        Vector2::angle_unit(&a.normalization(), &d.normalization()) * num::RAD2DEG, 
        Vector2::angle_signed(&a.normalization(), &d.normalization()) * num::RAD2DEG, 
        Vector2::angle_unit(&d.normalization(), &a.normalization()), 
        Vector2::angle_unit(&d.normalization(), &a.normalization()) * num::RAD2DEG,
        Vector2::angle_signed(&d.normalization(), &a.normalization()) * num::RAD2DEG);
    println!("Vector2::angle_unit(a.normalization(), e.normalization()): {}r, {}d, {}ds, Vector2::angle_unit(e.normalization(), a.normalization()) = {}r, {}d, {}ds", 
        Vector2::angle_unit(&a.normalization(), &e.normalization()), 
        Vector2::angle_unit(&a.normalization(), &e.normalization()) * num::RAD2DEG, 
        Vector2::angle_signed(&a.normalization(), &e.normalization()) * num::RAD2DEG, 
        Vector2::angle_unit(&e.normalization(), &a.normalization()), 
        Vector2::angle_unit(&e.normalization(), &a.normalization()) * num::RAD2DEG,
        Vector2::angle_signed(&e.normalization(), &a.normalization()) * num::RAD2DEG);

    let A2 = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    let B2 = Matrix2::new(3.0, 2.0, 0.0, 7.0);
    println!("A2 * B2: {}", (A2 * B2).to_string());
}