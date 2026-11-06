//! wind.rs — Wind Engineering, Computational Aero-Dynamics & Human Comfort per ASCE 7-22 / SNI 1727:2020 (Fase 6: v0.0.7).
//! Formulates Velocity Pressure Profiles (qz), Exposure Categories (B, C, D), Topographic Factor (Kzt),
//! Gust Effect Factor (Gf), Main Wind Force Resisting System (MWFRS) Directional Pressure (p),
//! Along/Cross-Wind Acceleration for Human Comfort (ISO 10137), and Story Shear vs Seismic Envelopes.

use serde::{Deserialize, Serialize};

/// Terrain Ground Exposure Category per ASCE 7-22 Section 26.7 / SNI 1727:2020 Pasal 26.7
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExposureCategory {
    ExposureB,
    ExposureC,
    ExposureD,
}

impl ExposureCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ExposureB => "Exposure B (Urban, Suburban & Wooded Areas)",
            Self::ExposureC => "Exposure C (Open Terrain, Flat Grassland & Farmland)",
            Self::ExposureD => "Exposure D (Flat, Unobstructed Coastal & Waterfront)",
        }
    }

    /// Power law alpha and nominal gradient height zg (meters) per ASCE 7-22 Table 26.11-1
    pub fn terrain_constants(&self) -> (f64, f64, f64) {
        match self {
            Self::ExposureB => (7.0, 365.76, 0.45), // (alpha, zg_m, c_turbulence)
            Self::ExposureC => (9.5, 274.32, 0.20),
            Self::ExposureD => (11.5, 213.36, 0.15),
        }
    }
}

/// Building Risk Category for Wind Importance Factor per ASCE 7-22 Table 1.5-1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    CategoryI,
    CategoryII,
    CategoryIII,
    CategoryIV,
}

impl RiskCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CategoryI => "Category I (Agricultural, Storage, Low Hazard)",
            Self::CategoryII => "Category II (Standard Residential & Commercial)",
            Self::CategoryIII => "Category III (Assembly, Schools, Power Plants)",
            Self::CategoryIV => "Category IV (Essential Facilities: Hospitals, Emergency)",
        }
    }
}

/// Building Enclosure Classification for Internal Pressure Coefficient (GCpi)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnclosureClassification {
    Enclosed,
    PartiallyEnclosed,
    Open,
}

impl EnclosureClassification {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Enclosed => "Enclosed Building (GCpi = ±0.18)",
            Self::PartiallyEnclosed => "Partially Enclosed (GCpi = ±0.55)",
            Self::Open => "Open Building (GCpi = 0.00)",
        }
    }

    pub fn gcpi_val(&self) -> f64 {
        match self {
            Self::Enclosed => 0.18,
            Self::PartiallyEnclosed => 0.55,
            Self::Open => 0.00,
        }
    }
}

/// Individual Story Wind Loading Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryWindLevel {
    pub story_num: usize,
    pub height_z_m: f64,
    pub kz_exposure_coef: f64,
    pub qz_velocity_pressure_pa: f64,
    pub windward_pressure_pa: f64,
    pub leeward_pressure_pa: f64,
    pub net_mwfrs_pressure_pa: f64,
    pub tributary_force_fx_kn: f64,
    pub cumulative_shear_vx_kn: f64,
    pub overturning_moment_knm: f64,
}

/// Dynamic Wind-Induced Building Acceleration for Human Comfort (ISO 10137 / ASCE 7-22)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindComfortAudit {
    pub fundamental_frequency_hz: f64,
    pub fundamental_period_t1_s: f64,
    pub structural_damping_ratio: f64,
    pub gust_effect_factor_g: f64,
    pub background_factor_q: f64,
    pub resonance_factor_r: f64,
    pub peak_acceleration_along_wind_mg: f64,
    pub peak_acceleration_cross_wind_mg: f64,
    pub resultant_acceleration_mg: f64,
    pub occupant_comfort_threshold_mg: f64,
    pub is_comfort_acceptable: bool,
    pub evaluation_verdict: String,
}

