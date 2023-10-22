use crate::body::*;
use bnum::cast::*;
use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::Sub;

#[inline(always)]
fn assert_err<T: As + Sub<Output = T> + PartialOrd + Copy + Display, F: Display>(
    calculated: T,
    expected: impl Into<T>,
    error: f64,
    msg: F,
) where
    f64: CastFrom<T>,
{
    let expected = expected.into();
    let err = (if calculated > expected {
        calculated - expected
    } else {
        expected - calculated
    })
    .as_::<f64>()
        / expected.as_::<f64>();
    if err.abs() > error {
        panic!(
            "{msg}: error is {:.4}%, maximum allowed is {:.4}%\ncalculated: {calculated}, expected: {expected}",
            err * 100.0,
            error * 100.0
        );
    }
}
#[test]
fn earth_sun() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let sol = UInt::from(19885u64) * UInt::TEN.pow(20);
    let o = Orbit::EARTH;
    assert_err(o.perihelion(), 147095000u64, 0.00028, "perihelion failed");
    assert_err(o.aphelion(), 152100000u64, 0.00028, "aphelion failed");
    assert_err(
        o.orbital_distance(0.0),
        o.perihelion(),
        0.00002,
        "orbital distance @ perihelion failed",
    );
    assert_err(
        o.orbital_distance(PI),
        o.aphelion(),
        0.00002,
        "orbital distance @ aphelion failed",
    );
    assert_err(
        o.orbital_period(sol),
        31558809u64,
        0.0001,
        "orbital period failed",
    );
    assert_err(
        o.predict(sol, 0.0, o.orbital_period(sol) / 2),
        PI,
        0.001,
        "orbital predictions failed for half orbit",
    );
}

#[test]
fn earth_moon() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let sol = UInt::from(5972u64) * UInt::TEN.pow(15);
    let o = Orbit::MOON;
    assert_err(o.perihelion(), 363300u64, 0.00002, "perihelion failed");
    assert_err(o.aphelion(), 405500u64, 0.00002, "aphelion failed");
    assert_err(
        o.orbital_distance(0.0),
        o.perihelion(),
        0.0,
        "orbital distance @ perihelion failed",
    );
    assert_err(
        o.orbital_distance(PI),
        o.aphelion(),
        0.0,
        "orbital distance @ aphelion failed",
    );
    assert_err(
        o.orbital_period(sol),
        2360594u64,
        0.005,
        "orbital period failed",
    );
    assert_err(
        o.predict(sol, 0.0, o.orbital_period(sol) / 2),
        PI,
        0.001,
        "orbital predictions failed for half orbit",
    );
}
#[test]
fn circular() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let semimajor = rng.gen_range(UInt::TEN.pow(3)..UInt::TEN.pow(6));
    let o = Orbit::circular(semimajor, bevy::math::DVec3::ZERO);
    let mass = rng.gen_range(UInt::TEN.pow(15)..UInt::TEN.pow(25));
    let per = o.orbital_period(mass);
    println!("semimajor = {semimajor}, mass = {mass}, orbital period = {per}");
    assert_err(
        o.orbital_speed(mass, 0.0),
        UInt::parse_str_radix("6283185", 10) * semimajor / per / PRECISION,
        0.15,
        "orbital speed failed",
    );
    assert_err(
        o.predict(mass, 0.0, per / 2),
        PI,
        0.001,
        "orbital predictions failed for half orbit",
    );
}
