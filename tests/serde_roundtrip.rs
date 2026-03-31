//! Serde roundtrip tests — serialize and deserialize every public type.

use hisab_mimamsa::relativity::{
    black_hole::BlackHoleProperties,
    geodesic::{GeodesicPoint, GeodesicType},
    lorentz::{FourVector, IntervalType},
    metric::{MetricSignature, SpacetimeMetric},
};

#[cfg(feature = "cosmology")]
use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;

const M_SUN: f64 = 1.989e30;

fn roundtrip_json<T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq>(
    val: &T,
    name: &str,
) {
    let json = serde_json::to_string(val).unwrap_or_else(|e| panic!("{name}: serialize failed: {e}"));
    let back: T =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("{name}: deserialize failed: {e}"));
    assert_eq!(*val, back, "{name}: roundtrip mismatch");
}

fn roundtrip_json_no_eq<T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug>(
    val: &T,
    name: &str,
) {
    let json = serde_json::to_string(val).unwrap_or_else(|e| panic!("{name}: serialize failed: {e}"));
    let _back: T =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("{name}: deserialize failed: {e}"));
}

#[test]
fn roundtrip_four_vector() {
    let fv = FourVector::new(3.0, 2.0, 1.0, 0.5).unwrap();
    roundtrip_json(&fv, "FourVector");
}

#[test]
fn roundtrip_interval_type() {
    for it in [IntervalType::Timelike, IntervalType::Lightlike, IntervalType::Spacelike] {
        roundtrip_json(&it, &format!("IntervalType::{it:?}"));
    }
}

#[test]
fn roundtrip_geodesic_point() {
    let gp = GeodesicPoint {
        t: 1.0,
        r: 1e6,
        theta: std::f64::consts::FRAC_PI_2,
        phi: 0.0,
    };
    roundtrip_json_no_eq(&gp, "GeodesicPoint");
}

#[test]
fn roundtrip_geodesic_type() {
    for gt in [GeodesicType::Timelike, GeodesicType::Null, GeodesicType::Spacelike] {
        roundtrip_json(&gt, &format!("GeodesicType::{gt:?}"));
    }
}

#[test]
fn roundtrip_metric_signature() {
    for ms in [MetricSignature::MostlyPlus, MetricSignature::MostlyMinus] {
        roundtrip_json(&ms, &format!("MetricSignature::{ms:?}"));
    }
}

#[test]
fn roundtrip_spacetime_metric() {
    for sm in [
        SpacetimeMetric::Minkowski,
        SpacetimeMetric::Schwarzschild,
        SpacetimeMetric::Kerr,
        SpacetimeMetric::FLRW,
        SpacetimeMetric::ReissnerNordstrom,
    ] {
        roundtrip_json(&sm, &format!("SpacetimeMetric::{sm:?}"));
    }
}

#[test]
fn roundtrip_black_hole_properties() {
    let props = BlackHoleProperties::from_mass(M_SUN).unwrap();
    roundtrip_json_no_eq(&props, "BlackHoleProperties");
}

#[cfg(feature = "cosmology")]
#[test]
fn roundtrip_cosmological_parameters() {
    let params = CosmologicalParameters::planck2018();
    roundtrip_json_no_eq(&params, "CosmologicalParameters");
}