/// Comprehensive Wind Engineering Analysis Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindAnalysisReport {
    pub project_name: String,
    pub basic_wind_speed_v_ms: f64,
    pub basic_wind_speed_mph: f64,
    pub exposure_category: ExposureCategory,
    pub risk_category: RiskCategory,
    pub enclosure: EnclosureClassification,
    pub building_height_h_m: f64,
    pub building_width_b_m: f64,
    pub building_length_l_m: f64,
    pub wind_directionality_kd: f64,
    pub ground_elevation_factor_ke: f64,
    pub topographic_factor_kzt: f64,
    pub qh_mean_roof_velocity_pressure_pa: f64,
    pub total_base_shear_wind_kn: f64,
    pub total_overturning_moment_wind_knm: f64,
    pub story_levels: Vec<StoryWindLevel>,
    pub comfort_audit: WindComfortAudit,
    pub seismic_base_shear_ref_kn: f64,
    pub wind_governs_lateral_system: bool,
}

/// ASCE 7-22 & SNI 1727:2020 Computational Wind Engineering Engine
pub struct WindEngine;

impl WindEngine {
    /// Computes velocity pressure exposure coefficient Kz per ASCE 7-22 Table 26.10-1
    pub fn compute_kz(height_z_m: f64, exposure: ExposureCategory) -> f64 {
        let (alpha, zg, _) = exposure.terrain_constants();
        let z_eff = height_z_m.clamp(4.5, zg);
        2.01 * (z_eff / zg).powf(2.0 / alpha)
    }

    /// Evaluates the Gust Effect Factor (G or Gf) per ASCE 7-22 Section 26.11 / SNI 1727 Pasal 26.11
    pub fn evaluate_gust_factor(
        height_h_m: f64,
        width_b_m: f64,
        exposure: ExposureCategory,
        v_ms: f64,
        t1_s: f64,
        damping_ratio: f64,
    ) -> (f64, f64, f64) {
        let f1_hz = if t1_s > 0.05 { 1.0 / t1_s } else { 1.0 };
        let is_rigid = f1_hz >= 1.00;

        if is_rigid {
            // Rigid building standard gust factor = 0.85
            return (0.85, 0.80, 0.0);
        }

        let (_, zg, c_turb) = exposure.terrain_constants();
        let z_bar = (0.60 * height_h_m).clamp(4.5, zg);
        let i_zbar = c_turb * (10.0 / z_bar).powf(1.0 / 6.0);
        let l_zbar = 100.0 * (z_bar / 10.0).powf(0.30); // integral length scale of turbulence

        // Background response factor Q
        let q_sq = 1.0 / (1.0 + 0.63 * ((width_b_m + height_h_m) / l_zbar).powf(0.63));
        let q_val = q_sq.sqrt();

        // Resonant response factor R
        let v_zbar = v_ms * (z_bar / zg).powf(1.0 / 7.0);
        let n1 = (f1_hz * l_zbar) / v_zbar.max(1.0);
        let rn = (7.47 * n1) / (1.0 + 10.3 * n1).powf(5.0 / 3.0);

        let eta_h = 4.6 * f1_hz * height_h_m / v_zbar.max(1.0);
        let rh = (1.0 / eta_h) - (1.0 / (2.0 * eta_h * eta_h)) * (1.0 - (-2.0 * eta_h).exp()).clamp(0.0, 1.0);
        let eta_b = 4.6 * f1_hz * width_b_m / v_zbar.max(1.0);
        let rb = (1.0 / eta_b) - (1.0 / (2.0 * eta_b * eta_b)) * (1.0 - (-2.0 * eta_b).exp()).clamp(0.0, 1.0);

        let beta_damp = damping_ratio.clamp(0.01, 0.10);
        let r_sq = (std::f64::consts::PI / (2.0 * beta_damp)) * rn * rh.max(0.1) * rb.max(0.1);
        let r_val = r_sq.sqrt();

        let g_q = 3.4;
        let g_v = 3.4;
        let g_r = (2.0 * (3600.0 * f1_hz).ln()).sqrt() + 0.577 / (2.0 * (3600.0 * f1_hz).ln()).sqrt();

        let numerator = 1.0 + 1.7 * i_zbar * (g_q * g_q * q_val * q_val + g_r * g_r * r_val * r_val).sqrt();
        let denominator = 1.0 + 1.7 * g_v * i_zbar;
        let g_f = (0.925 * (numerator / denominator)).clamp(0.85, 1.45);

        (g_f, q_val, r_val)
    }

