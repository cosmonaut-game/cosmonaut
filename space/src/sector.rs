use cosmonaut_common::*;

/// A star. Holds information pertaining to one star.
#[derive(Debug, Clone, Copy)]
pub struct StarData {
    /// Temperature in kelvins
    pub temperature: u16,
    /// Mass in 10^6 kilograms
    pub mass: UInt,
    /// Radius in kilometers
    pub radius: UInt,
}
impl StarData {
    /// Power output in megawatts
    pub fn power_output(self) -> UInt {
        UInt::from(567030000u64)
            * UInt::from(self.temperature).pow(4)
            * PI_INT
            * UInt::FOUR
            * self.radius.pow(2)
            / PRECISION.pow(3)
    }
    /// Apricity for the surface of a given planet, given in kW/m^2
    /// More precise than manual calculations
    /// Multiply by the sine of the tilt if the surface isn't flat
    pub fn apricity(self, distance: UInt) -> f64 {
        (UInt::from(567030000u64) * UInt::from(self.temperature).pow(4) * self.radius.pow(2))
            .as_::<f64>()
            / (distance.pow(2).as_::<f64>() * PREC_F64.powi(3) * 1000f64)
    }
}

/// Either a star or a binary pair.
#[derive(Debug, Clone, Copy)]
pub enum Star {
    Single(StarData),
    Binary(StarData, StarData), // TODO: binary orbits maybe?
}
impl Star {
    /// Mass of the combined system
    /// See StarData::mass
    pub fn mass(self) -> UInt {
        match self {
            Star::Single(star) => star.mass,
            Star::Binary(a, b) => a.mass + b.mass,
        }
    }
    /// Power output in megawatts
    /// See StarData::power_output
    pub fn power_output(self) -> UInt {
        match self {
            Star::Single(star) => star.power_output(),
            Star::Binary(a, b) => a.power_output() + b.power_output(),
        }
    }
    /// Apricity in kW/m^2
    /// See StarData::apricity
    pub fn apricity(self, distance: UInt) -> f64 {
        match self {
            Star::Single(star) => star.apricity(distance),
            Star::Binary(a, b) => a.apricity(distance) + b.apricity(distance), // TODO: optimize calculation
        }
    }
}
