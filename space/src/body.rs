use bevy::math::DVec3;
use bnum::cast::As;
use bnum::types::U256;
use num_integer::Roots;
pub type UInt = U256;
pub const PRECISION: UInt = UInt::TEN.pow(6);
const PREC_F64: f64 = 10u32.pow(6) as _;
const PREC_SQRT: UInt = UInt::TEN.pow(3);
const GRAV_INT: UInt = UInt::parse_str_radix("66743", 10); // 6.6743015e-14 * PRECISION^3
const TAU_INT: UInt = UInt::parse_str_radix("6283185", 10);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit {
    /// Direction of second focus from the star, must be a unit vector or 0
    pub focus: DVec3,
    /// Measure of semimajor axis of orbit, in kilometers
    pub semimajor: UInt,
    /// Eccentricity of orbit
    pub eccentricity: f64,
}
impl Orbit {
    pub const EARTH: Self = Self {
        focus: DVec3::X,
        semimajor: UInt::parse_str_radix("149598000", 10),
        eccentricity: 0.017,
    };
    pub const MOON: Self = Self {
        focus: DVec3::X,
        semimajor: UInt::parse_str_radix("384400", 10),
        eccentricity: 0.0549,
    };
    pub fn circular(semimajor: UInt, focus: DVec3) -> Self {
        Self {
            focus,
            semimajor,
            eccentricity: 0.0,
        }
    }
    /// Maximum distence from sun, in kilometers
    #[inline(always)]
    pub fn aphelion(self) -> UInt {
        self.semimajor * ((1.0 + self.eccentricity) * PREC_F64).as_::<UInt>() / PRECISION
    }
    /// Minimum distance from sun, in kilometers
    #[inline(always)]
    pub fn perihelion(self) -> UInt {
        self.semimajor * ((1.0 - self.eccentricity) * PREC_F64).as_::<UInt>() / PRECISION
    }
    /// Finds the distance of the planet from the sun in kilometers, with the angle being measured from the perihelion in radians
    pub fn orbital_distance(self, theta: f64) -> UInt {
        (((1.0 - self.eccentricity.powi(2)) / (1.0 + self.eccentricity * theta.cos()) * PREC_F64)
            as u128)
            .as_::<UInt>()
            * self.semimajor
            / PRECISION
    }
    /// Orbital period in seconds, mass in kilograms
    pub fn orbital_period(self, mass: UInt) -> UInt {
        TAU_INT * (self.semimajor.pow(3) * PRECISION.pow(3) / (GRAV_INT * mass)).sqrt() / PRECISION
    }
    /// Speed of the body in km/s, mass of the parent in kilograms, angle in radians
    pub fn orbital_speed(self, mass: UInt, theta: f64) -> UInt {
        (GRAV_INT
            * mass
            * (UInt::TWO * PRECISION / self.orbital_distance(theta) - PRECISION / self.semimajor))
            .sqrt()
            / PREC_SQRT
            / PRECISION
    }
    /// Predict the position of an object after a given number of milliseconds.
    /// Current and resulting angles are measured in radians from perihelion. Mass of the star is given in kilograms.
    #[allow(unused_variables)]
    pub fn predict(self, mass: UInt, current: f64, time: UInt) -> f64 {
        unimplemented!("orbital predictions aren't implemented yet")
    }
}
