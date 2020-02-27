// Math library for the game engine, rendering engine, and physics engine
use std::f32::{ self, EPSILON, NAN };

/*
    To do: 
    - Consider whether to keep member-altering functions like normalize(), div_assign(), etc. 
    - Maybe rather make it a conscious decision to do those types of operations instead? 
    - Use Self::new() on swizzling, prefabs, etc. 
    - Vector3 projection, and all the Vector4 geometric operations
    - Check if angle_signed works for both unit and non-unit vectors. Make new functions if needed. 
    - Check if angle_signed is counter-clockwise (like it should be). 
    - Consider making (Vector2) / (Vector2) into a (Vector3), or whether to keep as (f32). 
    - Make MatrixN class? Needs to be on the heap. 
    - Clear up confusion on column-majority, display style, getting elements, and matrix multiplication. 
*/

pub mod num {
    pub mod constants {
        // Function constants
        pub const E             : f32 = 2.718281828459;         // 2.718281828459
        pub const PHI           : f32 = 1.6180339887498;        // 1.6180339887498948482
        pub const SQRT2         : f32 = 1.4142135623731;        // 1.4142135623731
        pub const SQRT2OVER2    : f32 = 0.7071067811865;        // 0.7071067811865
        pub const SQRT3         : f32 = 1.7320508075689;        // 1.7320508075689
        pub const SQRT3OVER2    : f32 = 0.8660254037844;        // 0.8660254037844
        pub const SQRT3OVER3    : f32 = 0.57735026919;
        pub const LN2           : f32 = 0.6931471805599;        // 0.693147180559945309417232121458
        pub const LN3           : f32 = 1.0986122886681;        // 1.09861228866810969139524523692
    
        // Trigonometric constants
        pub const DEG2RAD       : f32 = 0.0174532925199;        // 0.0174532925199
        pub const RAD2DEG       : f32 = 57.2957795130823;       // 57.2957795130823
        pub const TAU           : f32 = 6.2831853071796;        // 6.2831853071796
        pub const TAUOVER2      : f32 = 3.1415926535898;        // 3.1415926535898
        pub const TAUOVER4      : f32 = 1.5707963267949;        // 1.5707963267949
        pub const TAUOVER6      : f32 = 1.0471975511966;        // 1.0471975511966
        pub const TAUOVER8      : f32 = 0.7853981633974;        // 0.7853981633974
        pub const TAUOVER12     : f32 = 0.5235987755983;        // 0.5235987755983
        pub const PI            : f32 = TAUOVER2;
        pub const PIOVER2       : f32 = TAUOVER4;
        pub const PIOVER3       : f32 = TAUOVER6;
        pub const PIOVER4       : f32 = TAUOVER8;
        pub const PIOVER6       : f32 = TAUOVER12;
    
        // Refractive indices
        // Taken from https://en.wikipedia.org/wiki/List_of_refractive_indices
        pub const IOR_vacuum        : f32 = 1.00000;
        pub const IOR_air           : f32 = 1.00029;
        pub const IOR_water         : f32 = 1.333;
        pub const IOR_oliveOil      : f32 = 1.47;
        pub const IOR_silicon       : f32 = 3.45;               // 3.42 - 3.48
        pub const IOR_ice           : f32 = 1.31;
        pub const IOR_glassLight    : f32 = 1.44;
        pub const IOR_glassWindow   : f32 = 1.52;
        pub const IOR_glassFlint    : f32 = 1.77;
        pub const IOR_glassHeavy    : f32 = 1.90;
        pub const IOR_opal          : f32 = 1.445;              // 1.37 - 1.52
        pub const IOR_obsidian      : f32 = 1.5;                // 1.45 - 1.55
        pub const IOR_moonstone     : f32 = 1.522;              // 1.518 - 1.526
        pub const IOR_sunstone      : f32 = 1.5365;             // 1.525 - 1.548
        pub const IOR_amethyst      : f32 = 1.5485;             // 1.544 – 1.553
        pub const IOR_citrine       : f32 = 1.5485;             // 1.544 - 1.553
        pub const IOR_amber         : f32 = 1.55;
        pub const IOR_emerald       : f32 = 1.5835;             // 1.565 - 1.602
        pub const IOR_pearl         : f32 = 1.605;              // 1.52 - 1.69
        pub const IOR_topaz         : f32 = 1.626;              // 1.609 - 1.643
        pub const IOR_turquoise     : f32 = 1.63;               // 1.610 - 1.650
        pub const IOR_jet           : f32 = 1.66;               // 1.640 - 1.680
        pub const IOR_jade          : f32 = 1.67;               // 1.652 - 1.688
        pub const IOR_sapphire      : f32 = 1.77;
        pub const IOR_ruby          : f32 = 1.77;               // 1.762 - 1.778
        pub const IOR_malachite     : f32 = 1.782;              // 1.655 - 1.909
        pub const IOR_azurite       : f32 = 1.784;              // 1.720 - 1.848
        pub const IOR_diamond       : f32 = 2.42;
        pub const IOR_humanLens     : f32 = 1.406;              // 1.386 - 1.406
        pub const IOR_humanCornea   : f32 = 1.38466666667;      // 1.373 / 1.380 / 1.401
    }
}

