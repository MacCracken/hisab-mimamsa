//! Scale bridge — multi-scale structure, RG wrappers, bhava bridge functions.
//!
//! Connects renormalization group flow (scale-dependent couplings) with the
//! cosmological fixed-point analysis to provide bridge functions for bhava's
//! consciousness model at Scales 6 and 7.
//!
//! All energy scales in GeV (natural units).

use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

use crate::constants::{ALPHA, ALPHA_S_MZ, M_Z_GEV};
use crate::cosmology::friedmann::CosmologicalParameters;
use crate::error::{MimamsaError, ensure_finite, require_finite};
use crate::quantum_field::coupling::{
    running_coupling_qcd_analytic, running_coupling_qed_analytic,
};

use crate::error::require_all_finite;

use super::fixed_point::manifestation_intensity;

// ── Zodiac mapping tables ────────────────────────────────────────────────

/// Element index per sign: Aries=Fire(0), Taurus=Earth(1), Gemini=Air(2), Cancer=Water(3), ...
const SIGN_ELEMENT: [usize; 12] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

/// Modality index per sign: Aries=Cardinal(0), Taurus=Fixed(1), Gemini=Mutable(2), ...
const SIGN_MODALITY: [usize; 12] = [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2];

/// Planet weights: Sun and Moon (luminaries) get 2.0, all others 1.0.
const PLANET_WEIGHTS: [f64; 10] = [2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

/// Aspect angle tolerance for classification (degrees).
///
/// 8° is the standard classical astrology orb tolerance for major aspects.
/// Wide enough to catch most meaningful aspects, narrow enough to avoid
/// false positives from unrelated angular separations.
const ASPECT_ORB: f64 = 8.0;

/// Output bundle for a bhava bridge computation at a given redshift.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BridgeOutput {
    /// Manifestation intensity ∈ [0, 1] (Scale 6).
    pub intensity: f64,
    /// Cosmic breath phase ∈ [0, 1] (Scale 7).
    pub phase: f64,
    /// Unity parameter = 1 − intensity.
    pub unity_param: f64,
    /// Rate of convergence toward fixed point (|dI/dz|, approximate).
    pub convergence_rate: f64,
}

impl BridgeOutput {
    /// Compute the full bridge output at redshift z.
    ///
    /// Convergence rate is estimated by finite difference with Δz = 0.01.
    /// This step size is adequate for the smooth dark-energy-dominated era (z < ~0.3)
    /// but may be too coarse near phase transitions (radiation→matter at z ~ 3400).
    /// Future versions may use adaptive step sizing for high-z precision.
    #[instrument(level = "debug", skip(params), ret)]
    pub fn at_redshift(params: &CosmologicalParameters, z: f64) -> Result<Self, MimamsaError> {
        let intensity = bridge_scale_6(params, z)?;
        let phase = bridge_scale_7(params, z)?;
        let unity_param = 1.0 - intensity;

        let dz = 0.01;
        let intensity_plus = bridge_scale_6(params, z + dz)?;
        let convergence_rate = ensure_finite(
            ((intensity - intensity_plus) / dz).abs(),
            "BridgeOutput::convergence_rate",
        )?;

        Ok(Self {
            intensity,
            phase,
            unity_param,
            convergence_rate,
        })
    }
}

/// QED running coupling at energy scale μ (GeV).
///
/// Convenience wrapper: runs α from [`ALPHA`] at [`M_Z_GEV`] to μ using
/// the one-loop analytic formula.
#[instrument(level = "trace")]
#[inline]
pub fn scale_coupling_qed(mu_gev: f64) -> Result<f64, MimamsaError> {
    require_finite(mu_gev, "scale_coupling_qed")?;
    if mu_gev <= 0.0 {
        warn!(mu_gev, "scale_coupling_qed: scale must be positive");
        return Err(MimamsaError::Computation(
            "scale_coupling_qed: scale must be positive".to_string(),
        ));
    }
    running_coupling_qed_analytic(ALPHA, M_Z_GEV, mu_gev)
}

