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

// ── Scale 4: Stellar field ──────────────────────────────────────────────

/// Stellar field state — Scale 4 bridge output for bhava/soorat.
///
/// Dependency-free: computed from f64 primitives (tara stellar properties).
/// The caller computes stellar properties with tara, then passes results
/// through this bridge function.
///
/// All scalar fields are normalized ∈ [0, 1].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StellarField {
    /// Lifecycle fraction ∈ [0, 1]. 0 = stellar birth, 1 = end of main sequence.
    pub lifecycle_fraction: f64,
    /// Luminosity intensity ∈ [0, 1]. Normalized log-luminosity (0 = 0.01 L_sun, 1 = 1e6 L_sun).
    pub luminosity_intensity: f64,
    /// Thermal temperament ∈ [0, 1]. 0 = cool (M-type, ~2400 K), 1 = hot (O-type, ~50000 K).
    pub thermal_temperament: f64,
    /// Compositional complexity ∈ [0, 1]. Derived from metallicity [Fe/H].
    /// 0 = metal-poor ([Fe/H] <= -2), 1 = metal-rich ([Fe/H] >= +0.5).
    pub compositional_complexity: f64,
    /// Evolutionary urgency ∈ [0, 1]. Accelerates near end of main sequence.
    /// Uses τ³ to model increasing urgency as fuel depletes.
    pub evolutionary_urgency: f64,
    /// Spectral archetype index (0=O, 1=B, 2=A, 3=F, 4=G, 5=K, 6=M).
    pub spectral_archetype: u8,
}

/// Spectral class index from effective temperature.
///
/// Maps temperature to OBAFGKM spectral type (0–6).
fn spectral_archetype_from_temperature(temperature_k: f64) -> u8 {
    match temperature_k {
        t if t >= 30000.0 => 0, // O
        t if t >= 10000.0 => 1, // B
        t if t >= 7500.0 => 2,  // A
        t if t >= 6000.0 => 3,  // F
        t if t >= 5200.0 => 4,  // G
        t if t >= 3700.0 => 5,  // K
        _ => 6,                 // M
    }
}

/// bhava Scale 4 bridge: stellar influence → soul motivation layers.
///
/// Computes the stellar field state from tara f64 primitives.
/// All inputs are dependency-free — no tara types cross this boundary.
///
/// # Arguments
/// * `temperature_k` — Effective surface temperature (K, from tara `Star::temperature_k`).
/// * `luminosity_solar` — Luminosity in solar luminosities (from tara `Star::luminosity_solar`).
/// * `age_years` — Stellar age in years (from tara `Star::age_years`).
/// * `main_sequence_lifetime_years` — Main-sequence lifetime (from tara `evolution::main_sequence_lifetime`).
/// * `metallicity_feh` — Metallicity [Fe/H] in dex (from tara `Star::metallicity`, solar = 0.0).
#[instrument(level = "debug", skip_all)]
pub fn bridge_scale_4(
    temperature_k: f64,
    luminosity_solar: f64,
    age_years: f64,
    main_sequence_lifetime_years: f64,
    metallicity_feh: f64,
) -> Result<StellarField, MimamsaError> {
    require_all_finite(
        &[
            temperature_k,
            luminosity_solar,
            age_years,
            main_sequence_lifetime_years,
            metallicity_feh,
        ],
        "bridge_scale_4",
    )?;
    if temperature_k <= 0.0 || luminosity_solar <= 0.0 || main_sequence_lifetime_years <= 0.0 {
        warn!(
            temperature_k,
            luminosity_solar,
            main_sequence_lifetime_years,
            "bridge_scale_4: temperature, luminosity, and lifetime must be positive"
        );
        return Err(MimamsaError::Computation(
            "bridge_scale_4: temperature, luminosity, and lifetime must be positive".to_string(),
        ));
    }

    // Lifecycle fraction: age / main-sequence lifetime, clamped to [0, 1]
    let tau = (age_years / main_sequence_lifetime_years).clamp(0.0, 1.0);

    // Luminosity intensity: log-scale normalization
    // 0.01 L_sun → 0.0, 1e6 L_sun → 1.0 (8 decades)
    let log_l = luminosity_solar.max(0.01).log10(); // range: -2 to ~6
    let luminosity_intensity = ((log_l + 2.0) / 8.0).clamp(0.0, 1.0);

    // Thermal temperament: log-scale temperature normalization
    // 2400 K (M) → 0.0, 50000 K (O) → 1.0
    let log_t = temperature_k.log10(); // range: ~3.38 to ~4.7
    let thermal_temperament =
        ((log_t - 2400.0_f64.log10()) / (50000.0_f64.log10() - 2400.0_f64.log10())).clamp(0.0, 1.0);

    // Compositional complexity: metallicity normalization
    // [Fe/H] = -2.0 → 0.0, [Fe/H] = +0.5 → 1.0
    let compositional_complexity = ((metallicity_feh + 2.0) / 2.5).clamp(0.0, 1.0);

    // Evolutionary urgency: cubic acceleration near end of main sequence
    let evolutionary_urgency = (tau * tau * tau).clamp(0.0, 1.0);

    let spectral_archetype = spectral_archetype_from_temperature(temperature_k);

    Ok(StellarField {
        lifecycle_fraction: tau,
        luminosity_intensity,
        thermal_temperament,
        compositional_complexity,
        evolutionary_urgency,
        spectral_archetype,
    })
}