pub mod linalg {
    pub use super::num::constants::*;
    pub use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, BitXor, Not };
    pub use std::f32;

    // Struct Definitions
    #[derive(Debug, Copy, Clone)] pub struct Vector2    { x: f32, y: f32 }
    //      Complex number form:         c = a + bi
    //      Complex numbers base law:    i^2 = (-1)
    #[derive(Debug, Copy, Clone)] pub struct Complex    { r: f32, i: f32 }
    //      Guidance: http://www.dtecta.com/files/GDC13_vandenBergen_Gino_Math_Tut.pdf
    //      Paper on Automatic Differentiation (AD): https://www.duo.uio.no/bitstream/handle/10852/41535/Kjelseth-Master.pdf?sequence=9 
    //      Introduction to AD: https://alexey.radul.name/ideas/2013/introduction-to-automatic-differentiation/
    //      Dual number form:            d = a + bε
    //      Dual numbers base law:       ε^2 = 0
    #[derive(Debug, Copy, Clone)] pub struct Dual       { r: f32, e: f32 }
    #[derive(Debug, Copy, Clone)] pub struct Vector3    { x: f32, y: f32, z: f32 }
    #[derive(Debug, Copy, Clone)] pub struct Vector4    { x: f32, y: f32, z: f32, w: f32 }
    //      https://www.3dgep.com/understanding-quaternions/
    //      Quaternion number form:      q = s + xi + yj + zk -> { s, x, y, z } ∈ R
    //      Quaternion base law:         i^2 + j^2 + k^2 = ijk = (-1)
    /*
            ij = k      jk = i      ki = j
            ji = -k     kj = -i     ik = -j
    */
    #[derive(Debug, Copy, Clone)] pub struct Quaternion { s: f32, i: f32, j: f32, k: f32 }
    // #[derive(Debug, Copy, Clone)] pub struct QuaternionDual { /* ? */ }

    /*
        Matrix conventions: 
        - Column-major, store as 1 row of column vectors
        - Transformations written like this: M * v
        - Data definition: [[type; col]; row]
    */
    #[derive(Debug, Copy, Clone)] pub struct Matrix2    { e: [[f32; 2]; 2] }
    #[derive(Debug, Copy, Clone)] pub struct Matrix3    { e: [[f32; 3]; 3] }
    #[derive(Debug, Copy, Clone)] pub struct Matrix4    { e: [[f32; 4]; 4] }

    // Field Interpretation
    // (do vector [] syntax)
    impl Matrix2 { pub fn row(&self, n: usize)          -> Vector2 { return Vector2::new(self.e[n][0], self.e[n][1]); } }
    impl Matrix3 { pub fn row(&self, n: usize)          -> Vector3 { return Vector3::new(self.e[n][0], self.e[n][1], self.e[n][2]); } }
    impl Matrix4 { pub fn row(&self, n: usize)          -> Vector4 { return Vector4::new(self.e[n][0], self.e[n][1], self.e[n][2], self.e[n][3]); } }
    impl Matrix2 { pub fn column(&self, n: usize)       -> Vector2 { return Vector2::new(self.e[0][n], self.e[1][n]); } }
    impl Matrix3 { pub fn column(&self, n: usize)       -> Vector3 { return Vector3::new(self.e[0][n], self.e[1][n], self.e[2][n]); } }
    impl Matrix4 { pub fn column(&self, n: usize)       -> Vector4 { return Vector4::new(self.e[0][n], self.e[1][n], self.e[2][n], self.e[3][n]); } }
    impl Matrix2 { pub fn diagonal(&self)               -> Vector2 { return Vector2::new(self.e[0][0], self.e[1][1]); } }
    impl Matrix3 { pub fn diagonal(&self)               -> Vector3 { return Vector3::new(self.e[0][0], self.e[1][1], self.e[2][2]); } }
    impl Matrix4 { pub fn diagonal(&self)               -> Vector4 { return Vector4::new(self.e[0][0], self.e[1][1], self.e[2][2], self.e[3][3]); } }
    
    // Construction
    impl Vector2    { pub fn new(a: f32, b: f32)                        -> Self { return Self { x: a, y: b }; } }
    impl Complex    { pub fn new(a: f32, b: f32)                        -> Self { return Self { r: a, i: b}; } }
    impl Dual       { pub fn new(a: f32, b: f32)                        -> Self { return Self { r: a, e: b}; } }
    impl Vector3    { pub fn new(a: f32, b: f32, c: f32)                -> Self { return Self { x: a, y: b, z: c }; } }
    impl Vector4    { pub fn new(a: f32, b: f32, c: f32, d: f32)        -> Self { return Self { x: a, y: b, z: c, w: d }; } }
    impl Quaternion { pub fn new(a: f32, b: f32, c: f32, d: f32)        -> Self { return Self { s: a, i: b, j: c, k: d }; } }
    impl Vector2    { pub fn from_polar(angle: f32, radius: f32)        -> Self { return Self::new(f32::cos(angle), f32::sin(angle)) * radius; } }
    impl Complex    { pub fn from_polar(angle: f32, radius: f32)        -> Self { return Self::new(f32::cos(angle), f32::sin(angle)) * radius; } }
    impl Dual       { pub fn from_polar(angle: f32, radius: f32)        -> Self { return Self::new(f32::cos(angle), f32::sin(angle)) * radius; } }
    impl Vector3    { pub fn from_spherical(r: f32, t: f32, p: f32)     -> Self { return Self::new(f32::sin(p) * f32::cos(t), f32::sin(p) * f32::sin(t), f32::cos(p)) * r; } }
    impl Matrix2    { pub fn new(   a: f32, b: f32, 
                                    c: f32, d: f32) -> Self {
        return Self { e:           [[a, c],
                                    [b, d]] } } }
    impl Matrix2    { pub fn from_vector2(a: Vector2, b: Vector2) -> Self {
        return Self::new(a.x, a.y, b.x, b.y); } }
    impl Matrix3    { pub fn new(   a: f32, b: f32, c: f32, 
                                    d: f32, e: f32, f: f32,
                                    g: f32, h: f32, i: f32) -> Self {
        return Self { e:           [[a, d, g],
                                    [b, e, h],
                                    [c, f, i]] } } }
    impl Matrix3    { pub fn from_vector3(a: Vector3, b: Vector3, c: Vector3) -> Self {
        return Self::new(a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z); } }
    impl Matrix4    { pub fn new(   a: f32, b: f32, c: f32, d: f32,
                                    e: f32, f: f32, g: f32, h: f32,
                                    i: f32, j: f32, k: f32, l: f32,
                                    m: f32, n: f32, o: f32, p: f32) -> Self {
        return Self { e:           [[a, e, i, m],
                                    [b, f, j, n],
                                    [c, g, k, o],
                                    [d, h, l, p]] } } }
    impl Matrix4    { pub fn from_vector4(a: Vector4, b: Vector4, c: Vector4, d: Vector4) -> Self {
        return Self::new(a.x, a.y, a.z, a.w, b.x, b.y, b.z, b.w, c.x, c.y, c.z, c.w, d.x, d.y, d.z, d.w); } }

    // Read functions
    impl Vector2    { pub fn as_ptr(&self)  -> *const f32 { return &self.x; } }
    impl Vector3    { pub fn as_ptr(&self)  -> *const f32 { return &self.x; } }
    impl Vector4    { pub fn as_ptr(&self)  -> *const f32 { return &self.x; } }
    impl Matrix2    { pub fn as_ptr(&self)  -> *const f32 { return &self.e[0][0]; } }
    impl Matrix3    { pub fn as_ptr(&self)  -> *const f32 { return &self.e[0][0]; } }
    impl Matrix4    { pub fn as_ptr(&self)  -> *const f32 { return &self.e[0][0]; } }
    
    // impl Quaternion {
    //     // Unfinished
    // }
    // impl QuaternionDual {
    //     // Unfinished
    // }
    
    //      Transformation Constructors
    //          Translation
    impl Matrix4    { pub fn translation(v: Vector3) -> Self {
        return Matrix4::new(
            1.0, 0.0, 0.0, v.x,
            0.0, 1.0, 0.0, v.y,
            0.0, 0.0, 1.0, v.z,
            0.0, 0.0, 0.0, 1.0); } }
    //          Rotation
    impl Complex    { pub fn from_rotor(angle: f32)                     -> Self { return Self::new(f32::cos(angle), f32::sin(angle)); } }
    impl Matrix4    { pub fn rotation_x(t: f32) -> Self { 
        let cos = f32::cos(t); 
        let sin = f32::sin(t);
        return Self::new(
            1.0,    0.0,    0.0,    0.0,
            0.0,    cos,    -sin,   0.0,
            0.0,    sin,    cos,    0.0,
            0.0,    0.0,    0.0,    1.0); } }
    impl Matrix4    { pub fn rotation_y(t: f32) -> Self { 
        let cos = f32::cos(t); 
        let sin = f32::sin(t);
        return Self::new(
            cos,    0.0,    sin,    0.0,
            0.0,    1.0,    0.0,    0.0,
            -sin,   0.0,    cos,    0.0,
            0.0,    0.0,    0.0,    1.0); } }
    impl Matrix4    { pub fn rotation_z(t: f32) -> Self { 
        let cos = f32::cos(t); 
        let sin = f32::sin(t);
        return Self::new(
            cos,    -sin,   0.0,    0.0,
            sin,    cos,    0.0,    0.0,
            0.0,    0.0,    0.0,    0.0,
            0.0,    0.0,    0.0,    1.0); } }
    impl Matrix4    { pub fn rotation(t: f32, v: Vector3) -> Self {
        // Add cosine-sine double calculation here at a later time
        let cos = f32::cos(t);
        let sin = f32::sin(t);
        let d = 1.0 - cos;

        let x = v.x * d;
        let y = v.y * d;
        let z = v.z * d;
        let vxvy = x * v.y;
        let vxvz = x * v.z;
        let vyvz = y * v.z;

        return Self::new(
            cos + x * v.x,      vxvy - sin * v.z,   vxvz + sin * v.y,   0.0,
            vxvy + sin * v.z,   cos + y * v.y,      vyvz - sin * v.x,   0.0,
            vxvz - sin * v.y,   vyvz + sin * v.x,   cos + z * v.z,      0.0,
            0.0,                0.0,                0.0,                1.0); } }

    //          Scale
    impl Matrix4    { pub fn scale_uniform(t: f32) -> Self {
        return Self::new(
            t, 0.0, 0.0, 0.0,
            0.0, t, 0.0, 0.0,
            0.0, 0.0, t, 0.0,
            0.0, 0.0, 0.0, 1.0); } }
    impl Matrix4    { pub fn scale_vector(v: Vector3) -> Self {
        return Self::new(
            v.x, 0.0, 0.0, 0.0,
            0.0, v.y, 0.0, 0.0,
            0.0, 0.0, v.z, 0.0,
            0.0, 0.0, 0.0, 1.0); } }
    //          Other
    impl Matrix4    { pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let scl_y = 1.0 / f32::tan(DEG2RAD * fov / 2.0);
        let scl_x = scl_y / aspect_ratio;
        let near_m_far = near - far;

        return Self::new(
            scl_x,  0.0,    0.0,                            0.0,
            0.0,    scl_y,  0.0,                            0.0,
            0.0,    0.0,    (near + far) / near_m_far,      2.0 * near * far / near_m_far,
            0.0,    0.0,    -1.0,                           0.0); } }
    
    // Conversion Methods
    impl Vector2    { pub fn from_vector2(v: &Vector2)                  -> Self { return Self::new(v.x, v.y); } }
    impl Complex    { pub fn from_vector2(v: &Vector2)                  -> Self { return Self::new(v.x, v.y); } }
    impl Dual       { pub fn from_vector2(v: &Vector2)                  -> Self { return Self::new(v.x, v.y); } }
    impl Vector3    { pub fn from_vector2(v: &Vector2, c: f32)          -> Self { return Self::new(v.x, v.y, c); } }
    impl Vector4    { pub fn from_vector2(v: &Vector2, c: f32, d: f32)  -> Self { return Self::new(v.x, v.y, c, d); } }
    impl Vector2    { pub fn from_complex(c: &Complex)                  -> Self { return Self::new(c.r, c.i); } }
    impl Complex    { pub fn from_complex(c: &Complex)                  -> Self { return Self::new(c.r, c.i); } }
    impl Dual       { pub fn from_complex(c: &Complex)                  -> Self { return Self::new(c.r, c.i); } }
    impl Vector3    { pub fn from_complex(c: &Complex, c2: f32)         -> Self { return Self::new(c.r, c.i, c2); } }
    impl Vector4    { pub fn from_complex(c: &Complex, c2: f32, d: f32) -> Self { return Self::new(c.r, c.i, c2, d); } }
    impl Vector2    { pub fn from_dual(c: &Dual)                        -> Self { return Self::new(c.r, c.e); } }
    impl Complex    { pub fn from_dual(d: &Dual)                        -> Self { return Self::new(d.r, d.e); } }
    impl Dual       { pub fn from_dual(d: &Dual)                        -> Self { return Self::new(d.r, d.e); } }
    impl Vector3    { pub fn from_dual(d: &Dual, c: f32)                -> Self { return Self::new(d.r, d.e, c); } }
    impl Vector4    { pub fn from_dual(d: &Dual, c: f32, d2: f32)       -> Self { return Self::new(d.r, d.e, c, d2); } }
    impl Vector2    { pub fn from_vector3(v: &Vector3)                  -> Self { return Self::new(v.x, v.y); } }
    impl Complex    { pub fn from_vector3(v: &Vector3)                  -> Self { return Self::new(v.x, v.y); } }
    impl Dual       { pub fn from_vector3(v: &Vector3)                  -> Self { return Self::new(v.x, v.y); } }
    impl Vector3    { pub fn from_vector3(v: &Vector3)                  -> Self { return Self::new(v.x, v.y, v.z); } }
    impl Vector4    { pub fn from_vector3(v: &Vector3, d: f32)          -> Self { return Self::new(v.x, v.y, v.z, d); } }
    impl Vector2    { pub fn from_vector4(v: &Vector4)                  -> Self { return Self::new(v.x, v.y); } }
    impl Complex    { pub fn from_vector4(v: &Vector4)                  -> Self { return Self::new(v.x, v.y); } }
    impl Dual       { pub fn from_vector4(v: &Vector4)                  -> Self { return Self::new(v.x, v.y); } }
    impl Vector3    { pub fn from_vector4(v: &Vector4)                  -> Self { return Self::new(v.x, v.y, v.z); } }
    impl Vector4    { pub fn from_vector4(v: &Vector4)                  -> Self { return Self::new(v.x, v.y, v.z, v.w); } }
    //      https://stackoverflow.com/questions/36138768/finding-minor-matrices-of-3x3-matrix-c
    
    // Transpose (also implemented for unary operator [-])
    impl Matrix2    { pub fn transpose(&self)                           -> Self {
        return Self::new(
            self.e[0][0],   self.e[0][1],
            self.e[1][0],   self.e[1][1]); } }
    impl Matrix3    { pub fn transpose(&self)                           -> Self {
        return Self::new(
            self.e[0][0],   self.e[0][1],   self.e[0][2],
            self.e[1][0],   self.e[1][1],   self.e[1][2],
            self.e[2][0],   self.e[2][1],   self.e[2][2]); } }
    impl Matrix4    { pub fn transpose(&self)                           -> Self {
        return Self::new(
            self.e[0][0],   self.e[0][1],   self.e[0][2],   self.e[0][3],
            self.e[1][0],   self.e[1][1],   self.e[1][2],   self.e[1][3],
            self.e[2][0],   self.e[2][1],   self.e[2][2],   self.e[2][3],
            self.e[3][0],   self.e[3][1],   self.e[3][2],   self.e[3][3]); } }
    // Matrix minor
    impl Matrix2    { pub fn minor(&self, i: usize, j: usize)           -> f32 { return self.e[1 - i][1 - j]; } }
    impl Matrix3    { pub fn minor(&self, i: usize, j: usize)           -> Matrix2 {
        let mut M = Matrix2::zero();
        let mut row = 0;
        let mut col = 0;

        for a in 0..3 {
            row = a;
            if i < a {
                row = row - 1;
            } 
            for b in 0..3 {
                col = b;
                if j < b {
                    col = col - 1;
                }
                if a != i && b != j {
                    M.e[row][col] = self.e[a][b];
                }
            }
        }
        return M; } }
    impl Matrix4    { pub fn minor(&self, i: usize, j: usize)           -> Matrix3 {
        let mut M = Matrix3::zero();
        let mut row = 0;
        let mut col = 0;

        for a in 0..4 {
            row = a;
            if i < a {
                row = row - 1;
            } 
            for b in 0..4 {
                col = b;
                if j < b {
                    col = col - 1;
                }
                if a != i && b != j {
                    M.e[row][col] = self.e[a][b];
                }
            }
        }
        return M; } }

    
    // Cofactors
    impl Matrix2    { pub fn cofactor(&self, i: usize, j: usize)        -> f32 {
        return self.minor(i, j) * f32::signum((((i % 2) as f32) - 0.5) * (((j % 2) as f32) - 0.5)); } }
    impl Matrix3    { pub fn cofactor(&self, i: usize, j: usize)        -> f32 {
        return self.minor(i, j).determinant() * f32::signum((((i % 2) as f32) - 0.5) * (((j % 2) as f32) - 0.5)); } }
    impl Matrix4    { pub fn cofactor(&self, i: usize, j: usize)        -> f32 {
        return self.minor(i, j).determinant() * f32::signum((((i % 2) as f32) - 0.5) * (((j % 2) as f32) - 0.5)); } }

    // Cofactor matrix
    impl Matrix2    { pub fn cofactor_matrix(&self)                     -> Self {
        let mut c = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                c.push(self.cofactor(j, i));
            }
        }
        return Matrix2::new(
            c[0],   c[1],
            c[2],   c[3]); } }
    impl Matrix3    { pub fn cofactor_matrix(&self)                     -> Self {
        let mut c = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                c.push(self.cofactor(j, i));
            }
        }
        return Matrix3::new(
            c[0],   c[1],   c[2],
            c[3],   c[4],   c[5],
            c[6],   c[7],   c[8]); } }
    impl Matrix4    { pub fn cofactor_matrix(&self)                     -> Self {
        let mut c = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                c.push(self.cofactor(j, i));
            }
        }
        return Matrix4::new(
            c[0],   c[1],   c[2],   c[3],
            c[4],   c[5],   c[6],   c[7],
            c[8],   c[9],   c[10],  c[11],
            c[12],  c[13],  c[14],  c[15]); } }

    // Matrix adjugate
    impl Matrix2    { pub fn adjugate(&self)                            -> Self {
        return self.cofactor_matrix().transpose(); } }
    impl Matrix3    { pub fn adjugate(&self)                            -> Self {
        return self.cofactor_matrix().transpose(); } }
    impl Matrix4    { pub fn adjugate(&self)                            -> Self {
        return self.cofactor_matrix().transpose(); } }
    // Inverse (also implemented for unary operator [!])
    impl Matrix2    { pub fn inverse(&self)                             -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }
    impl Matrix3    { pub fn inverse(&self)                             -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }
    impl Matrix4    { pub fn inverse(&self)                             -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }

    // Matrix triangulation
    impl Matrix2    { pub fn triangular_lower(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);

        return Matrix2::from_vector2(col0, col1 - (col1.x / col0.x) * col0); } }
    impl Matrix3    { pub fn triangular_lower(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);
        let col2 = self.column(2);
        let col1_a = col1 - col0 * (col1.x / col0.x);
        let col2_a = col2 - col0 * (col2.x / col0.x);
        let col2_b = col2_a - col1_a * (col2_a.y / col1_a.y);

        return Matrix3::from_vector3(col0, col1_a, col2_b); } }
    impl Matrix4    { pub fn triangular_lower(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);
        let col2 = self.column(2);
        let col3 = self.column(3);
        let col1_a = col1 - col0 * (col1.x / col0.x);
        let col2_a = col2 - col0 * (col2.x / col0.x);
        let col3_a = col3 - col0 * (col3.x / col0.x);
        let col2_b = col2_a - col1_a * (col2_a.y / col1_a.y);
        let col3_b = col3_a - col1_a * (col3_a.y / col1_a.y);
        let col3_c = col3_b - col2_b * (col3_b.z / col2_b.z);

        return Matrix4::from_vector4(col0, col1_a, col2_b, col3_c); } }
    impl Matrix2    { pub fn triangular_upper(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);

        return Matrix2::from_vector2(col0 - col1 * (col0.y / col1.y), col1); } }
    impl Matrix3    { pub fn triangular_upper(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);
        let col2 = self.column(2);
        let col0_a = col0 - col2 * (col0.z / col2.z);
        let col1_a = col1 - col2 * (col1.z / col2.z);
        let col0_b = col0_a - col1_a * (col0_a.y / col1_a.y);

        return Matrix3::from_vector3(col0_b, col1_a, col2); } }
    impl Matrix4    { pub fn triangular_upper(&self)                    -> Self {
        let col0 = self.column(0);
        let col1 = self.column(1);
        let col2 = self.column(2);
        let col3 = self.column(3);
        let col0_a = col0 - col3 * (col0.w / col3.w); 
        let col1_a = col1 - col3 * (col1.w / col3.w); 
        let col2_a = col2 - col3 * (col2.w / col3.w); 
        let col0_b = col0_a - col2_a * (col0_a.z / col2_a.z);
        let col1_b = col1_a - col2_a * (col1_a.z / col2_a.z);
        let col0_c = col0_b - col1_b * (col0_b.y / col1_b.y);

        return Matrix4::from_vector4(col0_c, col1_b, col2_a, col3); } }

    // Prefabrication
    impl Vector2 { pub fn one()         -> Self { return Self::new(1.0, 1.0); } }
    impl Vector3 { pub fn one()         -> Self { return Self::new(1.0, 1.0, 1.0); } }
    impl Vector4 { pub fn one()         -> Self { return Self::new(1.0, 1.0, 1.0, 1.0); } }
    impl Matrix2 { pub fn one()         -> Self { return Self::new(1.0, 1.0, 1.0, 1.0); } }
    impl Matrix3 { pub fn one()         -> Self { return Self::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0); } }
    impl Matrix4 { pub fn one()         -> Self { return Self::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0); } }
    impl Vector2 { pub fn zero()        -> Self { return Self::new(0.0, 0.0); } }
    impl Vector3 { pub fn zero()        -> Self { return Self::new(0.0, 0.0, 0.0); } }
    impl Vector4 { pub fn zero()        -> Self { return Self::new(0.0, 0.0, 0.0, 0.0); } }
    impl Matrix2 { pub fn zero()        -> Self { return Self::new(0.0, 0.0, 0.0, 0.0); } }
    impl Matrix3 { pub fn zero()        -> Self { return Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0); } }
    impl Matrix4 { pub fn zero()        -> Self { return Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0); } }
    impl Matrix2 { pub fn identity()    -> Self { return Self::new(1.0, 0.0, 0.0, 1.0); } }
    impl Matrix3 { pub fn identity()    -> Self { return Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0); } }
    impl Matrix4 { pub fn identity()    -> Self { return Self::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0); } }

    impl Vector2 { pub fn right()       -> Self { return Self::new(1.0, 0.0); } }
    impl Vector3 { pub fn right()       -> Self { return Self::new(1.0, 0.0, 0.0); } }
    impl Vector2 { pub fn left()        -> Self { return Self::new(-1.0, 0.0); } }
    impl Vector3 { pub fn left()        -> Self { return Self::new(-1.0, 0.0, 0.0); } }
    impl Vector2 { pub fn forth()       -> Self { return Self::new(0.0, 1.0); } }
    impl Vector3 { pub fn forth()       -> Self { return Self::new(0.0, 1.0, 0.0); } }
    impl Vector2 { pub fn back()        -> Self { return Self::new(0.0, -1.0); } }
    impl Vector3 { pub fn back()        -> Self { return Self::new(0.0, -1.0, 0.0); } }
    impl Vector3 { pub fn up()          -> Self { return Self::new(0.0, 0.0, 1.0); } }
    impl Vector3 { pub fn down()        -> Self { return Self::new(0.0, 0.0, -1.0); } }

    impl Vector2 { pub fn Q1()          -> Self { return Self::new(1.0, 1.0); } }
    impl Vector2 { pub fn Q1n()         -> Self { return Self::new(SQRT2OVER2, SQRT2OVER2); } }
    impl Vector3 { pub fn Q1()          -> Self { return Self::new(1.0, 1.0, 1.0); } }
    impl Vector3 { pub fn Q1n()         -> Self { return Self::new(SQRT3OVER3, SQRT3OVER3, SQRT3OVER3); } }
    impl Vector2 { pub fn Q2()          -> Self { return Self::new(-1.0, 1.0); } }
    impl Vector2 { pub fn Q2n()         -> Self { return Self::new(-SQRT2OVER2, SQRT2OVER2); } }
    impl Vector3 { pub fn Q2()          -> Self { return Self::new(-1.0, 1.0, 1.0); } }
    impl Vector3 { pub fn Q2n()         -> Self { return Self::new(-SQRT3OVER3, SQRT3OVER3, SQRT3OVER3); } }
    impl Vector2 { pub fn Q3()          -> Self { return Self::new(-1.0, -1.0); } }
    impl Vector2 { pub fn Q3n()         -> Self { return Self::new(-SQRT2OVER2, -SQRT2OVER2); } }
    impl Vector3 { pub fn Q3()          -> Self { return Self::new(-1.0, -1.0, 1.0); } }
    impl Vector3 { pub fn Q3n()         -> Self { return Self::new(-SQRT3OVER3, -SQRT3OVER3, SQRT3OVER3); } }
    impl Vector2 { pub fn Q4()          -> Self { return Self::new(1.0, 1.0); } }
    impl Vector2 { pub fn Q4n()         -> Self { return Self::new(SQRT2OVER2, SQRT2OVER2); } }
    impl Vector3 { pub fn Q4()          -> Self { return Self::new(1.0, 1.0, 1.0); } }
    impl Vector3 { pub fn Q4n()         -> Self { return Self::new(SQRT3OVER3, SQRT3OVER3, SQRT3OVER3); } }
    impl Vector3 { pub fn Q5()          -> Self { return Self::new(1.0, 1.0, -1.0); } }
    impl Vector3 { pub fn Q5n()         -> Self { return Self::new(SQRT3OVER3, SQRT3OVER3, -SQRT3OVER3); } }
    impl Vector3 { pub fn Q6()          -> Self { return Self::new(-1.0, 1.0, -1.0); } }
    impl Vector3 { pub fn Q6n()         -> Self { return Self::new(-SQRT3OVER3, SQRT3OVER3, -SQRT3OVER3); } }
    impl Vector3 { pub fn Q7()          -> Self { return Self::new(-1.0, -1.0, -1.0); } }
    impl Vector3 { pub fn Q7n()         -> Self { return Self::new(-SQRT3OVER3, -SQRT3OVER3, -SQRT3OVER3); } }
    impl Vector3 { pub fn Q8()          -> Self { return Self::new(1.0, -1.0, -1.0); } }
    impl Vector3 { pub fn Q8n()         -> Self { return Self::new(SQRT3OVER3, -SQRT3OVER3, -SQRT3OVER3); } }
    
    // impl Complex { /* Unfinished */ }
    // impl Dual { /* Unfinished */ }
    // impl Quaternion { /* Unfinished */ }
    // impl QuaternionDual { /* Unfinished */ }
    
    // Swizzling
    impl Vector2 {
        pub fn xx(&self)    -> Self { return Vector2::new(self.x, self.x); }
        pub fn xy(&self)    -> Self { return Vector2::new(self.x, self.y); }
        pub fn yx(&self)    -> Self { return Vector2::new(self.y, self.x); }
        pub fn yy(&self)    -> Self { return Vector2::new(self.y, self.y); }
    }
    impl Vector3 {
        pub fn xxx(&self)   -> Self { return Vector3::new(self.x, self.x, self.x); }
        pub fn xxy(&self)   -> Self { return Vector3::new(self.x, self.x, self.y); }
        pub fn xxz(&self)   -> Self { return Vector3::new(self.x, self.x, self.z); }

        pub fn xyx(&self)   -> Self { return Vector3::new(self.x, self.y, self.x); }
        pub fn xyy(&self)   -> Self { return Vector3::new(self.x, self.y, self.y); }
        pub fn xyz(&self)   -> Self { return Vector3::new(self.x, self.y, self.z); }

        pub fn xzx(&self)   -> Self { return Vector3::new(self.x, self.z, self.x); }
        pub fn xzy(&self)   -> Self { return Vector3::new(self.x, self.z, self.y); }
        pub fn xzz(&self)   -> Self { return Vector3::new(self.x, self.z, self.z); }


        pub fn yxx(&self)   -> Self { return Vector3::new(self.y, self.x, self.x); }
        pub fn yxy(&self)   -> Self { return Vector3::new(self.y, self.x, self.y); }
        pub fn yxz(&self)   -> Self { return Vector3::new(self.y, self.x, self.z); }

        pub fn yyx(&self)   -> Self { return Vector3::new(self.y, self.y, self.x); }
        pub fn yyy(&self)   -> Self { return Vector3::new(self.y, self.y, self.y); }
        pub fn yyz(&self)   -> Self { return Vector3::new(self.y, self.y, self.z); }

        pub fn yzx(&self)   -> Self { return Vector3::new(self.y, self.z, self.x); }
        pub fn yzy(&self)   -> Self { return Vector3::new(self.y, self.z, self.y); }
        pub fn yzz(&self)   -> Self { return Vector3::new(self.y, self.z, self.z); }


        pub fn zxx(&self)   -> Self { return Vector3::new(self.z, self.x, self.x); }
        pub fn zxy(&self)   -> Self { return Vector3::new(self.z, self.x, self.y); }
        pub fn zxz(&self)   -> Self { return Vector3::new(self.z, self.x, self.z); }

        pub fn zyx(&self)   -> Self { return Vector3::new(self.z, self.y, self.x); }
        pub fn zyy(&self)   -> Self { return Vector3::new(self.z, self.y, self.y); }
        pub fn zyz(&self)   -> Self { return Vector3::new(self.z, self.y, self.z); }

        pub fn zzx(&self)   -> Self { return Vector3::new(self.z, self.z, self.x); }
        pub fn zzy(&self)   -> Self { return Vector3::new(self.z, self.z, self.y); }
        pub fn zzz(&self)   -> Self { return Vector3::new(self.z, self.z, self.z); }
    }
    impl Vector4 {
        pub fn xxxx(&self)  -> Self { return Vector4::new(self.x, self.x, self.x, self.x); }
        pub fn xxxy(&self)  -> Self { return Vector4::new(self.x, self.x, self.x, self.y); }
        pub fn xxxz(&self)  -> Self { return Vector4::new(self.x, self.x, self.x, self.z); }
        pub fn xxxw(&self)  -> Self { return Vector4::new(self.x, self.x, self.x, self.w); }

        pub fn xxyx(&self)  -> Self { return Vector4::new(self.x, self.x, self.y, self.x); }
        pub fn xxyy(&self)  -> Self { return Vector4::new(self.x, self.x, self.y, self.y); }
        pub fn xxyz(&self)  -> Self { return Vector4::new(self.x, self.x, self.y, self.z); }
        pub fn xxyw(&self)  -> Self { return Vector4::new(self.x, self.x, self.y, self.w); }

        pub fn xxzx(&self)  -> Self { return Vector4::new(self.x, self.x, self.z, self.x); }
        pub fn xxzy(&self)  -> Self { return Vector4::new(self.x, self.x, self.z, self.y); }
        pub fn xxzz(&self)  -> Self { return Vector4::new(self.x, self.x, self.z, self.z); }
        pub fn xxzw(&self)  -> Self { return Vector4::new(self.x, self.x, self.z, self.w); }

        pub fn xxwx(&self)  -> Self { return Vector4::new(self.x, self.x, self.w, self.x); }
        pub fn xxwy(&self)  -> Self { return Vector4::new(self.x, self.x, self.w, self.y); }
        pub fn xxwz(&self)  -> Self { return Vector4::new(self.x, self.x, self.w, self.z); }
        pub fn xxww(&self)  -> Self { return Vector4::new(self.x, self.x, self.w, self.w); }


        pub fn xyxx(&self)  -> Self { return Vector4::new(self.x, self.y, self.x, self.x); }
        pub fn xyxy(&self)  -> Self { return Vector4::new(self.x, self.y, self.x, self.y); }
        pub fn xyxz(&self)  -> Self { return Vector4::new(self.x, self.y, self.x, self.z); }
        pub fn xyxw(&self)  -> Self { return Vector4::new(self.x, self.y, self.x, self.w); }

        pub fn xyyx(&self)  -> Self { return Vector4::new(self.x, self.y, self.y, self.x); }
        pub fn xyyy(&self)  -> Self { return Vector4::new(self.x, self.y, self.y, self.y); }
        pub fn xyyz(&self)  -> Self { return Vector4::new(self.x, self.y, self.y, self.z); }
        pub fn xyyw(&self)  -> Self { return Vector4::new(self.x, self.y, self.y, self.w); }

        pub fn xyzx(&self)  -> Self { return Vector4::new(self.x, self.y, self.z, self.x); }
        pub fn xyzy(&self)  -> Self { return Vector4::new(self.x, self.y, self.z, self.y); }
        pub fn xyzz(&self)  -> Self { return Vector4::new(self.x, self.y, self.z, self.z); }
        pub fn xyzw(&self)  -> Self { return Vector4::new(self.x, self.y, self.z, self.w); }

        pub fn xywx(&self)  -> Self { return Vector4::new(self.x, self.y, self.w, self.x); }
        pub fn xywy(&self)  -> Self { return Vector4::new(self.x, self.y, self.w, self.y); }
        pub fn xywz(&self)  -> Self { return Vector4::new(self.x, self.y, self.w, self.z); }
        pub fn xyww(&self)  -> Self { return Vector4::new(self.x, self.y, self.w, self.w); }

        
        pub fn xzxx(&self)  -> Self { return Vector4::new(self.x, self.z, self.x, self.x); }
        pub fn xzxy(&self)  -> Self { return Vector4::new(self.x, self.z, self.x, self.y); }
        pub fn xzxz(&self)  -> Self { return Vector4::new(self.x, self.z, self.x, self.z); }
        pub fn xzxw(&self)  -> Self { return Vector4::new(self.x, self.z, self.x, self.w); }

        pub fn xzyx(&self)  -> Self { return Vector4::new(self.x, self.z, self.y, self.x); }
        pub fn xzyy(&self)  -> Self { return Vector4::new(self.x, self.z, self.y, self.y); }
        pub fn xzyz(&self)  -> Self { return Vector4::new(self.x, self.z, self.y, self.z); }
        pub fn xzyw(&self)  -> Self { return Vector4::new(self.x, self.z, self.y, self.w); }

        pub fn xzzx(&self)  -> Self { return Vector4::new(self.x, self.z, self.z, self.x); }
        pub fn xzzy(&self)  -> Self { return Vector4::new(self.x, self.z, self.z, self.y); }
        pub fn xzzz(&self)  -> Self { return Vector4::new(self.x, self.z, self.z, self.z); }
        pub fn xzzw(&self)  -> Self { return Vector4::new(self.x, self.z, self.z, self.w); }

        pub fn xzwx(&self)  -> Self { return Vector4::new(self.x, self.z, self.w, self.x); }
        pub fn xzwy(&self)  -> Self { return Vector4::new(self.x, self.z, self.w, self.y); }
        pub fn xzwz(&self)  -> Self { return Vector4::new(self.x, self.z, self.w, self.z); }
        pub fn xzww(&self)  -> Self { return Vector4::new(self.x, self.z, self.w, self.w); }


        pub fn xwxx(&self)  -> Self { return Vector4::new(self.x, self.w, self.x, self.x); }
        pub fn xwxy(&self)  -> Self { return Vector4::new(self.x, self.w, self.x, self.y); }
        pub fn xwxz(&self)  -> Self { return Vector4::new(self.x, self.w, self.x, self.z); }
        pub fn xwxw(&self)  -> Self { return Vector4::new(self.x, self.w, self.x, self.w); }

        pub fn xwyx(&self)  -> Self { return Vector4::new(self.x, self.w, self.y, self.x); }
        pub fn xwyy(&self)  -> Self { return Vector4::new(self.x, self.w, self.y, self.y); }
        pub fn xwyz(&self)  -> Self { return Vector4::new(self.x, self.w, self.y, self.z); }
        pub fn xwyw(&self)  -> Self { return Vector4::new(self.x, self.w, self.y, self.w); }

        pub fn xwzx(&self)  -> Self { return Vector4::new(self.x, self.w, self.z, self.x); }
        pub fn xwzy(&self)  -> Self { return Vector4::new(self.x, self.w, self.z, self.y); }
        pub fn xwzz(&self)  -> Self { return Vector4::new(self.x, self.w, self.z, self.z); }
        pub fn xwzw(&self)  -> Self { return Vector4::new(self.x, self.w, self.z, self.w); }

        pub fn xwwx(&self)  -> Self { return Vector4::new(self.x, self.w, self.w, self.x); }
        pub fn xwwy(&self)  -> Self { return Vector4::new(self.x, self.w, self.w, self.y); }
        pub fn xwwz(&self)  -> Self { return Vector4::new(self.x, self.w, self.w, self.z); }
        pub fn xwww(&self)  -> Self { return Vector4::new(self.x, self.w, self.w, self.w); }



        pub fn yxxx(&self)  -> Self { return Vector4::new(self.y, self.x, self.x, self.x); }
        pub fn yxxy(&self)  -> Self { return Vector4::new(self.y, self.x, self.x, self.y); }
        pub fn yxxz(&self)  -> Self { return Vector4::new(self.y, self.x, self.x, self.z); }
        pub fn yxxw(&self)  -> Self { return Vector4::new(self.y, self.x, self.x, self.w); }

        pub fn yxyx(&self)  -> Self { return Vector4::new(self.y, self.x, self.y, self.x); }
        pub fn yxyy(&self)  -> Self { return Vector4::new(self.y, self.x, self.y, self.y); }
        pub fn yxyz(&self)  -> Self { return Vector4::new(self.y, self.x, self.y, self.z); }
        pub fn yxyw(&self)  -> Self { return Vector4::new(self.y, self.x, self.y, self.w); }

        pub fn yxzx(&self)  -> Self { return Vector4::new(self.y, self.x, self.z, self.x); }
        pub fn yxzy(&self)  -> Self { return Vector4::new(self.y, self.x, self.z, self.y); }
        pub fn yxzz(&self)  -> Self { return Vector4::new(self.y, self.x, self.z, self.z); }
        pub fn yxzw(&self)  -> Self { return Vector4::new(self.y, self.x, self.z, self.w); }

        pub fn yxwx(&self)  -> Self { return Vector4::new(self.y, self.x, self.w, self.x); }
        pub fn yxwy(&self)  -> Self { return Vector4::new(self.y, self.x, self.w, self.y); }
        pub fn yxwz(&self)  -> Self { return Vector4::new(self.y, self.x, self.w, self.z); }
        pub fn yxww(&self)  -> Self { return Vector4::new(self.y, self.x, self.w, self.w); }


        pub fn yyxx(&self)  -> Self { return Vector4::new(self.y, self.y, self.x, self.x); }
        pub fn yyxy(&self)  -> Self { return Vector4::new(self.y, self.y, self.x, self.y); }
        pub fn yyxz(&self)  -> Self { return Vector4::new(self.y, self.y, self.x, self.z); }
        pub fn yyxw(&self)  -> Self { return Vector4::new(self.y, self.y, self.x, self.w); }

        pub fn yyyx(&self)  -> Self { return Vector4::new(self.y, self.y, self.y, self.x); }
        pub fn yyyy(&self)  -> Self { return Vector4::new(self.y, self.y, self.y, self.y); }
        pub fn yyyz(&self)  -> Self { return Vector4::new(self.y, self.y, self.y, self.z); }
        pub fn yyyw(&self)  -> Self { return Vector4::new(self.y, self.y, self.y, self.w); }

        pub fn yyzx(&self)  -> Self { return Vector4::new(self.y, self.y, self.z, self.x); }
        pub fn yyzy(&self)  -> Self { return Vector4::new(self.y, self.y, self.z, self.y); }
        pub fn yyzz(&self)  -> Self { return Vector4::new(self.y, self.y, self.z, self.z); }
        pub fn yyzw(&self)  -> Self { return Vector4::new(self.y, self.y, self.z, self.w); }

        pub fn yywx(&self)  -> Self { return Vector4::new(self.y, self.y, self.w, self.x); }
        pub fn yywy(&self)  -> Self { return Vector4::new(self.y, self.y, self.w, self.y); }
        pub fn yywz(&self)  -> Self { return Vector4::new(self.y, self.y, self.w, self.z); }
        pub fn yyww(&self)  -> Self { return Vector4::new(self.y, self.y, self.w, self.w); }


        pub fn yzxx(&self)  -> Self { return Vector4::new(self.y, self.z, self.x, self.x); }
        pub fn yzxy(&self)  -> Self { return Vector4::new(self.y, self.z, self.x, self.y); }
        pub fn yzxz(&self)  -> Self { return Vector4::new(self.y, self.z, self.x, self.z); }
        pub fn yzxw(&self)  -> Self { return Vector4::new(self.y, self.z, self.x, self.w); }

        pub fn yzyx(&self)  -> Self { return Vector4::new(self.y, self.z, self.y, self.x); }
        pub fn yzyy(&self)  -> Self { return Vector4::new(self.y, self.z, self.y, self.y); }
        pub fn yzyz(&self)  -> Self { return Vector4::new(self.y, self.z, self.y, self.z); }
        pub fn yzyw(&self)  -> Self { return Vector4::new(self.y, self.z, self.y, self.w); }

        pub fn yzzx(&self)  -> Self { return Vector4::new(self.y, self.z, self.z, self.x); }
        pub fn yzzy(&self)  -> Self { return Vector4::new(self.y, self.z, self.z, self.y); }
        pub fn yzzz(&self)  -> Self { return Vector4::new(self.y, self.z, self.z, self.z); }
        pub fn yzzw(&self)  -> Self { return Vector4::new(self.y, self.z, self.z, self.w); }

        pub fn yzwx(&self)  -> Self { return Vector4::new(self.y, self.z, self.w, self.x); }
        pub fn yzwy(&self)  -> Self { return Vector4::new(self.y, self.z, self.w, self.y); }
        pub fn yzwz(&self)  -> Self { return Vector4::new(self.y, self.z, self.w, self.z); }
        pub fn yzww(&self)  -> Self { return Vector4::new(self.y, self.z, self.w, self.w); }

        
        pub fn ywxx(&self)  -> Self { return Vector4::new(self.y, self.w, self.x, self.x); }
        pub fn ywxy(&self)  -> Self { return Vector4::new(self.y, self.w, self.x, self.y); }
        pub fn ywxz(&self)  -> Self { return Vector4::new(self.y, self.w, self.x, self.z); }
        pub fn ywxw(&self)  -> Self { return Vector4::new(self.y, self.w, self.x, self.w); }

        pub fn ywyx(&self)  -> Self { return Vector4::new(self.y, self.w, self.y, self.x); }
        pub fn ywyy(&self)  -> Self { return Vector4::new(self.y, self.w, self.y, self.y); }
        pub fn ywyz(&self)  -> Self { return Vector4::new(self.y, self.w, self.y, self.z); }
        pub fn ywyw(&self)  -> Self { return Vector4::new(self.y, self.w, self.y, self.w); }

        pub fn ywzx(&self)  -> Self { return Vector4::new(self.y, self.w, self.z, self.x); }
        pub fn ywzy(&self)  -> Self { return Vector4::new(self.y, self.w, self.z, self.y); }
        pub fn ywzz(&self)  -> Self { return Vector4::new(self.y, self.w, self.z, self.z); }
        pub fn ywzw(&self)  -> Self { return Vector4::new(self.y, self.w, self.z, self.w); }

        pub fn ywwx(&self)  -> Self { return Vector4::new(self.y, self.w, self.w, self.x); }
        pub fn ywwy(&self)  -> Self { return Vector4::new(self.y, self.w, self.w, self.y); }
        pub fn ywwz(&self)  -> Self { return Vector4::new(self.y, self.w, self.w, self.z); }
        pub fn ywww(&self)  -> Self { return Vector4::new(self.y, self.w, self.w, self.w); }



        pub fn zxxx(&self)  -> Self { return Vector4::new(self.z, self.x, self.x, self.x); }
        pub fn zxxy(&self)  -> Self { return Vector4::new(self.z, self.x, self.x, self.y); }
        pub fn zxxz(&self)  -> Self { return Vector4::new(self.z, self.x, self.x, self.z); }
        pub fn zxxw(&self)  -> Self { return Vector4::new(self.z, self.x, self.x, self.w); }

        pub fn zxyx(&self)  -> Self { return Vector4::new(self.z, self.x, self.y, self.x); }
        pub fn zxyy(&self)  -> Self { return Vector4::new(self.z, self.x, self.y, self.y); }
        pub fn zxyz(&self)  -> Self { return Vector4::new(self.z, self.x, self.y, self.z); }
        pub fn zxyw(&self)  -> Self { return Vector4::new(self.z, self.x, self.y, self.w); }

        pub fn zxzx(&self)  -> Self { return Vector4::new(self.z, self.x, self.z, self.x); }
        pub fn zxzy(&self)  -> Self { return Vector4::new(self.z, self.x, self.z, self.y); }
        pub fn zxzz(&self)  -> Self { return Vector4::new(self.z, self.x, self.z, self.z); }
        pub fn zxzw(&self)  -> Self { return Vector4::new(self.z, self.x, self.z, self.w); }

        pub fn zxwx(&self)  -> Self { return Vector4::new(self.z, self.x, self.w, self.x); }
        pub fn zxwy(&self)  -> Self { return Vector4::new(self.z, self.x, self.w, self.y); }
        pub fn zxwz(&self)  -> Self { return Vector4::new(self.z, self.x, self.w, self.z); }
        pub fn zxww(&self)  -> Self { return Vector4::new(self.z, self.x, self.w, self.w); }


        pub fn zyxx(&self)  -> Self { return Vector4::new(self.z, self.y, self.x, self.x); }
        pub fn zyxy(&self)  -> Self { return Vector4::new(self.z, self.y, self.x, self.y); }
        pub fn zyxz(&self)  -> Self { return Vector4::new(self.z, self.y, self.x, self.z); }
        pub fn zyxw(&self)  -> Self { return Vector4::new(self.z, self.y, self.x, self.w); }

        pub fn zyyx(&self)  -> Self { return Vector4::new(self.z, self.y, self.y, self.x); }
        pub fn zyyy(&self)  -> Self { return Vector4::new(self.z, self.y, self.y, self.y); }
        pub fn zyyz(&self)  -> Self { return Vector4::new(self.z, self.y, self.y, self.z); }
        pub fn zyyw(&self)  -> Self { return Vector4::new(self.z, self.y, self.y, self.w); }

        pub fn zyzx(&self)  -> Self { return Vector4::new(self.z, self.y, self.z, self.x); }
        pub fn zyzy(&self)  -> Self { return Vector4::new(self.z, self.y, self.z, self.y); }
        pub fn zyzz(&self)  -> Self { return Vector4::new(self.z, self.y, self.z, self.z); }
        pub fn zyzw(&self)  -> Self { return Vector4::new(self.z, self.y, self.z, self.w); }

        pub fn zywx(&self)  -> Self { return Vector4::new(self.z, self.y, self.w, self.x); }
        pub fn zywy(&self)  -> Self { return Vector4::new(self.z, self.y, self.w, self.y); }
        pub fn zywz(&self)  -> Self { return Vector4::new(self.z, self.y, self.w, self.z); }
        pub fn zyww(&self)  -> Self { return Vector4::new(self.z, self.y, self.w, self.w); }


        pub fn zzxx(&self)  -> Self { return Vector4::new(self.z, self.z, self.x, self.x); }
        pub fn zzxy(&self)  -> Self { return Vector4::new(self.z, self.z, self.x, self.y); }
        pub fn zzxz(&self)  -> Self { return Vector4::new(self.z, self.z, self.x, self.z); }
        pub fn zzxw(&self)  -> Self { return Vector4::new(self.z, self.z, self.x, self.w); }

        pub fn zzyx(&self)  -> Self { return Vector4::new(self.z, self.z, self.y, self.x); }
        pub fn zzyy(&self)  -> Self { return Vector4::new(self.z, self.z, self.y, self.y); }
        pub fn zzyz(&self)  -> Self { return Vector4::new(self.z, self.z, self.y, self.z); }
        pub fn zzyw(&self)  -> Self { return Vector4::new(self.z, self.z, self.y, self.w); }

        pub fn zzzx(&self)  -> Self { return Vector4::new(self.z, self.z, self.z, self.x); }
        pub fn zzzy(&self)  -> Self { return Vector4::new(self.z, self.z, self.z, self.y); }
        pub fn zzzz(&self)  -> Self { return Vector4::new(self.z, self.z, self.z, self.z); }
        pub fn zzzw(&self)  -> Self { return Vector4::new(self.z, self.z, self.z, self.w); }

        pub fn zzwx(&self)  -> Self { return Vector4::new(self.z, self.z, self.w, self.x); }
        pub fn zzwy(&self)  -> Self { return Vector4::new(self.z, self.z, self.w, self.y); }
        pub fn zzwz(&self)  -> Self { return Vector4::new(self.z, self.z, self.w, self.z); }
        pub fn zzww(&self)  -> Self { return Vector4::new(self.z, self.z, self.w, self.w); }


        pub fn zwxx(&self)  -> Self { return Vector4::new(self.z, self.w, self.x, self.x); }
        pub fn zwxy(&self)  -> Self { return Vector4::new(self.z, self.w, self.x, self.y); }
        pub fn zwxz(&self)  -> Self { return Vector4::new(self.z, self.w, self.x, self.z); }
        pub fn zwxw(&self)  -> Self { return Vector4::new(self.z, self.w, self.x, self.w); }

        pub fn zwyx(&self)  -> Self { return Vector4::new(self.z, self.w, self.y, self.x); }
        pub fn zwyy(&self)  -> Self { return Vector4::new(self.z, self.w, self.y, self.y); }
        pub fn zwyz(&self)  -> Self { return Vector4::new(self.z, self.w, self.y, self.z); }
        pub fn zwyw(&self)  -> Self { return Vector4::new(self.z, self.w, self.y, self.w); }

        pub fn zwzx(&self)  -> Self { return Vector4::new(self.z, self.w, self.z, self.x); }
        pub fn zwzy(&self)  -> Self { return Vector4::new(self.z, self.w, self.z, self.y); }
        pub fn zwzz(&self)  -> Self { return Vector4::new(self.z, self.w, self.z, self.z); }
        pub fn zwzw(&self)  -> Self { return Vector4::new(self.z, self.w, self.z, self.w); }

        pub fn zwwx(&self)  -> Self { return Vector4::new(self.z, self.w, self.w, self.x); }
        pub fn zwwy(&self)  -> Self { return Vector4::new(self.z, self.w, self.w, self.y); }
        pub fn zwwz(&self)  -> Self { return Vector4::new(self.z, self.w, self.w, self.z); }
        pub fn zwww(&self)  -> Self { return Vector4::new(self.z, self.w, self.w, self.w); }



        pub fn wxxx(&self)  -> Self { return Vector4::new(self.w, self.x, self.x, self.x); }
        pub fn wxxy(&self)  -> Self { return Vector4::new(self.w, self.x, self.x, self.y); }
        pub fn wxxz(&self)  -> Self { return Vector4::new(self.w, self.x, self.x, self.z); }
        pub fn wxxw(&self)  -> Self { return Vector4::new(self.w, self.x, self.x, self.w); }

        pub fn wxyx(&self)  -> Self { return Vector4::new(self.w, self.x, self.y, self.x); }
        pub fn wxyy(&self)  -> Self { return Vector4::new(self.w, self.x, self.y, self.y); }
        pub fn wxyz(&self)  -> Self { return Vector4::new(self.w, self.x, self.y, self.z); }
        pub fn wxyw(&self)  -> Self { return Vector4::new(self.w, self.x, self.y, self.w); }

        pub fn wxzx(&self)  -> Self { return Vector4::new(self.w, self.x, self.z, self.x); }
        pub fn wxzy(&self)  -> Self { return Vector4::new(self.w, self.x, self.z, self.y); }
        pub fn wxzz(&self)  -> Self { return Vector4::new(self.w, self.x, self.z, self.z); }
        pub fn wxzw(&self)  -> Self { return Vector4::new(self.w, self.x, self.z, self.w); }

        pub fn wxwx(&self)  -> Self { return Vector4::new(self.w, self.x, self.w, self.x); }
        pub fn wxwy(&self)  -> Self { return Vector4::new(self.w, self.x, self.w, self.y); }
        pub fn wxwz(&self)  -> Self { return Vector4::new(self.w, self.x, self.w, self.z); }
        pub fn wxww(&self)  -> Self { return Vector4::new(self.w, self.x, self.w, self.w); }


        pub fn wyxx(&self)  -> Self { return Vector4::new(self.w, self.y, self.x, self.x); }
        pub fn wyxy(&self)  -> Self { return Vector4::new(self.w, self.y, self.x, self.y); }
        pub fn wyxz(&self)  -> Self { return Vector4::new(self.w, self.y, self.x, self.z); }
        pub fn wyxw(&self)  -> Self { return Vector4::new(self.w, self.y, self.x, self.w); }

        pub fn wyyx(&self)  -> Self { return Vector4::new(self.w, self.y, self.y, self.x); }
        pub fn wyyy(&self)  -> Self { return Vector4::new(self.w, self.y, self.y, self.y); }
        pub fn wyyz(&self)  -> Self { return Vector4::new(self.w, self.y, self.y, self.z); }
        pub fn wyyw(&self)  -> Self { return Vector4::new(self.w, self.y, self.y, self.w); }

        pub fn wyzx(&self)  -> Self { return Vector4::new(self.w, self.y, self.z, self.x); }
        pub fn wyzy(&self)  -> Self { return Vector4::new(self.w, self.y, self.z, self.y); }
        pub fn wyzz(&self)  -> Self { return Vector4::new(self.w, self.y, self.z, self.z); }
        pub fn wyzw(&self)  -> Self { return Vector4::new(self.w, self.y, self.z, self.w); }

        pub fn wywx(&self)  -> Self { return Vector4::new(self.w, self.y, self.w, self.x); }
        pub fn wywy(&self)  -> Self { return Vector4::new(self.w, self.y, self.w, self.y); }
        pub fn wywz(&self)  -> Self { return Vector4::new(self.w, self.y, self.w, self.z); }
        pub fn wyww(&self)  -> Self { return Vector4::new(self.w, self.y, self.w, self.w); }


        pub fn wzxx(&self)  -> Self { return Vector4::new(self.w, self.z, self.x, self.x); }
        pub fn wzxy(&self)  -> Self { return Vector4::new(self.w, self.z, self.x, self.y); }
        pub fn wzxz(&self)  -> Self { return Vector4::new(self.w, self.z, self.x, self.z); }
        pub fn wzxw(&self)  -> Self { return Vector4::new(self.w, self.z, self.x, self.w); }

        pub fn wzyx(&self)  -> Self { return Vector4::new(self.w, self.z, self.y, self.x); }
        pub fn wzyy(&self)  -> Self { return Vector4::new(self.w, self.z, self.y, self.y); }
        pub fn wzyz(&self)  -> Self { return Vector4::new(self.w, self.z, self.y, self.z); }
        pub fn wzyw(&self)  -> Self { return Vector4::new(self.w, self.z, self.y, self.w); }

        pub fn wzzx(&self)  -> Self { return Vector4::new(self.w, self.z, self.z, self.x); }
        pub fn wzzy(&self)  -> Self { return Vector4::new(self.w, self.z, self.z, self.y); }
        pub fn wzzz(&self)  -> Self { return Vector4::new(self.w, self.z, self.z, self.z); }
        pub fn wzzw(&self)  -> Self { return Vector4::new(self.w, self.z, self.z, self.w); }

        pub fn wzwx(&self)  -> Self { return Vector4::new(self.w, self.z, self.w, self.x); }
        pub fn wzwy(&self)  -> Self { return Vector4::new(self.w, self.z, self.w, self.y); }
        pub fn wzwz(&self)  -> Self { return Vector4::new(self.w, self.z, self.w, self.z); }
        pub fn wzww(&self)  -> Self { return Vector4::new(self.w, self.z, self.w, self.w); }


        pub fn wwxx(&self)  -> Self { return Vector4::new(self.w, self.w, self.x, self.x); }
        pub fn wwxy(&self)  -> Self { return Vector4::new(self.w, self.w, self.x, self.y); }
        pub fn wwxz(&self)  -> Self { return Vector4::new(self.w, self.w, self.x, self.z); }
        pub fn wwxw(&self)  -> Self { return Vector4::new(self.w, self.w, self.x, self.w); }

        pub fn wwyx(&self)  -> Self { return Vector4::new(self.w, self.w, self.y, self.x); }
        pub fn wwyy(&self)  -> Self { return Vector4::new(self.w, self.w, self.y, self.y); }
        pub fn wwyz(&self)  -> Self { return Vector4::new(self.w, self.w, self.y, self.z); }
        pub fn wwyw(&self)  -> Self { return Vector4::new(self.w, self.w, self.y, self.w); }

        pub fn wwzx(&self)  -> Self { return Vector4::new(self.w, self.w, self.z, self.x); }
        pub fn wwzy(&self)  -> Self { return Vector4::new(self.w, self.w, self.z, self.y); }
        pub fn wwzz(&self)  -> Self { return Vector4::new(self.w, self.w, self.z, self.z); }
        pub fn wwzw(&self)  -> Self { return Vector4::new(self.w, self.w, self.z, self.w); }

        pub fn wwwx(&self)  -> Self { return Vector4::new(self.w, self.w, self.w, self.x); }
        pub fn wwwy(&self)  -> Self { return Vector4::new(self.w, self.w, self.w, self.y); }
        pub fn wwwz(&self)  -> Self { return Vector4::new(self.w, self.w, self.w, self.z); }
        pub fn wwww(&self)  -> Self { return Vector4::new(self.w, self.w, self.w, self.w); }
    }

    // Utilities
    //      (Also implement to_latex)
    impl Vector2    { pub fn to_string(&self)   -> String {
        return format!("[{}, {}]", self.x, self.y); } }
    impl Complex    { pub fn to_string(&self)   -> String {
        return format!("[{} + {}i]", self.r, self.i); } }
    impl Dual       { pub fn to_string(&self)   -> String {
        return format!("[{} + {}ε]", self.r, self.e); } }
    impl Vector3    { pub fn to_string(&self)   -> String {
        return format!("[{}, {}, {}]", self.x, self.y, self.z); } }
    impl Vector4    { pub fn to_string(&self)   -> String {
        return format!("[{}, {}, {}, {}]", self.x, self.y, self.z, self.w); } }
    // impl Quaternion { /* Unfinished */ }
    // impl QuaternionDual { /* Unfinished */ }
    impl Matrix2    { pub fn to_string(&self)   -> String {
        return format!("[[{}, {}], [{}, {}]]", 
            self.e[0][0], self.e[0][1], 
            self.e[1][0], self.e[1][1]); } }
    impl Matrix3    { pub fn to_string(&self)   -> String {
        return format!("[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]", 
            self.e[0][0], self.e[0][1], self.e[0][2], 
            self.e[1][0], self.e[1][1], self.e[1][2],
            self.e[2][0], self.e[2][1], self.e[2][2]); } }
    impl Matrix4    { pub fn to_string(&self)   -> String {
        return format!("[[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]]", 
            self.e[0][0], self.e[0][1], self.e[0][2], self.e[0][3],
            self.e[1][0], self.e[1][1], self.e[1][2], self.e[1][3],
            self.e[2][0], self.e[2][1], self.e[2][2], self.e[2][3],
            self.e[3][0], self.e[3][1], self.e[3][2], self.e[3][3]); } }



    // Arithmetic
    //      Addition: { a + b, a += b }
    // Keep the example below - it might come of use regarding ownership (copy trait, etc.). 
    // This example should apply to most basic arithmetic operations (except assigns, unary operations, etc.). 
        /*
        impl Add<&Vector2> for &Vector2 { type Output = Vector2; fn add(self, v: &Vector2) -> Vector2 { 
            return Vector2::new(self.x + v.x, self.y + v.y); } }
        */
    impl Add<Vector2> for Vector2 { type Output = Self; fn add(self, v: Self) -> Self {
        return Self::new(self.x + v.x, self.y + v.y); } }
    impl Add<Complex> for Complex { type Output = Self; fn add(self, c: Self) -> Self {
        return Self::new(self.r + c.r, self.i + c.i); } }
    impl Add<Dual> for Dual { type Output = Self; fn add(self, d: Self) -> Self {
        return Self::new(self.r + d.r, self.e + d.e); } }
    impl Add<Vector3> for Vector3 { type Output = Self; fn add(self, v: Self) -> Self {
        return Self::new(self.x + v.x, self.y + v.y, self.z + v.z); } }
    impl Add<Vector4> for Vector4 { type Output = Self; fn add(self, v: Self) -> Self {
        return Self::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w); } }
    impl Add<Matrix2> for Matrix2 { type Output = Self; fn add(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] + m.e[0][0], self.e[0][1] + m.e[0][1],
            self.e[1][0] + m.e[1][0], self.e[1][1] + m.e[1][1]); } }
    impl Add<Matrix3> for Matrix3 { type Output = Self; fn add(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] + m.e[0][0], self.e[0][1] + m.e[0][1], self.e[0][2] + m.e[0][2],
            self.e[1][0] + m.e[1][0], self.e[1][1] + m.e[1][1], self.e[1][2] + m.e[1][2],
            self.e[2][0] + m.e[2][0], self.e[2][1] + m.e[2][1], self.e[2][2] + m.e[2][2]); } }
    impl Add<Matrix4> for Matrix4 { type Output = Self; fn add(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] + m.e[0][0], self.e[0][1] + m.e[0][1], self.e[0][2] + m.e[0][2], self.e[0][3] + m.e[0][3],
            self.e[1][0] + m.e[1][0], self.e[1][1] + m.e[1][1], self.e[1][2] + m.e[1][2], self.e[1][3] + m.e[1][3],
            self.e[2][0] + m.e[2][0], self.e[2][1] + m.e[2][1], self.e[2][2] + m.e[2][2], self.e[2][3] + m.e[2][3],
            self.e[3][0] + m.e[3][0], self.e[3][1] + m.e[3][1], self.e[3][2] + m.e[3][2], self.e[3][3] + m.e[3][3]); } }
    impl AddAssign<Vector2> for Vector2 { fn add_assign(&mut self, v: Self) { 
        self.x += v.x; self.y += v.y; } }
    impl AddAssign<Complex> for Complex { fn add_assign(&mut self, c: Self) { 
        self.r += c.r; self.i += c.i; } }
    impl AddAssign<Dual> for Dual { fn add_assign(&mut self, d: Self) { 
        self.r += d.r; self.e += d.e; } }
    impl AddAssign<Vector3> for Vector3 { fn add_assign(&mut self, v: Self) { 
        self.x += v.x; self.y += v.y; self.z += v.z; } }
    impl AddAssign<Vector4> for Vector4 { fn add_assign(&mut self, v: Self) { 
        self.x += v.x; self.y += v.y; self.z += v.z; self.w += v.w; } }
    impl AddAssign<Matrix2> for Matrix2 { fn add_assign(&mut self, m: Self) { 
        self.e[0][0] += m.e[0][0];  self.e[0][1] += m.e[0][1];
        self.e[1][0] += m.e[1][0];  self.e[1][1] += m.e[1][1]; } }
    impl AddAssign<Matrix3> for Matrix3 { fn add_assign(&mut self, m: Self) { 
        self.e[0][0] += m.e[0][0];  self.e[0][1] += m.e[0][1];  self.e[0][2] += m.e[0][2];
        self.e[1][0] += m.e[1][0];  self.e[1][1] += m.e[1][1];  self.e[1][2] += m.e[1][2];
        self.e[2][0] += m.e[2][0];  self.e[2][1] += m.e[2][1];  self.e[2][2] += m.e[2][2]; } }
    impl AddAssign<Matrix4> for Matrix4 { fn add_assign(&mut self, m: Self) { 
        self.e[0][0] += m.e[0][0];  self.e[0][1] += m.e[0][1];  self.e[0][2] += m.e[0][2];  self.e[0][3] += m.e[0][3];
        self.e[1][0] += m.e[1][0];  self.e[1][1] += m.e[1][1];  self.e[1][2] += m.e[1][2];  self.e[1][3] += m.e[1][3];
        self.e[2][0] += m.e[2][0];  self.e[2][1] += m.e[2][1];  self.e[2][2] += m.e[2][2];  self.e[2][3] += m.e[2][3];
        self.e[3][0] += m.e[3][0];  self.e[3][1] += m.e[3][1];  self.e[3][2] += m.e[3][2];  self.e[3][3] += m.e[3][3]; } }
    //      Subtraction: { a - b, a -= b, -a }
    impl Sub<Vector2> for Vector2 { type Output = Self; fn sub(self, v: Self) -> Self { 
        return Self::new(self.x - v.x, self.y - v.y); } }
    impl Sub<Complex> for Complex { type Output = Self; fn sub(self, c: Self) -> Self { 
        return Self::new(self.r - c.r, self.i - c.i); } }
    impl Sub<Dual> for Dual { type Output = Self; fn sub(self, d: Self) -> Self { 
        return Self::new(self.r - d.r, self.e - d.e); } }
    impl Sub<Vector3> for Vector3 { type Output = Self; fn sub(self, v: Self) -> Self { 
        return Self::new(self.x - v.x, self.y - v.y, self.z - v.z); } }
    impl Sub<Vector4> for Vector4 { type Output = Self; fn sub(self, v: Self) -> Self { 
        return Self::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w); } }
    impl Sub<Matrix2> for Matrix2 { type Output = Self; fn sub(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] - m.e[0][0], self.e[0][1] - m.e[0][1],
            self.e[1][0] - m.e[1][0], self.e[1][1] - m.e[1][1]); } }
    impl Sub<Matrix3> for Matrix3 { type Output = Self; fn sub(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] - m.e[0][0], self.e[0][1] - m.e[0][1], self.e[0][2] - m.e[0][2],
            self.e[1][0] - m.e[1][0], self.e[1][1] - m.e[1][1], self.e[1][2] - m.e[1][2],
            self.e[2][0] - m.e[2][0], self.e[2][1] - m.e[2][1], self.e[2][2] - m.e[2][2]); } }
    impl Sub<Matrix4> for Matrix4 { type Output = Self; fn sub(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] - m.e[0][0], self.e[0][1] - m.e[0][1], self.e[0][2] - m.e[0][2], self.e[0][3] - m.e[0][3],
            self.e[1][0] - m.e[1][0], self.e[1][1] - m.e[1][1], self.e[1][2] - m.e[1][2], self.e[1][3] - m.e[1][3],
            self.e[2][0] - m.e[2][0], self.e[2][1] - m.e[2][1], self.e[2][2] - m.e[2][2], self.e[2][3] - m.e[2][3],
            self.e[3][0] - m.e[3][0], self.e[3][1] - m.e[3][1], self.e[3][2] - m.e[3][2], self.e[3][3] - m.e[3][3]); } }
    impl SubAssign<Vector2> for Vector2 { fn sub_assign(&mut self, v: Self) { 
        self.x -= v.x; self.y -= v.y; } }
    impl SubAssign<Complex> for Complex { fn sub_assign(&mut self, c: Self) { 
        self.r -= c.r; self.i -= c.i; } }
    impl SubAssign<Dual> for Dual { fn sub_assign(&mut self, d: Self) { 
        self.r -= d.r; self.e -= d.e; } }
    impl SubAssign<Vector3> for Vector3 { fn sub_assign(&mut self, v: Self) { 
        self.x -= v.x; self.y -= v.y; self.z -= v.z; } }
    impl SubAssign<Vector4> for Vector4 { fn sub_assign(&mut self, v: Self) { 
        self.x -= v.x; self.y -= v.y; self.z -= v.z; self.w -= v.w; } }
    impl SubAssign<Matrix2> for Matrix2 { fn sub_assign(&mut self, m: Self) { 
        self.e[0][0] -= m.e[0][0];  self.e[0][1] -= m.e[0][1];
        self.e[1][0] -= m.e[1][0];  self.e[1][1] -= m.e[1][1]; } }
    impl SubAssign<Matrix3> for Matrix3 { fn sub_assign(&mut self, m: Self) { 
        self.e[0][0] -= m.e[0][0];  self.e[0][1] -= m.e[0][1];  self.e[0][2] -= m.e[0][2];
        self.e[1][0] -= m.e[1][0];  self.e[1][1] -= m.e[1][1];  self.e[1][2] -= m.e[1][2];
        self.e[2][0] -= m.e[2][0];  self.e[2][1] -= m.e[2][1];  self.e[2][2] -= m.e[2][2]; } }
    impl SubAssign<Matrix4> for Matrix4 { fn sub_assign(&mut self, m: Self) { 
        self.e[0][0] -= m.e[0][0];  self.e[0][1] -= m.e[0][1];  self.e[0][2] -= m.e[0][2];  self.e[0][3] -= m.e[0][3];
        self.e[1][0] -= m.e[1][0];  self.e[1][1] -= m.e[1][1];  self.e[1][2] -= m.e[1][2];  self.e[1][3] -= m.e[1][3];
        self.e[2][0] -= m.e[2][0];  self.e[2][1] -= m.e[2][1];  self.e[2][2] -= m.e[2][2];  self.e[2][3] -= m.e[2][3];
        self.e[3][0] -= m.e[3][0];  self.e[3][1] -= m.e[3][1];  self.e[3][2] -= m.e[3][2];  self.e[3][3] -= m.e[3][3]; } }
    impl Neg for Vector2 { type Output = Self; fn neg(self) -> Self { 
        return Self::new(-self.x, -self.y); } }
    impl Neg for Vector3 { type Output = Self; fn neg(self) -> Self { 
        return Self::new(-self.x, -self.y, -self.z); } }
    impl Neg for Vector4 { type Output = Self; fn neg(self) -> Self { 
        return Self::new(-self.x, -self.y, -self.z, -self.w); } }
    //      Special unary operators (conjugate, transpose, inverse, etc.)
    //          Conjugate (-(a + bi) = (a - bi))
    impl Neg for Complex { type Output = Self; fn neg(self) -> Self { 
        return Self::new(self.r, -self.i); } }
    impl Neg for Dual { type Output = Self; fn neg(self) -> Self { 
        return Self::new(self.r, -self.e); } }
    //          Transpose
    impl Neg for Matrix2 { type Output = Self; fn neg(self) -> Self {
        return self.transpose(); } }
    impl Neg for Matrix3 { type Output = Self; fn neg(self) -> Self {
        return self.transpose(); } }
    impl Neg for Matrix4 { type Output = Self; fn neg(self) -> Self {
        return self.transpose(); } }
    //          Inverse
    impl Not for Complex { type Output = Self; fn not(self) -> Self {
        let d = 1.0 / self.magnitude_sqr();
        return (-self) * d; } }
    impl Not for Matrix2 { type Output = Self; fn not(self) -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }
    impl Not for Matrix3 { type Output = Self; fn not(self) -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }
    impl Not for Matrix4 { type Output = Self; fn not(self) -> Self {
        if self.determinant() == 0.0 {
            return Self::zero();
        } else {
            return self.adjugate() / self.determinant(); } } }
    //      Scalar-Struct Multiplication: { a * s, a *= s, s * a }
    impl Mul<f32> for Vector2 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(self.x * s, self.y * s); } }
    impl Mul<f32> for Complex { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(self.r * s, self.i * s); } }
    impl Mul<f32> for Dual { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(self.r * s, self.e * s); } }
    impl Mul<f32> for Vector3 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(self.x * s, self.y * s, self.z * s); } }
    impl Mul<f32> for Vector4 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(self.x * s, self.y * s, self.z * s, self.w * s); } }
    impl Mul<f32> for Matrix2 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(
            self.e[0][0] * s, self.e[0][1] * s, 
            self.e[1][0] * s, self.e[1][1] * s); } }
    impl Mul<f32> for Matrix3 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(
            self.e[0][0] * s, self.e[0][1] * s, self.e[0][2] * s,
            self.e[1][0] * s, self.e[1][1] * s, self.e[1][2] * s,
            self.e[2][0] * s, self.e[2][1] * s, self.e[2][2] * s); } }
    impl Mul<f32> for Matrix4 { type Output = Self; fn mul(self, s: f32) -> Self { 
        return Self::new(
            self.e[0][0] * s, self.e[0][1] * s, self.e[0][2] * s, self.e[0][3] * s,
            self.e[1][0] * s, self.e[1][1] * s, self.e[1][2] * s, self.e[1][3] * s,
            self.e[2][0] * s, self.e[2][1] * s, self.e[2][2] * s, self.e[2][3] * s,
            self.e[3][0] * s, self.e[3][1] * s, self.e[3][2] * s, self.e[3][3] * s); } }
    impl MulAssign<f32> for Vector2 { fn mul_assign(&mut self, s: f32) { 
        self.x *= s; self.y *= s; } }
    impl MulAssign<f32> for Complex { fn mul_assign(&mut self, s: f32) { 
        self.r *= s; self.i *= s; } }
    impl MulAssign<f32> for Dual { fn mul_assign(&mut self, s: f32) { 
        self.r *= s; self.e *= s; } }
    impl MulAssign<f32> for Vector3 { fn mul_assign(&mut self, s: f32) { 
        self.x *= s; self.y *= s; self.z *= s; } }
    impl MulAssign<f32> for Vector4 { fn mul_assign(&mut self, s: f32) { 
        self.x *= s; self.y *= s; self.z *= s; self.w *= s; } }
    impl MulAssign<f32> for Matrix2 { fn mul_assign(&mut self, s: f32) { 
        self.e[0][0] *= s; self.e[0][1] *= s; 
        self.e[0][1] *= s; self.e[1][1] *= s; } }
    impl MulAssign<f32> for Matrix3 { fn mul_assign(&mut self, s: f32) { 
        self.e[0][0] *= s; self.e[0][1] *= s; self.e[0][2] *= s;
        self.e[1][0] *= s; self.e[1][1] *= s; self.e[1][2] *= s;
        self.e[2][0] *= s; self.e[2][1] *= s; self.e[2][2] *= s; } }
    impl MulAssign<f32> for Matrix4 { fn mul_assign(&mut self, s: f32) { 
        self.e[0][0] *= s; self.e[0][1] *= s; self.e[0][2] *= s; self.e[0][3] *= s;
        self.e[1][0] *= s; self.e[1][1] *= s; self.e[1][2] *= s; self.e[1][3] *= s;
        self.e[2][0] *= s; self.e[2][1] *= s; self.e[2][2] *= s; self.e[2][3] *= s;
        self.e[3][0] *= s; self.e[3][1] *= s; self.e[3][2] *= s; self.e[3][3] *= s;} }
    impl Mul<Vector2> for f32 { type Output = Vector2; fn mul(self, v: Vector2) -> Vector2 {
        return Vector2::new(v.x * self, v.y * self); } }
    impl Mul<Complex> for f32 { type Output = Complex; fn mul(self, c: Complex) -> Complex {
        return Complex::new(c.r * self, c.i * self); } }
    impl Mul<Dual> for f32 { type Output = Dual; fn mul(self, d: Dual) -> Dual {
        return Dual::new(d.r * self, d.e * self); } }
    impl Mul<Vector3> for f32 { type Output = Vector3; fn mul(self, v: Vector3) -> Vector3 {
        return Vector3::new(v.x * self, v.y * self, v.z * self); } }
    impl Mul<Vector4> for f32 { type Output = Vector4; fn mul(self, v: Vector4) -> Vector4 {
        return Vector4::new(v.x * self, v.y * self, v.z * self, v.w * self); } }
    impl Mul<Matrix2> for f32 { type Output = Matrix2; fn mul(self, m: Matrix2) -> Matrix2 {
        return Matrix2::new(
            m.e[0][0] * self, m.e[0][1] * self, 
            m.e[1][0] * self, m.e[1][1] * self); } }
    impl Mul<Matrix3> for f32 { type Output = Matrix3; fn mul(self, m: Matrix3) -> Matrix3 {
        return Matrix3::new(
            m.e[0][0] * self, m.e[0][1] * self, m.e[0][2] * self,
            m.e[1][0] * self, m.e[1][1] * self, m.e[1][2] * self,
            m.e[2][0] * self, m.e[2][1] * self, m.e[2][2] * self); } }
    impl Mul<Matrix4> for f32 { type Output = Matrix4; fn mul(self, m: Matrix4) -> Matrix4 {
        return Matrix4::new(
            m.e[0][0] * self, m.e[0][1] * self, m.e[0][2] * self, m.e[0][3] * self,
            m.e[1][0] * self, m.e[1][1] * self, m.e[1][2] * self, m.e[1][3] * self,
            m.e[2][0] * self, m.e[2][1] * self, m.e[2][2] * self, m.e[2][3] * self,
            m.e[3][0] * self, m.e[3][1] * self, m.e[3][2] * self, m.e[3][3] * self); } }
    
    /*
        Struct-Struct Multiplication: { 
            scalar (dot "*"), vector (cross "/"; z-value only), geometric (wedge "^"), triple products,
            complex multiplication, dual multiplication,
            matrix multiplication }
    */
    impl Mul<Vector2> for Vector2 { type Output = f32; fn mul(self, v: Self) -> f32 {
        return self.x * v.x + self.y * v.y; } }
    impl Mul<Complex> for Complex { type Output = Self; fn mul(self, c: Self) -> Self {
        return Self::new(self.r * self.i - c.r * c.i, self.r * c.i + c.r * self.i); } }
    impl Mul<Dual> for Dual { type Output = Self; fn mul(self, d: Self) -> Self {
        return Self::new(self.r * self.e, self.r * d.e + d.r * self.e); } }
    impl Mul<Vector3> for Vector3 { type Output = f32; fn mul(self, v: Self) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z; } }
    impl Mul<Vector4> for Vector4 { type Output = f32; fn mul(self, v: Self) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w; } }
    impl Mul<Matrix2> for Matrix2 { type Output = Self; fn mul(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] * m.e[0][0] + self.e[0][1] * m.e[1][0],
            self.e[0][0] * m.e[0][1] + self.e[0][1] * m.e[1][1],

            self.e[1][0] * m.e[0][0] + self.e[1][1] * m.e[1][0],
            self.e[1][0] * m.e[0][1] + self.e[1][1] * m.e[1][1]); } }
    impl Mul<Matrix3> for Matrix3 { type Output = Self; fn mul(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] * m.e[0][0] + self.e[0][1] * m.e[1][0] + self.e[0][2] * m.e[2][0],
            self.e[0][0] * m.e[0][1] + self.e[0][1] * m.e[1][1] + self.e[0][2] * m.e[2][1],
            self.e[0][0] * m.e[0][2] + self.e[0][1] * m.e[1][2] + self.e[0][2] * m.e[2][2],

            self.e[1][0] * m.e[0][0] + self.e[1][1] * m.e[1][0] + self.e[1][2] * m.e[2][0],
            self.e[1][0] * m.e[0][1] + self.e[1][1] * m.e[1][1] + self.e[1][2] * m.e[2][1],
            self.e[1][0] * m.e[0][2] + self.e[1][1] * m.e[1][2] + self.e[1][2] * m.e[2][2],

            self.e[2][0] * m.e[0][0] + self.e[2][1] * m.e[1][0] + self.e[2][2] * m.e[2][0],
            self.e[2][0] * m.e[0][1] + self.e[2][1] * m.e[1][1] + self.e[2][2] * m.e[2][1],
            self.e[2][0] * m.e[0][2] + self.e[2][1] * m.e[1][2] + self.e[2][2] * m.e[2][2]); } }
    impl Mul<Matrix4> for Matrix4 { type Output = Self; fn mul(self, m: Self) -> Self {
        return Self::new(
            self.e[0][0] * m.e[0][0] + self.e[0][1] * m.e[1][0] + self.e[0][2] * m.e[2][0] + self.e[0][3] * m.e[3][0],
            self.e[0][0] * m.e[0][1] + self.e[0][1] * m.e[1][1] + self.e[0][2] * m.e[2][1] + self.e[0][3] * m.e[3][1],
            self.e[0][0] * m.e[0][2] + self.e[0][1] * m.e[1][2] + self.e[0][2] * m.e[2][2] + self.e[0][3] * m.e[3][2],
            self.e[0][0] * m.e[0][3] + self.e[0][1] * m.e[1][3] + self.e[0][2] * m.e[2][3] + self.e[0][3] * m.e[3][3],

            self.e[1][0] * m.e[0][0] + self.e[1][1] * m.e[1][0] + self.e[1][2] * m.e[2][0] + self.e[1][3] * m.e[3][0],
            self.e[1][0] * m.e[0][1] + self.e[1][1] * m.e[1][1] + self.e[1][2] * m.e[2][1] + self.e[1][3] * m.e[3][1],
            self.e[1][0] * m.e[0][2] + self.e[1][1] * m.e[1][2] + self.e[1][2] * m.e[2][2] + self.e[1][3] * m.e[3][2],
            self.e[1][0] * m.e[0][3] + self.e[1][1] * m.e[1][3] + self.e[1][2] * m.e[2][3] + self.e[1][3] * m.e[3][3],

            self.e[2][0] * m.e[0][0] + self.e[2][1] * m.e[1][0] + self.e[2][2] * m.e[2][0] + self.e[2][3] * m.e[3][0],
            self.e[2][0] * m.e[0][1] + self.e[2][1] * m.e[1][1] + self.e[2][2] * m.e[2][1] + self.e[2][3] * m.e[3][1],
            self.e[2][0] * m.e[0][2] + self.e[2][1] * m.e[1][2] + self.e[2][2] * m.e[2][2] + self.e[2][3] * m.e[3][2],
            self.e[2][0] * m.e[0][3] + self.e[2][1] * m.e[1][3] + self.e[2][2] * m.e[2][3] + self.e[2][3] * m.e[3][3],

            self.e[3][0] * m.e[0][0] + self.e[3][1] * m.e[1][0] + self.e[3][2] * m.e[2][0] + self.e[3][3] * m.e[3][0],
            self.e[3][0] * m.e[0][1] + self.e[3][1] * m.e[1][1] + self.e[3][2] * m.e[2][1] + self.e[3][3] * m.e[3][1],
            self.e[3][0] * m.e[0][2] + self.e[3][1] * m.e[1][2] + self.e[3][2] * m.e[2][2] + self.e[3][3] * m.e[3][2],
            self.e[3][0] * m.e[0][3] + self.e[3][1] * m.e[1][3] + self.e[3][2] * m.e[2][3] + self.e[3][3] * m.e[3][3]); } }
    impl Vector2 { fn product_scalar(a: &Self, b: &Self) -> f32 { 
        return &a.x * &b.x + &a.y * &b.y; } }
    impl Vector3 { fn product_scalar(a: &Self, b: &Self) -> f32 { 
        return &a.x * &b.x + &a.y * &b.y + &a.z * &b.z; } }
    impl Vector4 { fn product_scalar(a: &Self, b: &Self) -> f32 { 
        return &a.x * &b.x + &a.y * &b.y + &a.z * &b.z + &a.w * &b.w; } }
    impl Div<Vector2> for Vector2 { type Output = f32; fn div(self, v: Self) -> f32 {
        return self.x * v.y - self.y * v.x; }}
    impl Div<Vector3> for Vector3 { type Output = Self; fn div(self, v: Self) -> Self {
        return Self::new(
            self.y * v.z - v.y * self.z,
            v.x * self.z - self.x * v.z,
            self.x * v.y - v.x * self.y); } }
    impl Vector2 { fn product_vector(a: &Self, b: &Self) -> f32 { 
        return a.x * b.y - a.y * b.x; } }
    impl Vector3 { fn product_vector(a: &Self, b: &Self) -> Self { 
        return Self::new(
            a.y * b.z - b.y * a.z,
            b.x * a.z - a.x * b.z,
            a.x * b.y - b.x * a.y); } }
    impl BitXor<Vector2> for Vector2 { type Output = Vector2 /*Bivector2*/; fn bitxor(self, v: Vector2) -> Vector2 /*Bivector2*/ {
        return Vector2::new(0.0, 0.0); /* This is just filler */ }}
        // Also do Vector3 and Vector4
    impl Vector2 {
        // fn product_geometric(a: &Vector2, b: &Vector2) -> Bivector2 {
        //     return ()
        // }
    }
    impl Vector3 {
        // geometric product
    }
    impl Vector4 {
        // geometric product
    }
    impl Vector3 {
        // http://mathworld.wolfram.com/ScalarTripleProduct.html
        // [a * (b / c)]
        pub fn product_scalar_triple(a: Self, b: Self, c: Self) -> f32 {
            return Vector3::product_scalar(&a, &Vector3::product_vector(&b, &c));
        }
        // http://mathworld.wolfram.com/VectorTripleProduct.html
        // [a / (b / c)] or [b * (a * c) - c * (a * b)]
        pub fn product_vector_triple(a: Self, b: Self, c: Self) -> Self {
            return b * Vector3::product_scalar(&a, &c) - c * Vector3::product_scalar(&a, &b);
        }
    }
    impl Vector4 {
        // scalar triple / vector triple exists in R4?
    }

    //      Division
    impl Div<f32> for Vector2 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return Self::new(self.x, self.y) * t; } }
    impl Div<f32> for Complex { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return Self::new(self.r, self.i) * t; } }
    impl Div<f32> for Dual { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return Self::new(self.r, self.e) * t; } }
    impl Div<f32> for Vector3 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return Self::new(self.x, self.y, self.z) * t; } }
    impl Div<f32> for Vector4 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return self * t; } }
    impl Div<f32> for Matrix2 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return self * t; } }
    impl Div<f32> for Matrix3 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return self * t; } }
    impl Div<f32> for Matrix4 { type Output = Self; fn div(self, s: f32) -> Self { 
        let t = 1.0 / s; return self * t; } }
    impl DivAssign<f32> for Vector2 { fn div_assign(&mut self, s: f32) { 
        let t = 1.0 / s; self.x *= t; self.y *= t; } }
    impl DivAssign<f32> for Complex { fn div_assign(&mut self, s: f32) { 
        let t = 1.0 / s; self.r *= t; self.i *= t; } }
    impl DivAssign<f32> for Dual { fn div_assign(&mut self, s: f32) { 
        let t = 1.0 / s; self.r *= t; self.e *= t; } }
    impl DivAssign<f32> for Vector3 { fn div_assign(&mut self, s: f32) { 
        let t = 1.0 / s; self.x *= t; self.y *= t; self.z *= t; } }
    impl DivAssign<f32> for Vector4 { fn div_assign(&mut self, s: f32) { 
        let t = 1.0 / s; self.x *= t; self.y *= t; self.z *= t; self.w *= t; } }


    // Inter-Struct Product: { Matrix-Vector }
    // Geometry
    //      Magnitude
    impl Vector2 { pub fn magnitude(&self)      -> f32 { return f32::sqrt((*self) * (*self)); } }
    impl Complex { pub fn magnitude(&self)      -> f32 { return f32::sqrt(self.r * self.r + self.i * self.i); } }
    impl Vector3 { pub fn magnitude(&self)      -> f32 { return f32::sqrt((*self) * (*self)); } }
    impl Vector4 { pub fn magnitude(&self)      -> f32 { return f32::sqrt((*self) * (*self)); } }
    impl Matrix2 { pub fn determinant(&self)    -> f32 { return 
        (*self).e[0][0] 
            * (*self).minor(0, 0) 
        - (*self).e[1][0] 
            * (*self).minor(1, 0); } }
    impl Matrix2 { pub fn determinant2(&self)   -> f32 { 
        let dia = self.triangular_lower().diagonal();
        return dia.x * dia.y; } }
    impl Matrix3 { pub fn determinant(&self)    -> f32 { return
        (*self).e[0][0]
            * (*self).minor(0, 0).determinant()
        - (*self).e[0][1]
            * (*self).minor(0, 1).determinant()
        + (*self).e[0][2]
            * (*self).minor(0, 2).determinant(); } }
    impl Matrix3 { pub fn determinant2(&self)   -> f32 {
        let dia = self.triangular_lower().diagonal();
        return dia.x * dia.y * dia.z; } }
    impl Matrix4 { pub fn determinant(&self)    -> f32 { return
        (*self).e[0][0]
            * (*self).minor(0, 0).determinant()
        - (*self).e[0][1]
            * (*self).minor(0, 1).determinant()
        + (*self).e[0][2]
            * (*self).minor(0, 2).determinant()
        - (*self).e[0][3]
            * (*self).minor(0, 3).determinant(); } }
    impl Matrix4 { pub fn determinant2(&self)   -> f32 {
        let dia = self.triangular_lower().diagonal();
        return dia.x * dia.y * dia.z * dia.w; } }
    impl Vector2 { pub fn magnitude_sqr(&self)  -> f32 { return (*self) * (*self); } }
    impl Complex { pub fn magnitude_sqr(&self)  -> f32 { return self.r * self.r + self.i * self.i; } }
    impl Vector3 { pub fn magnitude_sqr(&self)  -> f32 { return (*self) * (*self); } }
    impl Vector4 { pub fn magnitude_sqr(&self)  -> f32 { return (*self) * (*self); } }
    impl Vector2 { pub fn normalization(&self)  -> Self { let d = 1.0 / self.magnitude(); return (*self) * d; } }
    impl Vector3 { pub fn normalization(&self)  -> Self { let d = 1.0 / self.magnitude(); return (*self) * d; } }
    impl Vector4 { pub fn normalization(&self)  -> Self { let d = 1.0 / self.magnitude(); return (*self) * d; } }
    // /* Consider not using mutating functions */ impl Vector2 { pub fn normalize(&mut self)  { let d = 1.0 / self.mag(); self.x *= d; self.y *= d; } }
    
    //      Interpolation
    impl Vector2    { pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return (*a) + ((*b) - (*a)) * t; } }
    impl Complex    { pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return (*a) + ((*b) - (*a)) * t; } }
    impl Dual       { pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return (*a) + ((*b) - (*a)) * t; } }
    impl Vector3    { pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return (*a) + ((*b) - (*a)) * t; } }
    impl Vector4    { pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return (*a) + ((*b) - (*a)) * t; } }
    // impl Vector2 { pub fn slerp(a: &Self, b: &Self, t: f32) -> Self { return; } }
    // impl Vector3 { pub fn slerp(a: &Self, b: &Self, t: f32) -> Self { return; } }
    // impl Vector4 { pub fn slerp(a: &Self, b: &Self, t: f32) -> Self { return; } }
    
    //      Measurement (angles in radians)
    impl Vector2    { pub fn angle(a: &Self, b: &Self)         -> f32 { return f32::acos(((*a) * (*b)) / (a.magnitude() * b.magnitude())); } }
    impl Vector3    { pub fn angle(a: &Self, b: &Self)         -> f32 { return f32::acos(((*a) * (*b)) / (a.magnitude() * b.magnitude())); } }
    impl Vector4    { pub fn angle(a: &Self, b: &Self)         -> f32 { return f32::acos(((*a) * (*b)) / (a.magnitude() * b.magnitude())); } }
    impl Vector2    { pub fn angle_safe(a: &Self, b: &Self)    -> f32 {
        let d = a.magnitude() * b.magnitude();
        if d <= f32::EPSILON {
            return f32::NAN;
        } else {
            return Self::angle(&a, &b);
        }
    } }
    impl Vector3    { pub fn angle_safe(a: &Self, b: &Self)    -> f32 {
        let d = a.magnitude() * b.magnitude();
        if d <= f32::EPSILON {
            return f32::NAN;
        } else {
            return Self::angle(&a, &b);
        }
    } }
    impl Vector4    { pub fn angle_safe(a: &Self, b: &Self)    -> f32 {
        let d = a.magnitude() * b.magnitude();
        if d <= f32::EPSILON {
            return f32::NAN;
        } else {
            return Self::angle(&a, &b);
        }
    } }
    impl Vector2    { pub fn angle_unit(a: &Self, b: &Self)    -> f32 { return f32::acos((*a) * (*b)); } }
    impl Vector3    { pub fn angle_unit(a: &Self, b: &Self)    -> f32 { return f32::acos((*a) * (*b)); } }
    impl Vector4    { pub fn angle_unit(a: &Self, b: &Self)    -> f32 { return f32::acos((*a) * (*b)); } }
    // Taken from https://stackoverflow.com/questions/14066933/direct-way-of-computing-clockwise-angle-between-2-vectors
    impl Vector2    { pub fn angle_signed(a: &Self, b: &Self)  -> f32 {
        let dot = (*a) * (*b);
        let det = (*a) / (*b);
        let mut angle = f32::atan2(det, dot);
        if angle < 0.0 {
            angle += TAU;
        }
        return angle;
    } }
    // impl Vector3 { pub fn angle_signed(a: &Self, b: &Self)  -> f32 {

    // } }
    //      Vector Projection
    impl Vector2 { pub fn projection(a: &Self, b: &Self)        -> Self { return (*b) * (((*a) * (*b)) / ((*b) * (*b))); } }
    impl Vector3 { pub fn projection(a: &Self, b: &Self)        -> Self { return (*b) * (((*a) * (*b)) / ((*b) * (*b))); } }
    impl Vector4 { pub fn projection(a: &Self, b: &Self)        -> Self { return (*b) * (((*a) * (*b)) / ((*b) * (*b))); } }
    impl Vector2 { pub fn projection_unit(a: &Self, b: &Self)   -> Self { return (*b) * ((*a) * (*b)); } }
    impl Vector3 { pub fn projection_unit(a: &Self, b: &Self)   -> Self { return (*b) * ((*a) * (*b)); } }
    impl Vector4 { pub fn projection_unit(a: &Self, b: &Self)   -> Self { return (*b) * ((*a) * (*b)); } }
    //      Vector Rejection
    impl Vector2 { pub fn rejection(a: &Self, b: &Self)         -> Self { return (*a) - Self::projection(a, b); } }
    impl Vector3 { pub fn rejection(a: &Self, b: &Self)         -> Self { return (*a) - Self::projection(a, b); } }
    impl Vector4 { pub fn rejection(a: &Self, b: &Self)         -> Self { return (*a) - Self::projection(a, b); } }
    impl Vector2 { pub fn rejection_unit(a: &Self, b: &Self)    -> Self { return (*a) - Self::projection_unit(a, b); } }
    impl Vector3 { pub fn rejection_unit(a: &Self, b: &Self)    -> Self { return (*a) - Self::projection_unit(a, b); } }
    impl Vector4 { pub fn rejection_unit(a: &Self, b: &Self)    -> Self { return (*a) - Self::projection_unit(a, b); } }
    //      Vector Reflection
    impl Vector2 { pub fn reflection(a: &Self, b: &Self)        -> Self { return (*a) - 2.0 * Self::projection(a, b); } }
    impl Vector3 { pub fn reflection(a: &Self, b: &Self)        -> Self { return (*a) - 2.0 * Self::projection(a, b); } }
    impl Vector4 { pub fn reflection(a: &Self, b: &Self)        -> Self { return (*a) - 2.0 * Self::projection(a, b); } }
    impl Vector2 { pub fn reflection_unit(a: &Self, b: &Self)   -> Self { return (*a) - 2.0 * Self::projection_unit(a, b); } }
    impl Vector3 { pub fn reflection_unit(a: &Self, b: &Self)   -> Self { return (*a) - 2.0 * Self::projection_unit(a, b); } }
    impl Vector4 { pub fn reflection_unit(a: &Self, b: &Self)   -> Self { return (*a) - 2.0 * Self::projection_unit(a, b); } }
    //      Vector Refraction
    impl Vector2 { pub fn refraction(a: &Self, b: &Self, n1: f32, n2: f32)      -> Self {
        let mag = a.magnitude();
        let n = n1 / n2;
        let t1 = Self::angle(&(-(*a)), b);
        let mut signum = f32::cos(Self::angle(a, &Self::right()));
        if signum > 0.0 {
            signum = 1.0;
        } else {
            signum = (-1.0);
        }
        let t2 = f32::asin(n * f32::sin(t1)) * signum;
        let t3 = Self::angle(b, &Self::right());
        let arg = t3 + PI + t2;

        return Self::from_polar(arg, mag);
    } }
    impl Vector2 { pub fn refraction_unit(a: &Self, b: &Self, n1: f32, n2: f32) -> Self {
        let n = n1 / n2;
        let t1 = Self::angle_unit(&(-(*a)), b);
        let mut signum = f32::cos(Self::angle_unit(a, &Self::right()));
        if signum > 0.0 {
            signum = 1.0;
        } else {
            signum = (-1.0);
        }
        let t2 = f32::asin(n * f32::sin(t1)) * signum;
        let t3 = Self::angle(b, &Self::right());
        let arg = t3 + PI + t2;

        return Self::from_polar(arg, 1.0);
    } }


    /*
        // Queries
        ==
        <
        <=
        >
        >=
        isNormalized
        isParallel
        isParallelUnit
        isAntiParallel
        isAntiParallelUnit
        isCollinear
        isCollinearUnit
        isOrthogonal
        isOrthogonalUnit
        TestMode {
            // Test difference in magnitudes in world-space units
            AbsoluteMagnitude,
            // Test difference in magnitude in percentage
            RelativeMagnitude,
            // Test difference in coordinates in world-space units
            AbsoluteCoordinates,
            // Test difference in coordinates in percentage
            RelativeCoordinates
        }
    */
}