/// QCD running coupling at energy scale μ (GeV) with n_f active flavors.
///
/// Convenience wrapper: runs α_s from [`ALPHA_S_MZ`] at [`M_Z_GEV`] to μ.
#[instrument(level = "trace")]
#[inline]
pub fn scale_coupling_qcd(mu_gev: f64, n_f: u8) -> Result<f64, MimamsaError> {
    require_finite(mu_gev, "scale_coupling_qcd")?;
    if mu_gev <= 0.0 {
        warn!(mu_gev, "scale_coupling_qcd: scale must be positive");
        return Err(MimamsaError::Computation(
            "scale_coupling_qcd: scale must be positive".to_string(),
        ));
    }
    running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, mu_gev, n_f)
}

// ── Scale 3: Planetary field ─────────────────────────────────────────────

/// Planetary field state — Scale 3 bridge output for bhava/soorat.
///
/// Dependency-free: computed from f64 primitives (planetary longitudes,
/// aspect strengths), not jyotish types. The caller computes positions
/// with jyotish, then passes results through these bridge functions.
///
/// All arrays are normalized (sum = 1.0) so they represent distributions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryField {
    /// Element balance [Fire, Earth, Air, Water], sum = 1.0.
    pub element_balance: [f64; 4],
    /// Modality balance [Cardinal, Fixed, Mutable], sum = 1.0.
    pub modality_balance: [f64; 3],
    /// Aspect tension ∈ [0, 1]. Hard aspects (square, opposition).
    pub aspect_tension: f64,
    /// Aspect harmony ∈ [0, 1]. Soft aspects (trine, sextile).
    pub aspect_harmony: f64,
    /// Energy distribution across 12 houses, sum = 1.0.
    pub house_emphasis: [f64; 12],
    /// Fraction of planets in retrograde motion ∈ [0, 1].
    pub retrograde_fraction: f64,
    /// Dominant element index (0=Fire, 1=Earth, 2=Air, 3=Water).
    pub dominant_element: u8,
    /// Dominant modality index (0=Cardinal, 1=Fixed, 2=Mutable).
    pub dominant_modality: u8,
}

/// Sign index (0–11) from ecliptic longitude (0–360°).
#[inline]
fn sign_index(longitude_deg: f64) -> usize {
    let normalized = ((longitude_deg % 360.0) + 360.0) % 360.0;
    (normalized / 30.0) as usize % 12
}

/// Compute element balance from 10 planetary ecliptic longitudes.
///
/// Each planet contributes to its sign's element. Luminaries (Sun, Moon)
/// are weighted 2×. Returns normalized [Fire, Earth, Air, Water].
///
/// Planet order: Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto.
#[must_use = "element balance distribution should be used"]
#[instrument(level = "trace", skip(longitudes_deg))]
pub fn element_balance(longitudes_deg: &[f64; 10]) -> Result<[f64; 4], MimamsaError> {
    require_all_finite(longitudes_deg, "element_balance")?;
    let mut counts = [0.0_f64; 4];
    for (i, &lon) in longitudes_deg.iter().enumerate() {
        let sign = sign_index(lon);
        counts[SIGN_ELEMENT[sign]] += PLANET_WEIGHTS[i];
    }
    let total: f64 = counts.iter().sum();
    if total <= 0.0 {
        warn!(total, "element_balance: zero total weight");
        return Err(MimamsaError::Computation(
            "element_balance: zero total weight".to_string(),
        ));
    }
    for c in &mut counts {
        *c /= total;
    }
    Ok(counts)
}

/// Compute modality balance from 10 planetary ecliptic longitudes.
///
/// Returns normalized [Cardinal, Fixed, Mutable].
#[must_use = "modality balance distribution should be used"]
#[instrument(level = "trace", skip(longitudes_deg))]
pub fn modality_balance(longitudes_deg: &[f64; 10]) -> Result<[f64; 3], MimamsaError> {
    require_all_finite(longitudes_deg, "modality_balance")?;
    let mut counts = [0.0_f64; 3];
    for (i, &lon) in longitudes_deg.iter().enumerate() {
        let sign = sign_index(lon);
        counts[SIGN_MODALITY[sign]] += PLANET_WEIGHTS[i];
    }
    let total: f64 = counts.iter().sum();
    if total <= 0.0 {
        warn!(total, "modality_balance: zero total weight");
        return Err(MimamsaError::Computation(
            "modality_balance: zero total weight".to_string(),
        ));
    }
    for c in &mut counts {
        *c /= total;
    }
    Ok(counts)
}