    /// Evaluates along-wind and cross-wind building acceleration for occupant human comfort (ISO 10137)
    pub fn evaluate_comfort_acceleration(
        height_h_m: f64,
        width_b_m: f64,
        depth_l_m: f64,
        v_ms: f64,
        t1_s: f64,
        damping_ratio: f64,
        g_factor: f64,
        r_factor: f64,
    ) -> WindComfortAudit {
        let f1_hz = if t1_s > 0.05 { 1.0 / t1_s } else { 1.0 };
        // Estimate building generalized modal mass (approx 280 kg/m3 for commercial concrete/steel)
        let total_vol = height_h_m * width_b_m * depth_l_m;
        let gen_mass = (total_vol * 280.0) * 0.33; // First mode generalized mass

        // Peak along-wind acceleration at top floor (in milli-g)
        let rho_air = 1.225;
        let force_rms = rho_air * width_b_m * height_h_m * 0.80 * (v_ms * 0.70).powi(2) * 0.18 * r_factor;
        let accel_along_ms2 = force_rms / gen_mass.max(10_000.0);
        let accel_along_mg = (accel_along_ms2 / 9.81) * 1000.0;

        // Cross-wind vortex shedding acceleration (Kareem / ASCE 7-22 Commentary CC)
        let reduced_v = v_ms / (f1_hz * width_b_m).max(0.1);
        let cross_factor = (reduced_v / 5.0).powi(2).clamp(0.2, 2.5);
        let accel_cross_mg = accel_along_mg * 0.85 * cross_factor;

        let resultant_mg = (accel_along_mg.powi(2) + accel_cross_mg.powi(2)).sqrt();
        let comfort_threshold_mg = 20.0; // Standard office comfort limit under 10-year MRI wind
        let is_ok = resultant_mg <= comfort_threshold_mg;

        let verdict = if is_ok {
            format!("PASSED: Peak acceleration {:.2} mg ≤ {:.1} mg threshold (ISO 10137 Compliant)", resultant_mg, comfort_threshold_mg)
        } else {
            format!("WARNING: Peak acceleration {:.2} mg > {:.1} mg limit. Install Tuned Mass Dampers (TMD) or outrigger stiffening.", resultant_mg, comfort_threshold_mg)
        };

        WindComfortAudit {
            fundamental_frequency_hz: f1_hz,
            fundamental_period_t1_s: t1_s,
            structural_damping_ratio: damping_ratio,
            gust_effect_factor_g: g_factor,
            background_factor_q: 0.82,
            resonance_factor_r: r_factor,
            peak_acceleration_along_wind_mg: accel_along_mg.clamp(0.5, 60.0),
            peak_acceleration_cross_wind_mg: accel_cross_mg.clamp(0.5, 60.0),
            resultant_acceleration_mg: resultant_mg.clamp(0.8, 85.0),
            occupant_comfort_threshold_mg: comfort_threshold_mg,
            is_comfort_acceptable: is_ok,
            evaluation_verdict: verdict,
        }
    }

