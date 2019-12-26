// Math library for the game engine, rendering engine, and physics engine
use std::f32::{ self, EPSILON, NAN };

/*
    To do: 
    - Consider whether to keep member-altering functions like normalize(), div_assign(), etc. 
    - Maybe rather make it a conscious decision to do those types of operations instead? 
    - Use Self::new() on swizzling, prefabs, etc. 
    - Vector3 projection, and all the Vector4 geometric operations
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

    pub mod complex {
        // Includes
        use super::super::num::constants;
        use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign };

        // Complex number form:         c = a + bi
        // Complex numbers base law:    i^2 = (-1)
        #[derive(Debug)]
        pub struct Complex {
            a: f32,
            b: f32,
        }

        // Construction
        impl Complex {
            pub fn new(s: f32, t: f32) -> Self {
                return Self {
                    a: s,
                    b: t
                };
            }
            pub fn from_polar(radian: f32, radius: f32) -> Self {
                return Self {
                    a: f32::cos(radian) * radius,
                    b: f32::sin(radian) * radius
                };
            }
            pub fn from_complex(c: Complex) -> Self {
                return Self {
                    a: c.a,
                    b: c.b
                };
            }
        }

        // Prefabrication
        impl Complex {
            pub fn one() -> Self {
                return Self {
                    a: 1.0,
                    b: 1.0
                };
            }
            pub fn zero() -> Self {
                return Self {
                    a: 0.0,
                    b: 0.0
                };
            }
            pub fn right() -> Self {
                return Self {
                    a: 1.0,
                    b: 0.0
                };
            }
            pub fn left() -> Self {
                return Self {
                    a: -1.0,
                    b: 0.0
                };
            }
            pub fn up() -> Self {
                return Self {
                    a: 0.0,
                    b: 1.0
                };
            }
            pub fn down() -> Self {
                return Self {
                    a: 0.0,
                    b: -1.0
                };
            }
            pub fn QI() -> Self {
                return Self {
                    a: 1.0,
                    b: 1.0
                };
            }
            pub fn QIn() -> Self {
                return Self {
                    a: constants::SQRT2OVER2,
                    b: constants::SQRT2OVER2
                };
            }
            pub fn QII() -> Self {
                return Self {
                    a: -constants::SQRT2OVER2,
                    b: constants::SQRT2OVER2
                };
            }
            pub fn QIII() -> Self {
                return Self {
                    a: -constants::SQRT2OVER2,
                    b: -constants::SQRT2OVER2
                };
            }
            pub fn QIV() -> Self {
                return Self {
                    a: constants::SQRT2OVER2,
                    b: -constants::SQRT2OVER2
                };
            }
        }

        // Swizzling
        impl Complex {
            pub fn aa(&self) -> Self {
                return Self {
                    a: self.a,
                    b: self.a
                };
            }
            pub fn ab(&self) -> Self {
                return Self {
                    a: self.a,
                    b: self.b
                };
            }
            pub fn ba(&self) -> Self {
                return Self {
                    a: self.b, 
                    b: self.a
                };
            }
            pub fn bb(&self) -> Self {
                return Self {
                    a: self.b,
                    b: self.b
                };
            }
        }
        
        // Utilities
        impl Complex {
            pub fn to_string(&self) -> String {
                return format!("{} + {}i", self.a, self.b);
            }
            pub fn to_latex(&self) -> String {
                return format!("{} + {}i", self.a, self.b);
            }
        }
        
        // Arithmetic
        //      Addition
        impl Add<Complex> for Complex {
            type Output = Complex;
            fn add(self, c: Complex) -> Complex {
                return Self {
                    a: self.a + c.a,
                    b: self.b + c.b
                };
            }
        }
        impl AddAssign<Complex> for Complex {
            fn add_assign(&mut self, c: Complex) {
                self.a += c.a;
                self.b += c.b;
            }
        }
        //      Subtraction
        impl Sub<Complex> for Complex {
            type Output = Complex;
            fn sub(self, c: Complex) -> Complex {
                return Self {
                    a: self.a - c.a,
                    b: self.b - c.b
                };
            }
        }
        impl SubAssign<Complex> for Complex {
            fn sub_assign(&mut self, c: Complex) {
                self.a -= c.a;
                self.b -= c.b;
            }
        }
        impl Neg for Complex {
            type Output = Complex;
            fn neg(self) -> Complex {
                return Self {
                    a: -self.a,
                    b: -self.b
                };
            }
        }
        //      Multiplication
        impl Mul<f32> for Complex {
            type Output = Complex;
            fn mul(self, s: f32) -> Complex {
                return Self {
                    a: self.a * s,
                    b: self.b * s
                };
            }
        }
        impl MulAssign<f32> for Complex {
            fn mul_assign(&mut self, s: f32) {
                self.a *= s;
                self.b *= s;
            }
        }
        impl Mul<Complex> for Complex {
            type Output = Complex;
            fn mul(self, c: Complex) -> Complex {
                return Self {
                    a: (self.a * c.a - self.b * c.b), 
                    b: (self.a * c.b + self.b * c.a)
                };
            }
        }
        impl MulAssign<Complex> for Complex {
            fn mul_assign(&mut self, c: Complex) {
                self.a = (self.a * c.a - self.b * c.b);
                self.b = (self.a * c.b + self.b * c.a);
            }
        }
        //      Division
        impl Div<f32> for Complex {
            type Output = Complex;
            fn div(self, s: f32) -> Complex {
                let t = 1.0 / s;
                return Self {
                    a: self.a * t,
                    b: self.b * t
                };
            }
        }
        impl DivAssign<f32> for Complex {
            fn div_assign(&mut self, s: f32) {
                let t = 1.0 / s;
                self.a *= t;
                self.b *= t;
            }
        }
        impl Div<Complex> for Complex {
            type Output = Complex;
            fn div(self, c: Complex) -> Complex {
                return Self {
                    a: (self.a * c.a + self.b * c.b) / (c.a * c.a + c.b * c.b),
                    b: (self.b * c.a - self.a * c.b) / (c.a * c.a + c.b * c.b)
                }
            }
        }
        impl DivAssign<Complex> for Complex {
            fn div_assign(&mut self, c: Complex) {
                self.a = (self.a * c.a + self.b * c.b) / (c.a * c.a + c.b * c.b);
                self.b = (self.b * c.a - self.a * c.b) / (c.a * c.a + c.b * c.b);
            }
        }

        // Remaining arithmetic / operations
        impl Complex {
            //      Conjugate
            fn conjugate(&self) -> Complex {
                return Complex::new(
                    self.a, 
                    -self.b);
            }
            //      Inverse
            fn inverse(&self) -> Complex {
                let denom = 1.0 / self.mag_sqr();
                return Complex::new(
                    self.a * denom,
                    -self.b * denom
                );
            }
        }

        // Geometry
        impl Complex {
            //      Magnitude
            fn mag(&self) -> f32 {
                return f32::sqrt(self.a * self.a + self.b * self.b);
            }
            fn mag_sqr(&self) -> f32 {
                return self.a * self.a + self.b * self.b;
            }
            //      Rotation
            fn rotate(c: Complex, s: f32) -> Complex {
                let r = Complex::new(f32::cos(s), f32::sin(s));
                let c2 = c * r;
                return c2;
            }
        }
    }

    pub mod dual {
        // Guidance: http://www.dtecta.com/files/GDC13_vandenBergen_Gino_Math_Tut.pdf
        // Paper on Automatic Differentiation (AD): https://www.duo.uio.no/bitstream/handle/10852/41535/Kjelseth-Master.pdf?sequence=9 
        // Introduction to AD: https://alexey.radul.name/ideas/2013/introduction-to-automatic-differentiation/

        // Includes
        use super::super::num::constants;
        use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign };

        // Dual number form:            d = a + bε
        // Dual numbers base law:       ε^2 = 0
        #[derive(Debug)]
        pub struct Dual {
            a: f32,
            b: f32,
        }

        // Construction
        impl Dual {
            pub fn new(s: f32, t: f32) -> Self {
                return Self {
                    a: s,
                    b: t
                };
            }
            pub fn from_polar(radian: f32, radius: f32) -> Self {
                return Self {
                    a: f32::cos(radian) * radius,
                    b: f32::sin(radian) * radius
                };
            }
            pub fn from_dual(d: Dual) -> Self {
                return Self {
                    a: d.a,
                    b: d.b
                };
            }
        }

        // Prefabrication
        impl Dual {
            pub fn one() -> Self {
                return Self {
                    a: 1.0,
                    b: 1.0
                };
            }
            pub fn zero() -> Self {
                return Self {
                    a: 0.0,
                    b: 0.0
                };
            }
            pub fn right() -> Self {
                return Self {
                    a: 1.0,
                    b: 0.0
                };
            }
            pub fn left() -> Self {
                return Self {
                    a: -1.0,
                    b: 0.0
                };
            }
            pub fn up() -> Self {
                return Self {
                    a: 0.0,
                    b: 1.0
                };
            }
            pub fn down() -> Self {
                return Self {
                    a: 0.0,
                    b: -1.0
                };
            }
            pub fn QI() -> Self {
                return Self {
                    a: 1.0,
                    b: 1.0
                };
            }
            pub fn QIn() -> Self {
                return Self {
                    a: constants::SQRT2OVER2,
                    b: constants::SQRT2OVER2
                };
            }
            pub fn QII() -> Self {
                return Self {
                    a: -constants::SQRT2OVER2,
                    b: constants::SQRT2OVER2
                };
            }
            pub fn QIII() -> Self {
                return Self {
                    a: -constants::SQRT2OVER2,
                    b: -constants::SQRT2OVER2
                };
            }
            pub fn QIV() -> Self {
                return Self {
                    a: constants::SQRT2OVER2,
                    b: -constants::SQRT2OVER2
                };
            }
        }

        // Swizzling
        impl Dual {
            pub fn aa(&self) -> Self {
                return Self {
                    a: self.a,
                    b: self.a
                };
            }
            pub fn ab(&self) -> Self {
                return Self {
                    a: self.a,
                    b: self.b
                };
            }
            pub fn ba(&self) -> Self {
                return Self {
                    a: self.b, 
                    b: self.a
                };
            }
            pub fn bb(&self) -> Self {
                return Self {
                    a: self.b,
                    b: self.b
                };
            }
        }

        // Utilities
        impl Dual {
            pub fn to_string(&self) -> String {
                return format!("{} + {}ε", self.a, self.b);
            }
            pub fn to_latex(&self) -> String {
                return format!("{} + {}ε", self.a, self.b);
            }
        }

                // Arithmetic
        //      Addition
        impl Add<Dual> for Dual {
            type Output = Dual;
            fn add(self, d: Dual) -> Dual {
                return Self {
                    a: self.a + d.a,
                    b: self.b + d.b
                };
            }
        }
        impl AddAssign<Dual> for Dual {
            fn add_assign(&mut self, d: Dual) {
                self.a += d.a;
                self.b += d.b;
            }
        }
        //      Subtraction
        impl Sub<Dual> for Dual {
            type Output = Dual;
            fn sub(self, d: Dual) -> Dual {
                return Self {
                    a: self.a - d.a,
                    b: self.b - d.b
                };
            }
        }
        impl SubAssign<Dual> for Dual {
            fn sub_assign(&mut self, d: Dual) {
                self.a -= d.a;
                self.b -= d.b;
            }
        }
        impl Neg for Dual {
            type Output = Dual;
            fn neg(self) -> Dual {
                return Self {
                    a: -self.a,
                    b: -self.b
                };
            }
        }
        //      Multiplication
        impl Mul<f32> for Dual {
            type Output = Dual;
            fn mul(self, s: f32) -> Dual {
                return Self {
                    a: self.a * s,
                    b: self.b * s
                };
            }
        }
        impl MulAssign<f32> for Dual {
            fn mul_assign(&mut self, s: f32) {
                self.a *= s;
                self.b *= s;
            }
        }
        impl Mul<Dual> for Dual {
            type Output = Dual;
            fn mul(self, d: Dual) -> Dual {
                return Self {
                    a: (self.a * d.a), 
                    b: (self.a * d.b + self.b * d.a)
                };
            }
        }
        impl MulAssign<Dual> for Dual {
            fn mul_assign(&mut self, d: Dual) {
                self.a = (self.a * d.a);
                self.b = (self.a * d.b + self.b * d.a);
            }
        }
        //      Division
        impl Div<f32> for Dual {
            type Output = Dual;
            fn div(self, s: f32) -> Dual {
                let t = 1.0 / s;
                return Self {
                    a: self.a * t,
                    b: self.b * t
                };
            }
        }
        impl DivAssign<f32> for Dual {
            fn div_assign(&mut self, s: f32) {
                let t = 1.0 / s;
                self.a *= t;
                self.b *= t;
            }
        }
        impl Div<Dual> for Dual {
            type Output = Dual;
            fn div(self, d: Dual) -> Dual {
                let denom = 1.0 / (d.a * d.a);
                return Self {
                    a: (self.a * d.a) * denom,
                    b: (self.b * d.a - self.a * d.b) * denom
                }
            }
        }
        impl DivAssign<Dual> for Dual {
            fn div_assign(&mut self, d: Dual) {
                let denom = 1.0 / (d.a * d.a);
                self.a = (self.a * d.a) * denom;
                self.b = (self.b * d.a - self.a * d.b) * denom;
            }
        }

        // Remaining arithmetic / operations
        impl Dual {
            //      Conjugate
            fn conjugate(&self) -> Dual {
                return Dual::new(
                    self.a,
                    -self.b
                );
            }
            //      Inverse
            //  (Does not exist)
        }

        // Geometry
        impl Dual {
            fn mag(&self) -> f32 {
                return self.a;
            }
        }
    }
}

pub mod linalg {

    
    pub trait LinearAlgebra {
        // N.B.: construction, prefabrication, and swizzling 
        // aren't suitable to implement as traits. 
        // Implement these explicitly instead. 

        // Utilities
        fn to_string(&self)     -> String;
        fn to_latex(&self)      -> String;
    }

    pub mod vector {
        pub use super::super::num::constants as num;
        pub use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, BitXor };

        // Struct definitions
        #[derive(Debug, Copy, Clone)]
        pub struct Vector2 { x: f32, y: f32 }
        #[derive(Debug, Copy, Clone)]
        pub struct Vector3 { x: f32, y: f32, z: f32 }
        #[derive(Debug, Copy, Clone)]
        pub struct Vector4 { x: f32, y: f32, z: f32, w: f32 }

        // Construction
        impl Vector2 {
            pub fn new(a: f32, b: f32)                      -> Self { return Self { x: a, y: b }; }
            pub fn from_polar(radian: f32, radius: f32)     -> Self { return Vector2::new(f32::cos(radian) * radius, f32::sin(radian) * radius); }
            pub fn from_vector2(v: &Vector2)                -> Self { return Vector2::new(v.x, v.y); }
            pub fn from_vector3(v: &Vector3)                -> Self { return Vector2::new(v.x, v.y); }
            pub fn from_vector4(v: &Vector4)                -> Self { return Vector2::new(v.x, v.y); }
        }
        impl Vector3 {
            pub fn new(a: f32, b: f32, c: f32)              -> Self { return Self { x: a, y: b, z: c }; }
            pub fn from_spherical(r: f32, t: f32, p: f32)   -> Self { return Vector3::new(r * f32::sin(p) * f32::cos(t), r * f32::sin(p) * f32::sin(t), r * f32::cos(p)); }
            pub fn from_vector2(v: &Vector2, z: f32)        -> Self { return Vector3::new(v.x, v.y, z); }
            pub fn from_vector3(v: &Vector3)                -> Self { return Vector3::new(v.x, v.y, v.z); }
            pub fn from_vector4(v: &Vector4)                -> Self { return Vector3::new(v.x, v.y, v.z); }
        }
        impl Vector4 {
            pub fn new(a: f32, b: f32, c: f32, d: f32)      -> Self { return Self { x: a, y: b, z: c, w: d }; }
            pub fn from_vector2(v: &Vector2, z: f32, w: f32)-> Self { return Vector4::new(v.x, v.y, z, w); }
            pub fn from_vector3(v: &Vector3, w: f32)        -> Self { return Vector4::new(v.x, v.y, v.z, w); }
            pub fn from_vector4(v: &Vector4)                -> Self { return Vector4::new(v.x, v.y, v.z, v.w); }
        }
        
        // Conversion methods
        impl Vector2 {
            pub fn to_vector3(&self)        -> Vector3 { return Vector3::new(self.x, self.y, 0.0); }
            pub fn to_vector4(&self)        -> Vector4 { return Vector4::new(self.x, self.y, 0.0, 0.0); }
        }
        impl Vector3 {
            pub fn to_vector2(&self)        -> Vector2 { return Vector2::new(self.x, self.y); }
            pub fn to_vector4(&self)        -> Vector4 { return Vector4::new(self.x, self.y, self.z, 0.0); }
        }
        impl Vector4 {
            pub fn to_vector2(&self)        -> Vector2 { return Vector2::new(self.x, self.y); }
            pub fn to_vector3(&self)        -> Vector3 { return Vector3::new(self.x, self.y, self.z); }
        }

        // Prefabrication
        impl Vector2 {
            pub fn one()    -> Self { return Vector2::new(1.0, 1.0); }
            pub fn zero()   -> Self { return Vector2::new(0.0, 0.0); }

            pub fn right()  -> Self { return Vector2::new(1.0, 0.0); }
            pub fn left()   -> Self { return Vector2::new(-1.0, 0.0); }
            pub fn forth()  -> Self { return Vector2::new(0.0, 1.0); }
            pub fn back()   -> Self { return Vector2::new(0.0, -1.0); }

            pub fn Q1()     -> Self { return Vector2::new(1.0, 1.0); }
            pub fn Q1n()    -> Self { return Vector2::new(num::SQRT2OVER2, num::SQRT2OVER2); }
            pub fn Q2()     -> Self { return Vector2::new(-1.0, 1.0); }
            pub fn Q2n()    -> Self { return Vector2::new(-num::SQRT2OVER2, num::SQRT2OVER2); }
            pub fn Q3()     -> Self { return Vector2::new(-1.0, -1.0); }
            pub fn Q3n()    -> Self { return Vector2::new(-num::SQRT2OVER2, -num::SQRT2OVER2); }
            pub fn Q4()     -> Self { return Vector2::new(1.0, -1.0); }
            pub fn Q4n()    -> Self { return Vector2::new(num::SQRT2OVER2, -num::SQRT2OVER2); }
        }
        impl Vector3 {
            pub fn one()    -> Self { return Vector3::new(1.0, 1.0, 1.0); }
            pub fn zero()   -> Self { return Vector3::new(0.0, 0.0, 0.0); }

            pub fn right()  -> Self { return Vector3::new(1.0, 0.0, 0.0); }
            pub fn left()   -> Self { return Vector3::new(-1.0, 0.0, 0.0); }
            pub fn forth()  -> Self { return Vector3::new(0.0, 1.0, 0.0); }
            pub fn back()   -> Self { return Vector3::new(0.0, -1.0, 0.0); }
            pub fn up()     -> Self { return Vector3::new(0.0, 0.0, 1.0); }
            pub fn down()   -> Self { return Vector3::new(0.0, 0.0, -1.0); }

            pub fn Q1()     -> Self { return Vector3::new(1.0, 1.0, 1.0); }
            pub fn Q1n()    -> Self { return Vector3::new(num::SQRT3OVER3, num::SQRT3OVER3, num::SQRT3OVER3); }
            pub fn Q2()     -> Self { return Vector3::new(-1.0, 1.0, 1.0); }
            pub fn Q2n()    -> Self { return Vector3::new(-num::SQRT3OVER3, num::SQRT3OVER3, num::SQRT3OVER3); }
            pub fn Q3()     -> Self { return Vector3::new(-1.0, -1.0, 1.0); }
            pub fn Q3n()    -> Self { return Vector3::new(-num::SQRT3OVER3, -num::SQRT3OVER3, num::SQRT3OVER3); }
            pub fn Q4()     -> Self { return Vector3::new(1.0, -1.0, 1.0); }
            pub fn Q4n()    -> Self { return Vector3::new(num::SQRT3OVER3, -num::SQRT3OVER3, num::SQRT3OVER3); }
            pub fn Q5()     -> Self { return Vector3::new(1.0, 1.0, -1.0); }
            pub fn Q5n()    -> Self { return Vector3::new(num::SQRT3OVER3, num::SQRT3OVER3, -num::SQRT3OVER3); }
            pub fn Q6()     -> Self { return Vector3::new(-1.0, 1.0, -1.0); }
            pub fn Q6n()    -> Self { return Vector3::new(-num::SQRT3OVER3, num::SQRT3OVER3, -num::SQRT3OVER3); }
            pub fn Q7()     -> Self { return Vector3::new(-1.0, -1.0, -1.0); }
            pub fn Q7n()    -> Self { return Vector3::new(-num::SQRT3OVER3, -num::SQRT3OVER3, -num::SQRT3OVER3); }
            pub fn Q8()     -> Self { return Vector3::new(1.0, -1.0, -1.0); }
            pub fn Q8n()    -> Self { return Vector3::new(num::SQRT3OVER3, -num::SQRT3OVER3, -num::SQRT3OVER3); }
        }
        impl Vector4 {
            pub fn one()    -> Self { return Vector4::new(1.0, 1.0, 1.0, 1.0); }
            pub fn zero()   -> Self { return Vector4::new(0.0, 0.0, 0.0, 0.0); }
            // Add more prefabs? 
        }
        
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
        impl Vector2 {
            pub fn to_string(&self)     -> String { return format!("[{}, {}]", self.x, self.y); }
            pub fn to_latex(&self)      -> String { return format!("[{}, {}]", self.x, self.y); }
        }
        impl Vector3 {
            pub fn to_string(&self)     -> String { return format!("[{}, {}, {}]", self.x, self.y, self.z); }
            pub fn to_latex(&self)      -> String { return format!("[{}, {}, {}]", self.x, self.y, self.z); }
        }
        impl Vector4 {
            pub fn to_string(&self)     -> String { return format!("[{}, {}, {}, {}]", self.x, self.y, self.z, self.w); }
            pub fn to_latex(&self)      -> String { return format!("[{}, {}, {}, {}]", self.x, self.y, self.z, self.w); }
        }

        // Vector arithmetic
        //      Addition: { v + v, v += v }
        // Keep the example below - it might come of use regarding ownership (copy trait, etc.). 
        // This example should apply to most basic arithmetic operations (except assigns, unary operations, etc.). 
        // impl Add<&Vector2> for &Vector2 { type Output = Vector2; fn add(self, v: &Vector2) -> Vector2 { 
        //     return Vector2::new(self.x + v.x, self.y + v.y); } }
        impl Add<Vector2> for Vector2 { type Output = Self; fn add(self, v: Self) -> Self {
            return Self::new(self.x + v.x, self.y + v.y); } }
        impl Add<Vector3> for Vector3 { type Output = Self; fn add(self, v: Self) -> Self {
            return Self::new(self.x + v.x, self.y + v.y, self.z + v.z); } }
        impl Add<Vector4> for Vector4 { type Output = Self; fn add(self, v: Self) -> Self {
            return Self::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w); } }
        impl AddAssign<Vector2> for Vector2 { fn add_assign(&mut self, v: Self) { 
            self.x += v.x; self.y += v.y; } }
        impl AddAssign<Vector3> for Vector3 { fn add_assign(&mut self, v: Self) { 
            self.x += v.x; self.y += v.y; self.z += v.z; } }
        impl AddAssign<Vector4> for Vector4 { fn add_assign(&mut self, v: Self) { 
            self.x += v.x; self.y += v.y; self.z += v.z; self.w += v.w; } }
        //      Subtraction: { v - v, v -= v, -v }
        impl Sub<Vector2> for Vector2 { type Output = Self; fn sub(self, v: Self) -> Self { 
            return Self::new(self.x - v.x, self.y - v.y); } }
        impl Sub<Vector3> for Vector3 { type Output = Self; fn sub(self, v: Self) -> Self { 
            return Self::new(self.x - v.x, self.y - v.y, self.z - v.z); } }
        impl Sub<Vector4> for Vector4 { type Output = Self; fn sub(self, v: Self) -> Self { 
            return Self::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w); } }
        impl SubAssign<Vector2> for Vector2 { fn sub_assign(&mut self, v: Self) { 
            self.x -= v.x; self.y -= v.y; } }
        impl SubAssign<Vector3> for Vector3 { fn sub_assign(&mut self, v: Self) { 
            self.x -= v.x; self.y -= v.y; self.z -= v.z; } }
        impl SubAssign<Vector4> for Vector4 { fn sub_assign(&mut self, v: Self) { 
            self.x -= v.x; self.y -= v.y; self.z -= v.z; self.w -= v.w; } }
        impl Neg for Vector2 { type Output = Vector2; fn neg(self) -> Vector2 { 
            return Self::new(-self.x, -self.y); } }
        impl Neg for Vector3 { type Output = Self; fn neg(self) -> Self { 
            return Self::new(-self.x, -self.y, -self.z); } }
        impl Neg for Vector4 { type Output = Self; fn neg(self) -> Self { 
            return Self::new(-self.x, -self.y, -self.z, -self.w); } }
        //      Scalar-vector multiplication: { v * s, v *= s, s * v }
        impl Mul<f32> for Vector2 { type Output = Self; fn mul(self, s: f32) -> Self { 
            return Self::new(self.x * s, self.y * s); } }
        impl Mul<f32> for Vector3 { type Output = Self; fn mul(self, s: f32) -> Self { 
            return Self::new(self.x * s, self.y * s, self.z * s); } }
        impl Mul<f32> for Vector4 { type Output = Self; fn mul(self, s: f32) -> Self { 
            return Self::new(self.x * s, self.y * s, self.z * s, self.w * s); } }
        impl MulAssign<f32> for Vector2 { fn mul_assign(&mut self, s: f32) { 
            self.x *= s; self.y *= s; } }
        impl MulAssign<f32> for Vector3 { fn mul_assign(&mut self, s: f32) { 
            self.x *= s; self.y *= s; self.z *= s; } }
        impl MulAssign<f32> for Vector4 { fn mul_assign(&mut self, s: f32) { 
            self.x *= s; self.y *= s; self.z *= s; self.w *= s; } }
        impl Mul<Vector2> for f32 { type Output = Vector2; fn mul(self, v: Vector2) -> Vector2 {
            return Vector2::new(v.x * self, v.y * self); } }
        impl Mul<Vector3> for f32 { type Output = Vector3; fn mul(self, v: Vector3) -> Vector3 {
            return Vector3::new(v.x * self, v.y * self, v.z * self); } }
        impl Mul<Vector4> for f32 { type Output = Vector4; fn mul(self, v: Vector4) -> Vector4 {
            return Vector4::new(v.x * self, v.y * self, v.z * self, v.w * self); } }
        //      Vector-vector multiplication: { scalar (dot "*"), vector (cross "/"; z-value only), geometric (wedge "^"), triple products }
        impl Mul<Vector2> for Vector2 { type Output = f32; fn mul(self, v: Self) -> f32 {
            return self.x * v.x + self.y * v.y; }}
        impl Mul<Vector3> for Vector3 { type Output = f32; fn mul(self, v: Self) -> f32 {
            return self.x * v.x + self.y * v.y + self.z * v.z; }}
        impl Mul<Vector4> for Vector4 { type Output = f32; fn mul(self, v: Self) -> f32 {
            return self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w; }}
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
            let t = 1.0 / s; return Self::new(self.x * t, self.y * t); } }
        impl Div<f32> for Vector3 { type Output = Self; fn div(self, s: f32) -> Self { 
            let t = 1.0 / s; return Self::new(self.x * t, self.y * t, self.z * t); } }
        impl Div<f32> for Vector4 { type Output = Self; fn div(self, s: f32) -> Self { 
            let t = 1.0 / s; return Self::new(self.x * t, self.y * t, self.z * t, self.w * t); } }
        impl DivAssign<f32> for Vector2 { fn div_assign(&mut self, s: f32) { 
            let t = 1.0 / s; self.x *= t; self.y *= t; } }
        impl DivAssign<f32> for Vector3 { fn div_assign(&mut self, s: f32) { 
            let t = 1.0 / s; self.x *= t; self.y *= t; self.z *= t; } }
        impl DivAssign<f32> for Vector4 { fn div_assign(&mut self, s: f32) { 
            let t = 1.0 / s; self.x *= t; self.y *= t; self.z *= t; self.w *= t; } }

        // Geometry
        impl Vector2 {
            //      Magnitude
            pub fn mag(&self)               -> f32 { return f32::sqrt(self.x * self.x + self.y * self.y); }
            pub fn mag_sqr(&self)           -> f32 { return self.x * self.x + self.y * self.y; }
            pub fn normalization(&self)     -> Self { let d = 1.0 / self.mag(); return Self::new(self.x * d, self.y * d); }
            pub fn normalize(&mut self)     { let d = 1.0 / self.mag(); self.x *= d; self.y *= d; }

            //      Interpolation
            pub fn lerp(a: &Self, b: &Self, t: f32)   -> Self { return Self::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
            // fn slerp(a: &Self, b: &Self, s: f32)   -> Self { /* unfinished */ }

            //      Measurement (angles in radians)
            pub fn angle(a: &Self, b: &Self)                -> f32 { return f32::acos(((*a) * (*b)) / (a.mag() * b.mag())); }
            pub fn angle_safe(a: &Self, b: &Self)           -> f32 { 
                let d = a.mag() * b.mag();
                if d <= std::f32::EPSILON {
                    return std::f32::NAN;
                } else {
                    return f32::acos(((*a) * (*b)) / d);
                }
            }
            pub fn angle_unit(a: &Self, b: &Self)           -> f32 { return f32::acos((*a) * (*b)); }
            // Taken from https://stackoverflow.com/questions/14066933/direct-way-of-computing-clockwise-angle-between-2-vectors
            pub fn angle_signed(a: &Self, b: &Self)   -> f32 {
                let dot = ((*a) * (*b));
                let det = ((*a) / (*b));
                let mut angle = f32::atan2(det, dot);
                if angle < 0.0 {
                    angle += num::TAU;
                }
                return angle;
            }
            //  Is there no angle_signed_unit()?
            // fn angle_signed_unit(a: &Vector2, b: &Vector2) -> f32 {
            // }

            //      Vector projection
            pub fn projection(a: &Self, b: &Self) -> Self {
                return (*b) * (((*a) * (*b)) / ((*b) * (*b)));
            }
            pub fn projection_unit(a: &Self, b: &Self) -> Self {
                return (*b) * ((*a) * (*b));
            }
            
            pub fn rejection(a: &Self, b: &Self) -> Self {
                return (*a) - Self::projection(a, b);
            }
            pub fn rejection_unit(a: &Self, b: &Self) -> Self {
                return (*a) - Self::projection_unit(a, b);
            }

            pub fn reflection(a: &Self, b: &Self) -> Self {
                return (*a) - 2.0 * Self::projection(a, b);
            }
            pub fn reflection_unit(a:&Self, b: &Self) -> Self {
                return (*a) - 2.0 * Self::projection_unit(a, b);
            }

            pub fn refraction(a: &Self, b: &Self, n1: f32, n2: f32) -> Self {
                let mag = a.mag();
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
                let arg = t3 + num::PI + t2;

                return Self::from_polar(arg, mag);
            }
            pub fn refraction_unit(a: &Self, b: &Self, n1: f32, n2: f32) -> Self {
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
                let arg = t3 + num::PI + t2;

                return Self::from_polar(arg, 1.0);
            }
        }
        impl Vector3 {
            // Magnitude
            pub fn mag(&self)               -> f32 { return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z); }
            pub fn mag_sqr(&self)           -> f32 { return self.x * self.x + self.y * self.y + self.z * self.z; }
            pub fn normalization(&self)     -> Self { let d = 1.0 / self.mag(); return Self::new(self.x * d, self.y * d, self.z * d); }
            pub fn normalize(&mut self)     { let d = 1.0 / self.mag(); self.x *= d; self.y *= d; self.z *= d; }

            // Interpolation
            pub fn lerp(a: &Self, b: &Self, t: f32)     -> Self { return Self::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t); }
            // pub fn slerp(a: &Self, b: &Self, t: f32)    -> Self {}

            // Measurement (angles in radians)
            pub fn angle(a: &Self, b: &Self)            -> f32 { return f32::acos(((*a) * (*b)) / (a.mag() * b.mag())); }
            pub fn angle_safe(a: &Self, b: &Self)       -> f32 { 
                let d = a.mag() * b.mag();
                if d <= std::f32::EPSILON {
                    return std::f32::NAN;
                } else {
                    return f32::acos(((*a) * (*b)) / d);
                }
            }
            pub fn angle_unit(a: &Self, b: &Self)       -> f32 { return f32::acos((*a) * (*b)); }
            // is there no angle_signed in 3D?

            // Vector projection
        }
        impl Vector4 {

        }
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

    pub mod matrix {
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

    pub mod quaternion {
        // Includes
        use super::super::num::constants;
        use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign };

        // Quaternion number form:      q = s + xi + yj + zk -> { s, x, y, z } ∈ R
        // Quaternion base law:         i^2 + j^2 + k^2 = ijk = (-1)
        /*
                                        ij = k      jk = i      ki = j
                                        ji = -k     kj = -i     ik = -j
        */
        // #[derive(Debug)]
        // pub struct Quaternion {

        // }
    }
}