// Math library for the game engine, rendering engine, and physics engine
pub mod num {
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

pub mod linalg {
    pub mod vector {
        use super::super::num;
        use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign };

        
        #[derive(Debug)]
        pub struct Vector2 {
            x: f32,
            y: f32,
        }
        impl Vector2 {
            // Construction
            pub fn new() -> Self {
                return Self { x: 0.0, y: 0.0 };
            }
            pub fn from_floats(a: f32, b: f32) -> Self {
                return Self { x: a, y: b };
            }
            pub fn from_vector2(v: Vector2) -> Self {
                return Self { x: v.x, y: v.y };
            }
            pub fn from_vector3(v: Vector3) -> Self {
                return Self { x: v.x, y: v.y };
            }
            pub fn from_vector4(v: Vector4) -> Self {
                return Self { x: v.x, y: v.y };
            }
            
            // Prefabricated vectors
            pub fn one() -> Self {
                return Self { x: 1.0, y: 1.0 };
            }
            pub fn zero() -> Self {
                return Self { x: 0.0, y: 0.0 };
            }
            pub fn right() -> Self {
                return Self { x: 1.0, y: 0.0 };
            }
            pub fn left() -> Self {
                return Self { x: -1.0, y: 0.0 };
            }
            pub fn up() -> Self {
                return Self { x: 0.0, y: 1.0 };
            }
            pub fn down() -> Self {
                return Self { x: 0.0, y: -1.0 };
            }
            pub fn QI() -> Self {
                return Self { x: 1.0, y: 1.0 };
            }
            pub fn QIn() -> Self {
                return Self { x: num::SQRT2OVER2, y: num::SQRT2OVER2 };
            }
            pub fn QII() -> Self {
                return Self { x: -num::SQRT2OVER2, y: num::SQRT2OVER2 };
            }
            pub fn QIII() -> Self {
                return Self { x: -num::SQRT2OVER2, y: -num::SQRT2OVER2 };
            }
            pub fn QIV() -> Self {
                return Self { x: num::SQRT2OVER2, y: -num::SQRT2OVER2 };
            }

            // Utilities
            pub fn to_string(&self) -> String {
                return format!("[{}, {}]", self.x, self.y);
            }
            pub fn to_latex(&self) -> String {
                return format!("[{}, {}]", self.x, self.y);
            }
        }

        // Vector arithmetic
        impl Add for Vector2 {
            type Output = Vector2;
            fn add(self, v: Vector2) -> Vector2 {
                return Self { x: self.x + v.x, y: self.y + v.y };
            }
        }
        impl AddAssign for Vector2 {
            fn add_assign(&mut self, v: Vector2) {
                self.x += v.x;
                self.y += v.y;
            }
        }
        




        pub struct Vector3 {
            x: f32,
            y: f32,
            z: f32,
        }



        pub struct Vector4 {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }
    }
    mod matrix {
        struct Matrix2 {
            // ...
        }
        struct Matrix3 {
            // ...
        }
        struct Matrix4 {
            // ...
        }
        struct MatrixN {
            // ...
        }
    }
}