use bevy::math::DVec3;
use cosmonaut_common::*;
use num_integer::Roots;
use std::f64::consts::TAU;

const PREC_SQRT: UInt = UInt::TEN.pow(3);
const GRAV_INT: UInt = UInt::parse_str_radix("66743", 10); // 6.6743015e-14 * PRECISION^3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit {
    /// Direction of second focus from the star, must be a unit vector or 0
    pub focus: DVec3,
    /// Starting angle at universe time = 0
    pub start: f64,
    /// Measure of semimajor axis of orbit, in kilometers
    pub semimajor: UInt,
    /// Eccentricity of orbit
    pub eccentricity: f64,
}
impl Orbit {
    pub const EARTH: Self = Self {
        focus: DVec3::X,
        start: 0.0,
        semimajor: UInt::parse_str_radix("149598000", 10),
        eccentricity: 0.017,
    };
    pub const MOON: Self = Self {
        focus: DVec3::X,
        start: 0.0,
        semimajor: UInt::parse_str_radix("384400", 10),
        eccentricity: 0.0549,
    };
    pub fn circular(semimajor: UInt, focus: DVec3, start: f64) -> Self {
        Self {
            focus,
            start,
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
    /// Orbital period in seconds, mass in 10^6 kilograms
    pub fn orbital_period(self, mass: UInt) -> UInt {
        TAU_INT * (self.semimajor.pow(3) * PRECISION.pow(3) / (GRAV_INT * mass)).sqrt() / PRECISION
    }
    /// Speed of the body in km/s, mass of the parent in 10^6 kilograms, angle in radians
    pub fn orbital_speed(self, mass: UInt, theta: f64) -> UInt {
        (GRAV_INT
            * mass
            * (UInt::TWO * PRECISION / self.orbital_distance(theta) - PRECISION / self.semimajor))
            .sqrt()
            / UInt::from_digit(1000)
            / PREC_SQRT
            / PRECISION
    }
    /// Predict the position of an object after a given number of milliseconds.
    /// Current and resulting angles are measured in radians from perihelion. Mass of the star is given in 10^6 kilograms.
    pub fn predict_from(self, mass: UInt, current: f64, time: UInt) -> f64 {
        let p = self.orbital_period(mass);
        let a = self.semimajor.as_::<f64>();
        let a2 = a * a;
        let e = self.eccentricity;
        let e2 = e * e;
        let b2 = a2 * (1.0 - e2);
        let b = b2.sqrt();
        let mu = (mass * GRAV_INT / PRECISION / PRECISION).as_::<f64>() / PREC_F64;
        let calc = |theta: f64| {
            let cos = theta.cos();
            let ect = 1.0 + e * cos;
            b2 * b / (ect * ect)
                * ((2.0 * ect + e2 - 1.0) / (a * mu * (2.0 * a2 * ect - b2))).sqrt()
        };
        let mut t = (time % p).as_::<f64>();
        let mut out = current;
        let step = t / 1000.0;
        while t > 0.0 {
            // inverse riemann sum
            let d1 = calc(out);
            t -= step;
            out += step / d1;
        }
        out % TAU
    }
    /// Predict the output angle of an orbit at a given point in time.
    /// Can take a previous calculation for better performance, though benefits may be limited because of the modular nature of this problem.
    pub fn predict(self, mass: UInt, time: UInt, last: Option<(UInt, f64)>) -> f64 {
        if let Some((t0, a0)) = last {
            if time > t0 {
                return self.predict_from(mass, a0, time - t0);
            }
        }
        self.predict_from(mass, self.start, time)
    }
}