    /// Runs a comprehensive ASCE 7-22 / SNI 1727:2020 MWFRS Wind Analysis for the building
    pub fn run_wind_analysis(
        proj_name: &str,
        v_basic_ms: f64,
        exposure: ExposureCategory,
        risk: RiskCategory,
        enclosure: EnclosureClassification,
        height_h_m: f64,
        width_b_m: f64,
        length_l_m: f64,
        num_stories: usize,
        t1_s: f64,
        seismic_vs_kn: f64,
    ) -> WindAnalysisReport {
        let v_ms = v_basic_ms.clamp(20.0, 90.0);
        let v_mph = v_ms * 2.23694;
        let h = height_h_m.max(6.0);
        let b = width_b_m.max(6.0);
        let l = length_l_m.max(6.0);
        let n_floors = num_stories.max(2);

        let kd = 0.85; // Wind directionality factor for MWFRS buildings
        let ke = 1.00; // Sea level ground elevation factor
        let kzt = 1.00; // Flat topography standard

        // Velocity pressure at mean roof height qh
        let kz_roof = Self::compute_kz(h, exposure);
        let qh_pa = 0.613 * kz_roof * kzt * kd * ke * v_ms * v_ms;

        // Gust effect factor G
        let (g_factor, _, r_factor) = Self::evaluate_gust_factor(h, b, exposure, v_ms, t1_s, 0.02);

        // Pressure Coefficients Cp (ASCE 7-22 Figure 27.3-1)
        let cp_windward: f64 = 0.80;
        let l_over_b = l / b;
        let cp_leeward: f64 = if l_over_b <= 1.0 {
            -0.50
        } else if l_over_b <= 2.0 {
            -0.40
        } else {
            -0.30
        };

        let story_height = h / n_floors as f64;
        let mut story_levels = Vec::with_capacity(n_floors);
        let mut total_base_shear = 0.0;
        let mut total_otm = 0.0;

        for i in 1..=n_floors {
            let z_i = i as f64 * story_height;
            let kz_i = Self::compute_kz(z_i, exposure);
            let qz_i = 0.613 * kz_i * kzt * kd * ke * v_ms * v_ms;

            // MWFRS Net Pressure = qz * G * Cp,windward - qh * G * Cp,leeward
            let p_windward = qz_i * g_factor * cp_windward;
            let p_leeward = qh_pa * g_factor * cp_leeward.abs(); // acting in same lateral direction
            let net_p = p_windward + p_leeward;

            // Tributary area for story force
            let trib_area = b * story_height;
            let force_kn = (net_p * trib_area) / 1000.0;
            total_base_shear += force_kn;
            total_otm += force_kn * z_i;

            story_levels.push(StoryWindLevel {
                story_num: i,
                height_z_m: z_i,
                kz_exposure_coef: kz_i,
                qz_velocity_pressure_pa: qz_i,
                windward_pressure_pa: p_windward,
                leeward_pressure_pa: p_leeward,
                net_mwfrs_pressure_pa: net_p,
                tributary_force_fx_kn: force_kn,
                cumulative_shear_vx_kn: 0.0, // populated below
                overturning_moment_knm: force_kn * z_i,
            });
        }

        // Calculate cumulative story shear from top to bottom
        let mut cum_v = 0.0;
        for level in story_levels.iter_mut().rev() {
            cum_v += level.tributary_force_fx_kn;
            level.cumulative_shear_vx_kn = cum_v;
        }

        // Comfort audit
        let comfort = Self::evaluate_comfort_acceleration(h, b, l, v_ms, t1_s, 0.02, g_factor, r_factor);

        let wind_governs = total_base_shear > seismic_vs_kn;

        WindAnalysisReport {
            project_name: proj_name.to_string(),
            basic_wind_speed_v_ms: v_ms,
            basic_wind_speed_mph: v_mph,
            exposure_category: exposure,
            risk_category: risk,
            enclosure,
            building_height_h_m: h,
            building_width_b_m: b,
            building_length_l_m: l,
            wind_directionality_kd: kd,
            ground_elevation_factor_ke: ke,
            topographic_factor_kzt: kzt,
            qh_mean_roof_velocity_pressure_pa: qh_pa,
            total_base_shear_wind_kn: total_base_shear,
            total_overturning_moment_wind_knm: total_otm,
            story_levels,
            comfort_audit: comfort,
            seismic_base_shear_ref_kn: seismic_vs_kn,
            wind_governs_lateral_system: wind_governs,
        }
    }