/// Compute aspect tension and harmony from aspect data.
///
/// Hard aspects (square ≈ 90°, opposition ≈ 180°) contribute to tension.
/// Soft aspects (sextile ≈ 60°, trine ≈ 120°) contribute to harmony.
/// Conjunction (≈ 0°) is neutral (neither tension nor harmony).
///
/// Input: slice of (aspect_angle_degrees, strength_0_to_1) pairs.
/// Returns (tension, harmony), each ∈ [0, 1].
#[must_use = "aspect tension/harmony values should be used"]
#[instrument(level = "trace", skip(aspects))]
pub fn aspect_tension_harmony(aspects: &[(f64, f64)]) -> Result<(f64, f64), MimamsaError> {
    if aspects.is_empty() {
        return Ok((0.0, 0.0));
    }
    let mut hard_sum = 0.0_f64;
    let mut soft_sum = 0.0_f64;
    let mut total_strength = 0.0_f64;

    for &(angle, strength) in aspects {
        require_finite(angle, "aspect_tension_harmony")?;
        require_finite(strength, "aspect_tension_harmony")?;
        let s = strength.clamp(0.0, 1.0);
        total_strength += s;

        // Hard: square (90°) or opposition (180°)
        if (angle - 90.0).abs() < ASPECT_ORB || (angle - 180.0).abs() < ASPECT_ORB {
            hard_sum += s;
        }
        // Soft: sextile (60°) or trine (120°)
        if (angle - 60.0).abs() < ASPECT_ORB || (angle - 120.0).abs() < ASPECT_ORB {
            soft_sum += s;
        }
    }

    if total_strength <= 0.0 {
        return Ok((0.0, 0.0));
    }
    let tension = ensure_finite(hard_sum / total_strength, "aspect_tension")?;
    let harmony = ensure_finite(soft_sum / total_strength, "aspect_harmony")?;
    Ok((tension.clamp(0.0, 1.0), harmony.clamp(0.0, 1.0)))
}

/// Compute house emphasis from planet longitudes and house cusp longitudes.
///
/// Each planet contributes its weight to the house it occupies.
/// Returns normalized 12-element array (sum = 1.0).
///
/// Handles the 360° wraparound correctly.
#[must_use = "house emphasis distribution should be used"]
#[instrument(level = "trace", skip(planet_longitudes, house_cusps))]
pub fn house_emphasis(
    planet_longitudes: &[f64; 10],
    house_cusps: &[f64; 12],
) -> Result<[f64; 12], MimamsaError> {
    require_all_finite(planet_longitudes, "house_emphasis")?;
    require_all_finite(house_cusps, "house_emphasis")?;

    let mut emphasis = [0.0_f64; 12];

    for (p_idx, &p_lon) in planet_longitudes.iter().enumerate() {
        let p = ((p_lon % 360.0) + 360.0) % 360.0;
        let mut placed = false;
        for h in 0..12 {
            let cusp_start = ((house_cusps[h] % 360.0) + 360.0) % 360.0;
            let cusp_end = ((house_cusps[(h + 1) % 12] % 360.0) + 360.0) % 360.0;

            let in_house = if cusp_start < cusp_end {
                p >= cusp_start && p < cusp_end
            } else {
                // Wraps around 360°
                p >= cusp_start || p < cusp_end
            };

            if in_house {
                emphasis[h] += PLANET_WEIGHTS[p_idx];
                placed = true;
                break;
            }
        }
        if !placed {
            emphasis[0] += PLANET_WEIGHTS[p_idx]; // fallback to 1st house
        }
    }

    let total: f64 = emphasis.iter().sum();
    if total <= 0.0 {
        warn!(total, "house_emphasis: zero total weight");
        return Err(MimamsaError::Computation(
            "house_emphasis: zero total weight".to_string(),
        ));
    }
    for e in &mut emphasis {
        *e /= total;
    }
    Ok(emphasis)
}