// ── Scale 5: Galactic field ─────────────────────────────────────────────

/// Galactic field state — Scale 5 bridge output for bhava/soorat.
///
/// Dependency-free: computed from f64 primitives (brahmanda galactic properties).
/// The caller computes galactic properties with brahmanda, then passes results
/// through this bridge function.
///
/// All scalar fields are normalized ∈ [0, 1].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GalacticField {
    /// Halo concentration ∈ [0, 1]. Normalized NFW concentration parameter.
    /// 0 = diffuse (c ~ 3), 1 = highly concentrated (c ~ 20).
    /// Maps to personality cohesion vs dispersal.
    pub concentration: f64,
    /// Density pressure ∈ [0, 1]. Normalized density contrast δ.
    /// 0 = deep void (δ = -1), 0.5 = mean density (δ = 0), 1 = extreme overdensity.
    /// Maps to environmental pressure on civilizational field.
    pub density_pressure: f64,
    /// Structure growth ∈ [0, 1]. Linear growth factor D(z).
    /// 0 = no structure (early universe), 1 = fully grown (today).
    /// Maps to civilizational developmental maturity.
    pub structure_growth: f64,
    /// Cosmic activity ∈ [0, 1]. Normalized star formation rate density.
    /// Peaks at z ~ 2 (cosmic noon). Maps to creative/generative intensity.
    pub cosmic_activity: f64,
    /// Chemical complexity ∈ [0, 1]. Normalized mass-metallicity relation.
    /// 0 = primordial composition, 1 = highly enriched.
    /// Maps to experiential depth and accumulated history.
    pub chemical_complexity: f64,
    /// Filamentarity ∈ [0, 1]. Cosmic web geometry parameter.
    /// 0 = isotropic (void/blob), 1 = highly anisotropic (filament).
    /// Maps to directedness vs diffusion of civilizational flow.
    pub filamentarity: f64,
    /// Web environment classification (0=Void, 1=Sheet, 2=Filament, 3=Node).
    pub web_environment: u8,
}

/// Classify cosmic web environment from tidal tensor eigenvalues.
///
/// Counts eigenvalues above threshold: 0 above = void, 1 = sheet,
/// 2 = filament, 3 = node.
fn web_environment_from_eigenvalues(
    lambda1: f64,
    lambda2: f64,
    lambda3: f64,
    threshold: f64,
) -> u8 {
    let count = [lambda1, lambda2, lambda3]
        .iter()
        .filter(|&&l| l > threshold)
        .count();
    count as u8 // 0=Void, 1=Sheet, 2=Filament, 3=Node
}

