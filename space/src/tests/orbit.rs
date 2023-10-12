use crate::body::*;
use bnum::cast::*;
use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::Sub;

#[inline(always)]
fn assert_err<T: As + Sub<Output = T> + PartialOrd + Copy, F: Display>(
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
            "{msg}: error is {:.4}%, maximum allowed is {:.4}%",
            err * 100.0,
            error * 100.0
        );
    }
}
#[test]
fn earth_sun() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let sol = UInt::from(19885u64) * UInt::TEN.pow(26);
    let o = Orbit::EARTH;
    assert_err(
        o.perihelion(),
        147095000000u64,
        0.00028,
        "perihelion failed",
    );
    assert_err(o.aphelion(), 152100000000u64, 0.00028, "aphelion failed");
    assert_err(
        o.orbital_distance(0.0),
        o.perihelion(),
        0.00001,
        "orbital distance @ perihelion failed",
    );
    assert_err(
        o.orbital_distance(PI),
        o.aphelion(),
        0.00001,
        "orbital distance @ aphelion failed",
    );
    assert_err(
        o.orbital_period(sol),
        31558809u64,
        0.0001,
        "orbital period failed",
    );
    let per = o.orbital_period(sol);
    for _ in 0..100 {
        let t = rand::random();
        assert_err(
            o.predict(sol, t, per),
            t,
            0.0001,
            format_args!("orbital predictions failed with starting angle of {t:.6} radians"),
        );
    }
}

#[test]
fn earth_moon() {
    // numbers taken from https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let sol = UInt::from(5972u64) * UInt::TEN.pow(21);
    let o = Orbit::MOON;
    assert_err(o.perihelion(), 363300000u64, 0.00001, "perihelion failed");
    assert_err(o.aphelion(), 405500000u64, 0.00001, "aphelion failed");
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
    let per = o.orbital_period(sol);
    for _ in 0..100 {
        let t = rand::random();
        assert_err(
            o.predict(sol, t, per),
            t,
            0.0001,
            format_args!("orbital predictions failed with starting angle of {t:.6} radians"),
        );
    }
}