/// Compute retrograde fraction from daily motion values (degrees/day).
///
/// Negative daily motion indicates retrograde. Sun and Moon never retrograde
/// but the function handles any input.
#[must_use = "retrograde fraction should be used"]
pub fn retrograde_fraction(daily_motions: &[f64]) -> Result<f64, MimamsaError> {
    if daily_motions.is_empty() {
        return Ok(0.0);
    }
    for &m in daily_motions {
        require_finite(m, "retrograde_fraction")?;
    }
    let retro_count = daily_motions.iter().filter(|&&m| m < 0.0).count();
    ensure_finite(
        retro_count as f64 / daily_motions.len() as f64,
        "retrograde_fraction",
    )
}

/// bhava Scale 3 bridge: planetary field → personality manifestation.
///
/// Computes the full planetary field state from jyotish primitives.
/// All inputs are dependency-free f64 values — no jyotish types cross
/// this boundary.
///
/// # Arguments
/// * `planet_longitudes` — 10 ecliptic longitudes in degrees (Sun..Pluto).
/// * `house_cusps` — 12 house cusp longitudes in degrees.
/// * `aspects` — Slice of (angle_degrees, strength) pairs for active aspects.
/// * `daily_motions` — Daily motion in degrees/day per planet.
#[instrument(level = "debug", skip_all)]
pub fn bridge_scale_3(
    planet_longitudes: &[f64; 10],
    house_cusps: &[f64; 12],
    aspects: &[(f64, f64)],
    daily_motions: &[f64],
) -> Result<PlanetaryField, MimamsaError> {
    let elem = element_balance(planet_longitudes)?;
    let modal = modality_balance(planet_longitudes)?;
    let (tension, harmony) = aspect_tension_harmony(aspects)?;
    let houses = house_emphasis(planet_longitudes, house_cusps)?;
    let retro = retrograde_fraction(daily_motions)?;

    // Dominant element and modality
    let dominant_element = elem
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i as u8)
        .unwrap_or(0);

    let dominant_modality = modal
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i as u8)
        .unwrap_or(0);

    Ok(PlanetaryField {
        element_balance: elem,
        modality_balance: modal,
        aspect_tension: tension,
        aspect_harmony: harmony,
        house_emphasis: houses,
        retrograde_fraction: retro,
        dominant_element,
        dominant_modality,
    })
}

// ── Scale 4/5 stubs ─────────────────────────────────────────────────────

/// bhava Scale 4 bridge stub: stellar influence → soul motivation layers.
///
/// Placeholder returning 0.5 (neutral) until the tara crate is available.
#[deprecated(note = "stub returning 0.5 — will be replaced when tara v1 is available")]
#[inline]
pub fn bridge_scale_4() -> Result<f64, MimamsaError> {
    Ok(0.5)
}

/// bhava Scale 5 bridge stub: galactic structure → civilizational personality fields.
///
/// Placeholder returning 0.5 (neutral) until large-scale structure data is available.
#[deprecated(note = "stub returning 0.5 — will be replaced when brahmanda hardens")]
#[inline]
pub fn bridge_scale_5() -> Result<f64, MimamsaError> {
    Ok(0.5)
}

// ── Scale 6/7: Cosmic ───────────────────────────────────────────────────

/// bhava Scale 6 bridge: cosmic expansion → manifestation intensity.
///
/// Returns the manifestation intensity at redshift z ∈ [0, 1], serving as
/// the scalar input to bhava's Scale 6 personality field computations.
///
/// Maps the cosmos's thermodynamic distance from equilibrium into a scalar
/// that modulates all manifestation at that epoch.
#[instrument(level = "trace", skip(params))]
pub fn bridge_scale_6(params: &CosmologicalParameters, z: f64) -> Result<f64, MimamsaError> {
    manifestation_intensity(params, z)
}