/// bhava Scale 5 bridge: galactic structure → civilizational personality fields.
///
/// Computes the galactic field state from brahmanda f64 primitives.
/// All inputs are dependency-free — no brahmanda types cross this boundary.
///
/// # Arguments
/// * `halo_concentration` — NFW concentration parameter c = R_vir/r_s (from brahmanda `HaloProperties`).
/// * `density_contrast` — Local density contrast δ = (ρ - ρ̄)/ρ̄ (from brahmanda `density_contrast`).
/// * `growth_factor` — Linear growth factor D(z)/D(0) ∈ [0, 1] (from brahmanda `growth_factor`).
/// * `sfr_density` — Star formation rate density in M_sun/yr/Mpc³ (from brahmanda `sfr_density_madau`).
/// * `metallicity_12log_oh` — Gas-phase metallicity 12+log(O/H) (from brahmanda `mass_metallicity`).
/// * `tidal_eigenvalues` — Three eigenvalues of the tidal tensor [λ₁, λ₂, λ₃] sorted descending
///   (from brahmanda cosmic web analysis). Used for filamentarity and web environment.
/// * `tidal_threshold` — Threshold for web classification (typically 0.0–0.2).
#[instrument(level = "debug", skip_all)]
pub fn bridge_scale_5(
    halo_concentration: f64,
    density_contrast: f64,
    growth_factor: f64,
    sfr_density: f64,
    metallicity_12log_oh: f64,
    tidal_eigenvalues: &[f64; 3],
    tidal_threshold: f64,
) -> Result<GalacticField, MimamsaError> {
    require_all_finite(
        &[
            halo_concentration,
            density_contrast,
            growth_factor,
            sfr_density,
            metallicity_12log_oh,
            tidal_eigenvalues[0],
            tidal_eigenvalues[1],
            tidal_eigenvalues[2],
            tidal_threshold,
        ],
        "bridge_scale_5",
    )?;

    // Concentration: normalize c ∈ [3, 20] → [0, 1]
    let concentration = ((halo_concentration - 3.0) / 17.0).clamp(0.0, 1.0);

    // Density pressure: δ ∈ [-1, +10] → [0, 1] via shifted sigmoid
    // δ = -1 → 0.0, δ = 0 → 0.5, δ = 10 → ~1.0
    let density_pressure = (1.0 / (1.0 + (-density_contrast).exp())).clamp(0.0, 1.0);

    // Structure growth: already ∈ [0, 1], just clamp
    let structure_growth = growth_factor.clamp(0.0, 1.0);

    // Cosmic activity: SFR density peaks at ~0.1 M_sun/yr/Mpc³ (cosmic noon z~2)
    // Normalize: 0.0 → 0.0, 0.1 → 1.0
    let cosmic_activity = (sfr_density / 0.1).clamp(0.0, 1.0);

    // Chemical complexity: 12+log(O/H) ∈ [7.5, 9.3] → [0, 1]
    let chemical_complexity = ((metallicity_12log_oh - 7.5) / 1.8).clamp(0.0, 1.0);

    // Filamentarity from eigenvalues: F = (λ₂ - λ₃) / (λ₁ - λ₃)
    let [l1, l2, l3] = *tidal_eigenvalues;
    let eigen_range = l1 - l3;
    let filamentarity = if eigen_range.abs() > 1e-15 {
        ((l2 - l3) / eigen_range).clamp(0.0, 1.0)
    } else {
        0.5 // isotropic → neutral
    };

    let web_environment = web_environment_from_eigenvalues(l1, l2, l3, tidal_threshold);

    Ok(GalacticField {
        concentration,
        density_pressure,
        structure_growth,
        cosmic_activity,
        chemical_complexity,
        filamentarity,
        web_environment,
    })
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

    // ── Scale 4 tests ──

    #[test]
    fn test_bridge_scale_4_sun() {
        // Sun: T=5772K, L=1.0, age=4.6e9yr, MS lifetime=~1e10yr, [Fe/H]=0.0
        let field = bridge_scale_4(5772.0, 1.0, 4.6e9, 1.0e10, 0.0).unwrap();
        assert!(field.lifecycle_fraction > 0.4 && field.lifecycle_fraction < 0.5);
        assert!(field.luminosity_intensity > 0.0 && field.luminosity_intensity < 1.0);
        assert!(field.thermal_temperament > 0.0 && field.thermal_temperament < 1.0);
        assert!((field.compositional_complexity - 0.8).abs() < 0.01); // [Fe/H]=0.0 → (0+2)/2.5=0.8
        assert_eq!(field.spectral_archetype, 4); // G-type
    }

    #[test]
    fn test_bridge_scale_4_hot_star() {
        // O-type: T=40000K, L=1e5, young
        let field = bridge_scale_4(40000.0, 1e5, 1e6, 3e6, -0.5).unwrap();
        assert!(field.thermal_temperament > 0.9);
        assert!(field.luminosity_intensity > 0.8);
        assert_eq!(field.spectral_archetype, 0); // O-type
    }

    #[test]
    fn test_bridge_scale_4_cool_star() {
        // M-type red dwarf: T=3000K, L=0.01, very long lived
        let field = bridge_scale_4(3000.0, 0.01, 1e9, 1e12, -1.0).unwrap();
        assert!(field.thermal_temperament < 0.15);
        assert!(field.luminosity_intensity < 0.01);
        assert_eq!(field.spectral_archetype, 6); // M-type
    }

    #[test]
    fn test_bridge_scale_4_bounds() {
        let field = bridge_scale_4(5772.0, 1.0, 4.6e9, 1.0e10, 0.0).unwrap();
        assert!((0.0..=1.0).contains(&field.lifecycle_fraction));
        assert!((0.0..=1.0).contains(&field.luminosity_intensity));
        assert!((0.0..=1.0).contains(&field.thermal_temperament));
        assert!((0.0..=1.0).contains(&field.compositional_complexity));
        assert!((0.0..=1.0).contains(&field.evolutionary_urgency));
        assert!(field.spectral_archetype <= 6);
    }

    #[test]
    fn test_bridge_scale_4_invalid_rejected() {
        assert!(bridge_scale_4(-1.0, 1.0, 1e9, 1e10, 0.0).is_err());
        assert!(bridge_scale_4(5772.0, -1.0, 1e9, 1e10, 0.0).is_err());
        assert!(bridge_scale_4(5772.0, 1.0, 1e9, -1.0, 0.0).is_err());
    }

    #[test]
    fn test_bridge_scale_4_serde() {
        let field = bridge_scale_4(5772.0, 1.0, 4.6e9, 1.0e10, 0.0).unwrap();
        let json = serde_json::to_string(&field).unwrap();
        let _back: StellarField = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_bridge_scale_4_evolutionary_urgency() {
        // Young star: low urgency
        let young = bridge_scale_4(5772.0, 1.0, 1e8, 1e10, 0.0).unwrap();
        // Old star: high urgency
        let old = bridge_scale_4(5772.0, 1.0, 9.5e9, 1e10, 0.0).unwrap();
        assert!(old.evolutionary_urgency > young.evolutionary_urgency);
    }

    fn milky_way_scale_5() -> GalacticField {
        // Milky Way-like: c~10, δ~0 (mean density), D=1 (today), SFR~0.01, 12+log(O/H)~8.7
        // Filament environment: λ₁=1.0, λ₂=0.3, λ₃=-0.5
        bridge_scale_5(10.0, 0.0, 1.0, 0.01, 8.7, &[1.0, 0.3, -0.5], 0.0).unwrap()
    }

    #[test]
    fn test_bridge_scale_5_milky_way() {
        let field = milky_way_scale_5();
        // Concentration: (10-3)/17 ≈ 0.41
        assert!(field.concentration > 0.3 && field.concentration < 0.5);
        // Density pressure: sigmoid(0) = 0.5
        assert!((field.density_pressure - 0.5).abs() < 0.01);
        // Structure growth: 1.0 (today)
        assert!((field.structure_growth - 1.0).abs() < 1e-10);
        // Chemical complexity: (8.7-7.5)/1.8 ≈ 0.67
        assert!(field.chemical_complexity > 0.6 && field.chemical_complexity < 0.7);
        // Web environment: 2 eigenvalues > 0 → filament (2)
        assert_eq!(field.web_environment, 2);
    }

    #[test]
    fn test_bridge_scale_5_void() {
        // Deep void: low c, δ=-0.8, early universe, low SFR, low metallicity
        let field = bridge_scale_5(5.0, -0.8, 0.3, 0.001, 7.8, &[-0.1, -0.3, -0.5], 0.0).unwrap();
        assert!(field.concentration < 0.2);
        assert!(field.density_pressure < 0.35);
        assert!(field.structure_growth < 0.35);
        assert_eq!(field.web_environment, 0); // void: 0 eigenvalues above threshold
    }

    #[test]
    fn test_bridge_scale_5_cluster_node() {
        // Galaxy cluster: high c, high δ, node environment
        let field = bridge_scale_5(15.0, 5.0, 1.0, 0.05, 9.0, &[2.0, 1.5, 0.5], 0.0).unwrap();
        assert!(field.concentration > 0.6);
        assert!(field.density_pressure > 0.9);
        assert_eq!(field.web_environment, 3); // node: all 3 above threshold
    }

    #[test]
    fn test_bridge_scale_5_cosmic_noon() {
        // z~2 cosmic noon: peak SFR
        let field = bridge_scale_5(8.0, 1.0, 0.5, 0.1, 8.5, &[0.5, 0.1, -0.2], 0.0).unwrap();
        assert!((field.cosmic_activity - 1.0).abs() < 0.01); // SFR=0.1 → peak
    }

    #[test]
    fn test_bridge_scale_5_bounds() {
        let field = milky_way_scale_5();
        assert!((0.0..=1.0).contains(&field.concentration));
        assert!((0.0..=1.0).contains(&field.density_pressure));
        assert!((0.0..=1.0).contains(&field.structure_growth));
        assert!((0.0..=1.0).contains(&field.cosmic_activity));
        assert!((0.0..=1.0).contains(&field.chemical_complexity));
        assert!((0.0..=1.0).contains(&field.filamentarity));
        assert!(field.web_environment <= 3);
    }

    #[test]
    fn test_bridge_scale_5_serde() {
        let field = milky_way_scale_5();
        let json = serde_json::to_string(&field).unwrap();
        let _back: GalacticField = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_negative_scale_rejected() {
        assert!(scale_coupling_qed(-1.0).is_err());
        assert!(scale_coupling_qcd(-1.0, 6).is_err());
    }
}
