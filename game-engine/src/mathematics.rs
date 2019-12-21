// Math library for the game engine, rendering engine, and physics engine
use std::f32::{ self, EPSILON, NAN };

/*
    To do: 
    - Consider whether to keep member-altering functions like normalize(), div_assign(), etc. 
    - Maybe rather make it a conscious decision to do those types of operations instead? 
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
    pub mod vector {
        use super::super::num::constants as num;
        use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, BitXor };

        
        #[derive(Debug, Copy, Clone)]
        pub struct Vector2 { x: f32, y: f32 }

        // Construction
        impl Vector2 {
            pub fn new(a: f32, b: f32)                      -> Self { return Self { x: a, y: b}; }
            pub fn from_polar(radian: f32, radius: f32)     -> Self { return Vector2::new(f32::cos(radian) * radius, f32::sin(radian) * radius); }
            pub fn from_vector2(v: &Vector2)                -> Self { return Vector2::new(v.x, v.y); }
            pub fn from_vector3(v: &Vector3)                -> Self { return Vector2::new(v.x, v.y); }
            pub fn from_vector4(v: &Vector4)                -> Self { return Vector2::new(v.x, v.y); }
        }
        
        // Prefabrication
        impl Vector2 {
            pub fn one()    -> Self { return Vector2::new(1.0, 1.0); }
            pub fn zero()   -> Self { return Vector2::new(0.0, 0.0); }
            pub fn right()  -> Self { return Vector2::new(1.0, 0.0); }
            pub fn left()   -> Self { return Vector2::new(-1.0, 0.0); }
            pub fn up()     -> Self { return Vector2::new(0.0, 1.0); }
            pub fn down()   -> Self { return Vector2::new(0.0, -1.0); }
            pub fn QI()     -> Self { return Vector2::new(1.0, 1.0); }
            pub fn QIn()    -> Self { return Vector2::new(num::SQRT2OVER2, num::SQRT2OVER2); }
            pub fn QII()    -> Self { return Vector2::new(-1.0, 1.0); }
            pub fn QIIn()   -> Self { return Vector2::new(-num::SQRT2OVER2, num::SQRT2OVER2); }
            pub fn QIII()   -> Self { return Vector2::new(-1.0, -1.0); }
            pub fn QIIIn()  -> Self { return Vector2::new(-num::SQRT2OVER2, -num::SQRT2OVER2); }
            pub fn QIV()    -> Self { return Vector2::new(1.0, -1.0); }
            pub fn QIVn()   -> Self { return Vector2::new(num::SQRT2OVER2, -num::SQRT2OVER2); }
        }
        
        // Swizzling
        impl Vector2 {
            pub fn xx(&self)    -> Self { return Vector2::new(self.x, self.x); }
            pub fn xy(&self)    -> Self { return Vector2::new(self.x, self.y); }
            pub fn yx(&self)    -> Self { return Vector2::new(self.y, self.x); }
            pub fn yy(&self)    -> Self { return Vector2::new(self.y, self.y); }
        }
        
        // Utilities
        impl Vector2 {
            pub fn to_string(&self)     -> String { return format!("[{}, {}]", self.x, self.y); }
            pub fn to_latex(&self)      -> String { return format!("[{}, {}]", self.x, self.y); }
        }

        // Vector arithmetic
        //      Addition: { v + v, v += v }
        impl Add<Vector2> for Vector2 { type Output = Vector2; fn add(self, v: Vector2) -> Vector2 {
            return Vector2::new(self.x + v.x, self.y + v.y); } }
        // Keep the example below - it might come of use regarding ownership (copy trait, etc.). 
        // This example should apply to most basic arithmetic operations (except assigns, unary operations, etc.). 
        // impl Add<&Vector2> for &Vector2 { type Output = Vector2; fn add(self, v: &Vector2) -> Vector2 { 
        //     return Vector2::new(self.x + v.x, self.y + v.y); } }
        impl AddAssign<Vector2> for Vector2 { fn add_assign(&mut self, v: Vector2) { 
            self.x += v.x; self.y += v.y; } }
        //      Subtraction: { v - v, v -= v, -v }
        impl Sub<Vector2> for Vector2 { type Output = Vector2; fn sub(self, v: Vector2) -> Vector2 { 
            return Vector2::new(self.x - v.x, self.y - v.y); } }
        impl SubAssign<Vector2> for Vector2 { fn sub_assign(&mut self, v: Vector2) { 
            self.x -= v.x; self.y -= v.y; } }
        impl Neg for Vector2 { type Output = Vector2; fn neg(self) -> Vector2 { 
            return Vector2::new(-self.x, -self.y); } }
        //      Scalar-vector multiplication: { v * s, v *= s, s * v }
        impl Mul<f32> for Vector2 { type Output = Vector2; fn mul(self, s: f32) -> Vector2 { 
            return Vector2::new(self.x * s, self.y * s); } }
        impl MulAssign<f32> for Vector2 { fn mul_assign(&mut self, s: f32) { 
            self.x *= s; self.y *= s; } }
        impl Mul<Vector2> for f32 { type Output = Vector2; fn mul(self, v: Vector2) -> Vector2 {
            return Vector2::new(v.x * self, v.y * self); } }
        //      Vector-vector multiplication: { scalar (dot "*"), vector (cross "/"; z-value only), geometric (wedge "^"), triple products }
        impl Mul<Vector2> for Vector2 { type Output = f32; fn mul(self, v: Vector2) -> f32 {
            return self.x * v.x + self.y * v.y; }}
        impl Vector2 { fn product_scalar(a: Vector2, b: Vector2) -> f32 { 
            return a.x * b.x + a.y * b.y; } }
        impl Div<Vector2> for Vector2 { type Output = f32; fn div(self, v: Vector2) -> f32 {
            return self.x * v.y - self.y * v.x; }}
        impl Vector2 { fn product_vector(a: &Vector2, b: &Vector2) -> f32 { 
            return a.x * b.y - a.y * b.x; } }
        impl BitXor<Vector2> for Vector2 { type Output = Vector2 /*Bivector2*/; fn bitxor(self, v: Vector2) -> Vector2 /*Bivector2*/ {
            return Vector2::new(0.0, 0.0); /* This is just filler */ }}
        impl Vector2 {
            // fn product_geometric(a: &Vector2, b: &Vector2) -> Bivector2 {
            //     return ()
            // }
        }
        impl Vector2 {
            //      Scalar triple product: https://en.wikipedia.org/wiki/Triple_product
            //  (Makes no sense in 2D)
            // }
            //      Vector triple product: https://en.wikipedia.org/wiki/Triple_product 
            //  (Makes no sense in 2D)
        }

        //      Division
        impl Div<f32> for Vector2 { type Output = Vector2; fn div(self, s: f32) -> Vector2 { 
            let t = 1.0 / s; return Vector2::new(self.x * t, self.y * t); } }
        impl DivAssign<f32> for Vector2 { fn div_assign(&mut self, s: f32) { 
            let t = 1.0 / s; self.x *= t; self.y *= t; } }

        // Geometry
        impl Vector2 {
            //      Magnitude
            pub fn mag(&self)               -> f32 { return f32::sqrt(self.x * self.x + self.y * self.y); }
            pub fn mag_sqr(&self)           -> f32 { return self.x * self.x + self.y * self.y; }
            pub fn normalization(&self)     -> Vector2 { let denom = 1.0 / self.mag(); return Vector2::new(self.x * denom, self.y * denom); }
            pub fn normalize(&mut self)     { let denom = 1.0 / self.mag(); self.x *= denom; self.y *= denom; }

            //      Interpolation
            pub fn lerp(a: &Vector2, b: &Vector2, s: f32)   -> Vector2 { return Vector2::new(a.x + (b.x - a.x) * s, a.y + (b.y - a.y) * s); }
            // fn slerp(a: &Vector2, b: &Vector2, s: f32)   -> Vector2 { /* unfinished */ }

            //      Measurement (angles in radians)
            pub fn angle(a: &Vector2, b: &Vector2)          -> f32 { return f32::acos(((*a) * (*b)) / (a.mag() * b.mag())); }
            pub fn angle_safe(a: &Vector2, b: &Vector2)     -> f32 { 
                let denom = a.mag() * b.mag();
                if denom <= std::f32::EPSILON {
                    return std::f32::NAN;
                } else {
                    return f32::acos(((*a) * (*b)) / denom);
                }
            }
            pub fn angle_unit(a: &Vector2, b: &Vector2)     -> f32 { return f32::acos((*a) * (*b)); }
            // Taken from https://stackoverflow.com/questions/14066933/direct-way-of-computing-clockwise-angle-between-2-vectors
            pub fn angle_signed(a: &Vector2, b: &Vector2)   -> f32 {
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
            pub fn projection(a: &Vector2, b: &Vector2) -> Vector2 {
                return (*b) * (((*a) * (*b)) / ((*b) * (*b)));
            }
            pub fn projection_unit(a: &Vector2, b: &Vector2) -> Vector2 {
                return (*b) * ((*a) * (*b));
            }
            
            pub fn rejection(a: &Vector2, b: &Vector2) -> Vector2 {
                return (*a) - Vector2::projection(a, b);
            }
            pub fn rejection_unit(a: &Vector2, b: &Vector2) -> Vector2 {
                return (*a) - Vector2::projection_unit(a, b);
            }

            pub fn reflection(a: &Vector2, b: &Vector2) -> Vector2 {
                return (*a) - 2.0 * Vector2::projection(a, b);
            }
            pub fn reflection_unit(a:&Vector2, b: &Vector2) -> Vector2 {
                return (*a) - 2.0 * Vector2::projection_unit(a, b);
            }

            pub fn refraction(a: &Vector2, b: &Vector2, n1: f32, n2: f32) -> Vector2 {
                let mag = a.mag();
                let n = n1 / n2;
                let t1 = Vector2::angle(&(-(*a)), b);
                let mut signum = f32::cos(Vector2::angle(a, &Vector2::right()));
                if signum > 0.0 {
                    signum = 1.0;
                } else {
                    signum = (-1.0);
                }
                let t2 = f32::asin(n * f32::sin(t1)) * signum;
                let t3 = Vector2::angle(b, &Vector2::right());
                let arg = t3 + num::PI + t2;

                return Vector2::from_polar(arg, mag);
            }
            pub fn refraction_unit(a: &Vector2, b: &Vector2, n1: f32, n2: f32) -> Vector2 {
                let n = n1 / n2;
                let t1 = Vector2::angle_unit(&(-(*a)), b);
                let mut signum = f32::cos(Vector2::angle_unit(a, &Vector2::right()));
                if signum > 0.0 {
                    signum = 1.0;
                } else {
                    signum = (-1.0);
                }
                let t2 = f32::asin(n * f32::sin(t1)) * signum;
                let t3 = Vector2::angle(b, &Vector2::right());
                let arg = t3 + num::PI + t2;

                return Vector2::from_polar(arg, 1.0);
            }
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