/// bhava Scale 7 bridge: cosmic breath phase (unity/differentiation cycle).
///
/// Returns a phase ∈ [0, 1]:
/// - 0.0 = pure unity (heat death / maximum entropy)
/// - 1.0 = maximum differentiation (Big Bang / minimum entropy)
///
/// For ΛCDM (monotonic expansion) this equals Scale 6. In future cyclic
/// cosmology extensions, Scale 7 would incorporate the oscillation phase
/// of successive cosmic cycles.
#[instrument(level = "trace", skip(params))]
pub fn bridge_scale_7(params: &CosmologicalParameters, z: f64) -> Result<f64, MimamsaError> {
    // For ΛCDM, phase = intensity. Separated for future cyclic extensions.
    manifestation_intensity(params, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planck() -> CosmologicalParameters {
        CosmologicalParameters::planck2018()
    }

    #[test]
    fn test_scale_coupling_qed_at_mz() {
        let a = scale_coupling_qed(M_Z_GEV).unwrap();
        assert!((a - ALPHA).abs() / ALPHA < 1e-6);
    }

    #[test]
    fn test_scale_coupling_qed_increases() {
        let a_200 = scale_coupling_qed(200.0).unwrap();
        assert!(a_200 > ALPHA, "QED coupling should increase with energy");
    }

    #[test]
    fn test_scale_coupling_qcd_at_mz() {
        let a = scale_coupling_qcd(M_Z_GEV, 6).unwrap();
        assert!((a - ALPHA_S_MZ).abs() / ALPHA_S_MZ < 1e-6);
    }

    #[test]
    fn test_scale_coupling_qcd_decreases() {
        let a_1000 = scale_coupling_qcd(1000.0, 6).unwrap();
        assert!(
            a_1000 < ALPHA_S_MZ,
            "QCD coupling should decrease with energy"
        );
    }

    #[test]
    fn test_bridge_scale_6_bounds() {
        let p = planck();
        for z in [0.0, 0.5, 1.0, 5.0, 100.0] {
            let i = bridge_scale_6(&p, z).unwrap();
            assert!(
                (0.0..=1.0).contains(&i),
                "bridge_scale_6({z}) = {i} out of [0,1]"
            );
        }
    }

    #[test]
    fn test_bridge_scale_7_bounds() {
        let p = planck();
        for z in [0.0, 0.5, 1.0, 5.0, 100.0] {
            let phase = bridge_scale_7(&p, z).unwrap();
            assert!(
                (0.0..=1.0).contains(&phase),
                "bridge_scale_7({z}) = {phase} out of [0,1]"
            );
        }
    }

    #[test]
    fn test_bridge_output_consistency() {
        let out = BridgeOutput::at_redshift(&planck(), 0.0).unwrap();
        assert!((out.intensity + out.unity_param - 1.0).abs() < 1e-12);
        assert!(out.convergence_rate >= 0.0);
    }

    #[test]
    fn test_bridge_output_serde() {
        let out = BridgeOutput::at_redshift(&planck(), 0.5).unwrap();
        let json = serde_json::to_string(&out).unwrap();
        let _back: BridgeOutput = serde_json::from_str(&json).unwrap();
    }

    // ── Scale 3 tests ──

    /// All planets at 0° Aries (Fire, Cardinal)
    fn all_aries() -> [f64; 10] {
        [0.0; 10]
    }

    /// Spread: one planet per sign starting at Aries
    fn spread_signs() -> [f64; 10] {
        [
            5.0, 35.0, 65.0, 95.0, 125.0, 155.0, 185.0, 215.0, 245.0, 275.0,
        ]
    }

    /// Equal 30° house cusps
    fn equal_cusps() -> [f64; 12] {
        let mut cusps = [0.0; 12];
        for (i, cusp) in cusps.iter_mut().enumerate() {
            *cusp = i as f64 * 30.0;
        }
        cusps
    }

    #[test]
    fn test_element_balance_all_fire() {
        let elem = element_balance(&all_aries()).unwrap();
        assert!(
            (elem[0] - 1.0).abs() < 1e-10,
            "all-Aries should be 100% Fire"
        );
        assert!(elem[1].abs() < 1e-10);
    }

    #[test]
    fn test_element_balance_sums_to_one() {
        let elem = element_balance(&spread_signs()).unwrap();
        let sum: f64 = elem.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_modality_balance_sums_to_one() {
        let modal = modality_balance(&spread_signs()).unwrap();
        let sum: f64 = modal.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_aspect_tension_hard() {
        // All square aspects → pure tension
        let aspects = vec![(90.0, 1.0), (90.0, 1.0), (180.0, 1.0)];
        let (tension, harmony) = aspect_tension_harmony(&aspects).unwrap();
        assert!((tension - 1.0).abs() < 1e-10);
        assert!(harmony.abs() < 1e-10);
    }

    #[test]
    fn test_aspect_harmony_soft() {
        // All trine aspects → pure harmony
        let aspects = vec![(120.0, 1.0), (60.0, 1.0)];
        let (tension, harmony) = aspect_tension_harmony(&aspects).unwrap();
        assert!(tension.abs() < 1e-10);
        assert!((harmony - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_aspect_empty() {
        let (t, h) = aspect_tension_harmony(&[]).unwrap();
        assert!(t.abs() < 1e-10);
        assert!(h.abs() < 1e-10);
    }

    #[test]
    fn test_house_emphasis_sums_to_one() {
        let houses = house_emphasis(&spread_signs(), &equal_cusps()).unwrap();
        let sum: f64 = houses.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_retrograde_fraction_none() {
        let motions = [1.0, 13.0, 1.2, 0.6, 0.5, 0.08, 0.03, 0.01, 0.01, 0.01];
        let frac = retrograde_fraction(&motions).unwrap();
        assert!(frac.abs() < 1e-10);
    }

    #[test]
    fn test_retrograde_fraction_some() {
        let motions = [1.0, 13.0, -0.5, 0.6, -0.3, 0.08, 0.03, 0.01, 0.01, 0.01];
        let frac = retrograde_fraction(&motions).unwrap();
        assert!((frac - 0.2).abs() < 1e-10); // 2/10
    }

    #[test]
    fn test_bridge_scale_3_complete() {
        let field = bridge_scale_3(
            &spread_signs(),
            &equal_cusps(),
            &[(90.0, 0.8), (120.0, 0.9), (60.0, 0.5)],
            &[1.0, 13.0, -0.5, 0.6, 0.5, 0.08, 0.03, 0.01, 0.01, 0.01],
        )
        .unwrap();
        assert!((field.element_balance.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert!((field.modality_balance.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert!((field.house_emphasis.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert!(field.aspect_tension >= 0.0 && field.aspect_tension <= 1.0);
        assert!(field.aspect_harmony >= 0.0 && field.aspect_harmony <= 1.0);
        assert!(field.retrograde_fraction >= 0.0 && field.retrograde_fraction <= 1.0);
        assert!(field.dominant_element <= 3);
        assert!(field.dominant_modality <= 2);
    }

    #[test]
    fn test_bridge_scale_3_serde() {
        let field =
            bridge_scale_3(&spread_signs(), &equal_cusps(), &[(90.0, 0.8)], &[1.0; 10]).unwrap();
        let json = serde_json::to_string(&field).unwrap();
        let _back: PlanetaryField = serde_json::from_str(&json).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn test_bridge_scale_4_stub() {
        assert!((bridge_scale_4().unwrap() - 0.5).abs() < 1e-10);
    }

    #[test]
    #[allow(deprecated)]
    fn test_bridge_scale_5_stub() {
        assert!((bridge_scale_5().unwrap() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_negative_scale_rejected() {
        assert!(scale_coupling_qed(-1.0).is_err());
        assert!(scale_coupling_qcd(-1.0, 6).is_err());
    }
}
