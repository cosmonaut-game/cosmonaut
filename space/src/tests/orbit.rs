use crate::body::*;
use std::f64::consts::PI;
#[allow(dead_code)]
fn round(val: f64, digits: u8) -> f64 {
    let exp = 10u32.pow(digits as _) as f64;
    (val * exp).round() / exp
}
#[allow(dead_code)]
fn trunc(val: u128, digits: u8) -> u128 {
    let exp = 10u128.pow(digits as _);
    val / exp * exp
}
#[test]
fn earth_sun() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let sol = UInt::from(19885u64) * UInt::TEN.pow(26);
    let o = Orbit::EARTH;
    assert_eq!(
        o.perihelion() / 10u64.pow(9),
        UInt::from(147u64),
        "perihelion failed"
    );
    assert_eq!(
        o.aphelion() / 10u64.pow(9),
        UInt::from(152u64),
        "aphelion failed"
    );
    assert_eq!(
        o.orbital_distance(0.0) / 10u64.pow(6),
        o.perihelion() / 10u64.pow(6),
        "orbital distance @ perihelion failed"
    );
    assert_eq!(
        o.orbital_distance(PI) / 10u64.pow(6),
        o.aphelion() / 10u64.pow(6),
        "orbital distance @ aphelion failed"
    );
    assert_eq!(
        o.orbital_period(sol) / 10u64.pow(4),
        UInt::from(31558809u64) / 10u64.pow(4),
        "orbital period failed"
    );
}

#[test]
fn earth_moon() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let sol = UInt::from(5972u64) * UInt::TEN.pow(21);
    let o = Orbit::MOON;
    assert_eq!(
        o.perihelion() / 10u64.pow(6),
        UInt::from(363u64),
        "perihelion failed"
    );
    assert_eq!(
        o.aphelion() / 10u64.pow(6),
        UInt::from(405u64),
        "aphelion failed"
    );
    assert_eq!(
        o.orbital_distance(0.0) / 10u64.pow(3),
        o.perihelion() / 10u64.pow(3),
        "orbital distance @ perihelion failed"
    );
    assert_eq!(
        o.orbital_distance(PI) / 10u64.pow(3),
        o.aphelion() / 10u64.pow(3),
        "orbital distance @ aphelion failed"
    );
    assert_eq!(
        o.orbital_period(sol) / 10u64.pow(4),
        UInt::from(237u64),
        "orbital period failed"
    );
}