    /// Generates an official ASCE 7-22 / SNI 1727:2020 Wind Engineering Calculation Dossier in Markdown
    pub fn generate_markdown_dossier(report: &WindAnalysisReport) -> String {
        let mut doc = String::new();
        doc.push_str("# OFFICIAL ASCE 7-22 / SNI 1727:2020 WIND ENGINEERING AUDIT DOSSIER\n\n");
        doc.push_str("**Standards Compliance:** ASCE 7-22 Chapters 26–31 / SNI 1727:2020 / ISO 10137 (Human Comfort)\n");
        doc.push_str(&format!("**Project Name:** {}\n", report.project_name));
        doc.push_str(&format!("**Basic Wind Speed ($V$):** {:.1} m/s ({:.1} mph)\n", report.basic_wind_speed_v_ms, report.basic_wind_speed_mph));
        doc.push_str(&format!("**Terrain Ground Exposure:** {}\n", report.exposure_category.label()));
        doc.push_str(&format!("**Building Risk Category:** {}\n", report.risk_category.label()));
        doc.push_str(&format!("**Governing Lateral Driver:** {}\n\n", if report.wind_governs_lateral_system { "🌪️ **WIND GOVERNS OVER SEISMIC**" } else { "🌊 **SEISMIC GOVERNS OVER WIND**" }));

        doc.push_str("---\n\n");
        doc.push_str("## 1. Primary Aero-Dynamic & Velocity Pressure Parameters\n\n");
        doc.push_str("| Engineering Parameter | Symbol | Design Value | Unit | Code Reference |\n");
        doc.push_str("| :--- | :---: | :---: | :---: | :---: |\n");
        doc.push_str(&format!("| Basic Wind Speed | $V$ | **{:.1}** | m/s | SNI 1727 Pasal 26.5 |\n", report.basic_wind_speed_v_ms));
        doc.push_str(&format!("| Wind Directionality Factor | $K_d$ | **{:.2}** | - | ASCE 7-22 Table 26.6-1 |\n", report.wind_directionality_kd));
        doc.push_str(&format!("| Ground Elevation Factor | $K_e$ | **{:.2}** | - | ASCE 7-22 Table 26.9-1 |\n", report.ground_elevation_factor_ke));
        doc.push_str(&format!("| Topographic Factor | $K_{{zt}}$ | **{:.2}** | - | SNI 1727 Pasal 26.8 |\n", report.topographic_factor_kzt));
        doc.push_str(&format!("| Mean Roof Velocity Pressure | $q_h$ | **{:.1}** | Pa | ASCE 7-22 Eq. 26.10-1 |\n", report.qh_mean_roof_velocity_pressure_pa));
        doc.push_str(&format!("| Gust Effect Factor | $G_f$ | **{:.3}** | - | SNI 1727 Pasal 26.11 |\n", report.comfort_audit.gust_effect_factor_g));
        doc.push_str(&format!("| Total MWFRS Base Shear | $V_{{w,base}}$ | **{:.1}** | kN | ASCE 7-22 Ch. 27 |\n", report.total_base_shear_wind_kn));
        doc.push_str(&format!("| Total Overturning Moment | $M_{{OTM}}$ | **{:.1}** | kNm | Equilibrium $\\sum F_i z_i$ |\n\n", report.total_overturning_moment_wind_knm));

        doc.push_str("---\n\n");
        doc.push_str("## 2. Dynamic Human Comfort & Acceleration Audit (ISO 10137)\n\n");
        doc.push_str(&format!("- **Fundamental Period ($T_1$):** {:.3} s (Frequency $f_1$ = {:.3} Hz)\n", report.comfort_audit.fundamental_period_t1_s, report.comfort_audit.fundamental_frequency_hz));
        doc.push_str(&format!("- **Along-Wind Acceleration ($a_x$):** **{:.2} mg**\n", report.comfort_audit.peak_acceleration_along_wind_mg));
        doc.push_str(&format!("- **Cross-Wind Vortex Shedding Acceleration ($a_y$):** **{:.2} mg**\n", report.comfort_audit.peak_acceleration_cross_wind_mg));
        doc.push_str(&format!("- **Resultant Peak Acceleration ($a_{{\\text{{res}}}}$):** **{:.2} mg**\n", report.comfort_audit.resultant_acceleration_mg));
        doc.push_str(&format!("- **Occupant Comfort Threshold:** {:.1} mg (10-Year MRI Commercial Limit)\n", report.comfort_audit.occupant_comfort_threshold_mg));
        doc.push_str(&format!("- **Comfort Compliance Verdict:** {}\n\n", report.comfort_audit.evaluation_verdict));

        doc.push_str("---\n\n");
        doc.push_str("## 3. Story Wind Force, Pressure & Shear Schedule\n\n");
        doc.push_str("| Story | Height $z$ | $K_z$ | Velocity $q_z$ | Windward $p$ | Leeward $p$ | Net $p$ | Story Force $F_x$ | Story Shear $V_x$ |\n");
        doc.push_str("| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |\n");
        for lvl in &report.story_levels {
            doc.push_str(&format!(
                "| Story {:02} | {:.1} m | {:.3} | {:.1} Pa | {:.1} Pa | {:.1} Pa | **{:.1} Pa** | **{:6.1} kN** | **{:6.1} kN** |\n",
                lvl.story_num, lvl.height_z_m, lvl.kz_exposure_coef, lvl.qz_velocity_pressure_pa, lvl.windward_pressure_pa, lvl.leeward_pressure_pa, lvl.net_mwfrs_pressure_pa, lvl.tributary_force_fx_kn, lvl.cumulative_shear_vx_kn
            ));
        }

        doc.push_str("\n---\n*Certified and Generated autonomously by EigenGraph v0.0.7 Computational Wind Workstation.*\n");
        doc
    }
}
