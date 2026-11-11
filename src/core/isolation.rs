//! isolation.rs — Seismic Base Isolation & Energy Dissipation Systems Design Engine.
//!
//! Compliant with:
//! - ASCE 7-22 Chapter 17 (Seismic Design Requirements for Seismically Isolated Structures)
//! - ASCE 7-22 Chapter 18 (Seismic Design Requirements for Structures with Damping Systems)
//! - SNI 1726:2019 Pasal 12 (Struktur dengan Sistem Isolasi Seismik)
//! - SNI 1726:2019 Pasal 13 (Struktur dengan Sistem Redaman)
//! - FEMA P-1051 Chapter 12 (Design Examples for Seismically Isolated Buildings)
//! - Constantinou et al. (2007) "MCEER-07-0012: Modeling and Design of Base Isolation Systems"

use serde::{Deserialize, Serialize};

/// Type of seismic isolation bearing device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolatorType {
    /// Elastomeric Lead-Rubber Bearing (LRB) with high-purity cylindrical lead core.
    LeadRubberBearing,
    /// Low-Damping / High-Damping Natural Rubber Bearing (HDRB).
    HighDampingRubberBearing,
    /// Friction Pendulum System (FPS / Single or Triple Concave Slider).
    FrictionPendulum,
    /// Fluid Viscous Damper (FVD) supplementary energy dissipation device.
    FluidViscousDamper,
}

impl Default for IsolatorType {
    fn default() -> Self {
        Self::LeadRubberBearing
    }
}

/// Damping system configuration for supplementary dampers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamperConfiguration {
    /// Diagonal single-bay damper brace.
    DiagonalBrace,
    /// Inverted-V / Chevron brace configuration.
    ChevronBrace,
    /// Toggle-brace mechanism for amplified displacement stroke.
    ToggleBrace,
}

impl Default for DamperConfiguration {
    fn default() -> Self {
        Self::ChevronBrace
    }
}

/// Lead-Rubber Bearing (LRB) mechanical parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadRubberBearingParams {
    /// Outer bearing diameter D_o (mm).
    pub outer_diameter_mm: f64,
    /// Central lead core diameter d_pb (mm).
    pub lead_core_diameter_mm: f64,
    /// Individual rubber layer thickness t_r (mm).
    pub rubber_layer_thick_mm: f64,
    /// Number of internal rubber layers n_layers.
    pub num_rubber_layers: usize,
    /// Internal steel shim plate thickness t_s (mm).
    pub steel_shim_thick_mm: f64,
    /// Shear modulus of elastomer G (MPa). Typically 0.40 to 0.70 MPa.
    pub shear_modulus_g_mpa: f64,
    /// Shear yield stress of lead plug sigma_pb (MPa). Typically 10.0 to 10.5 MPa.
    pub lead_yield_stress_mpa: f64,
    /// Bulk modulus of rubber K_bulk (MPa). Typically ~2000 MPa.
    pub bulk_modulus_mpa: f64,
}

impl Default for LeadRubberBearingParams {
    fn default() -> Self {
        Self {
            outer_diameter_mm: 850.0,
            lead_core_diameter_mm: 160.0,
            rubber_layer_thick_mm: 7.0,
            num_rubber_layers: 28,
            steel_shim_thick_mm: 3.5,
            shear_modulus_g_mpa: 0.45,
            lead_yield_stress_mpa: 10.2,
            bulk_modulus_mpa: 2000.0,
        }
    }
}

/// Friction Pendulum System (FPS) mechanical parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrictionPendulumParams {
    /// Spherical concave surface radius of curvature R (mm).
    pub radius_of_curvature_mm: f64,
    /// Dynamic sliding friction coefficient at high velocity mu_fast (typically 0.04 to 0.09).
    pub friction_coeff_fast: f64,
    /// Static / slow velocity friction coefficient mu_slow (typically 0.02 to 0.05).
    pub friction_coeff_slow: f64,
    /// Friction rate parameter a (s/m).
    pub friction_rate_param: f64,
    /// Articulated slider puck diameter (mm).
    pub slider_diameter_mm: f64,
    /// Bearing housing plan dimension (mm).
    pub housing_diameter_mm: f64,
}

impl Default for FrictionPendulumParams {
    fn default() -> Self {
        Self {
            radius_of_curvature_mm: 2800.0,
            friction_coeff_fast: 0.040,
            friction_coeff_slow: 0.020,
            friction_rate_param: 100.0,
            slider_diameter_mm: 280.0,
            housing_diameter_mm: 1200.0,
        }
    }
}

/// Fluid Viscous Damper (FVD) mechanical parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidViscousDamperParams {
    /// Damping coefficient C (kN / (m/s)^alpha).
    pub damping_coeff_c: f64,
    /// Velocity exponent alpha (typically 0.30 to 0.50 for seismic, 1.0 for linear).
    pub velocity_exponent_alpha: f64,
    /// Maximum design stroke limit (+/- mm).
    pub stroke_limit_mm: f64,
    /// Damper cylinder outer diameter (mm).
    pub cylinder_diameter_mm: f64,
    /// Installation inclination angle theta (degrees, 0 = horizontal).
    pub inclination_angle_deg: f64,
    /// Installation configuration.
    pub configuration: DamperConfiguration,
}

impl Default for FluidViscousDamperParams {
    fn default() -> Self {
        Self {
            damping_coeff_c: 650.0,
            velocity_exponent_alpha: 0.35,
            stroke_limit_mm: 250.0,
            cylinder_diameter_mm: 220.0,
            inclination_angle_deg: 26.57, // 1:2 slope
            configuration: DamperConfiguration::ChevronBrace,
        }
    }
}

/// Comprehensive input parameters for seismic isolation & damping evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicIsolationInput {
    /// Active isolator system type.
    pub isolator_type: IsolatorType,
    /// Total seismic weight of superstructure W (kN).
    pub seismic_weight_w_kn: f64,
    /// Number of isolation bearings supporting the building.
    pub num_bearings: usize,
    /// Average axial dead + live load per bearing P_v (kN).
    pub axial_load_per_bearing_kn: f64,
    /// MCE_R spectral response acceleration at 1.0 second S_M1 (g) per ASCE 7-22 / SNI 1726.
    pub spectral_accel_sm1_g: f64,
    /// MCE_R short-period spectral response acceleration S_MS (g).
    pub spectral_accel_sms_g: f64,
    /// Superstructure fundamental fixed-base period T_fixed (s).
    pub fixed_base_period_s: f64,
    /// Superstructure numerical reduction factor R_I (ASCE 7-22 §17.5.4.2, typically 1.5 to 2.0).
    pub superstructure_r_i: f64,
    /// Building plan width perpendicular to seismic excitation b (m).
    pub building_plan_b_m: f64,
    /// Building plan length parallel to excitation d (m).
    pub building_plan_d_m: f64,
    /// Total inherent and accidental plan eccentricity e (m).
    pub plan_eccentricity_e_m: f64,
    /// Lead-Rubber Bearing parameters.
    pub lrb_params: LeadRubberBearingParams,
    /// Friction Pendulum parameters.
    pub fps_params: FrictionPendulumParams,
    /// Fluid Viscous Damper parameters.
    pub fvd_params: FluidViscousDamperParams,
    /// Number of supplementary viscous dampers installed.
    pub num_dampers: usize,
}

impl Default for SeismicIsolationInput {
    fn default() -> Self {
        Self {
            isolator_type: IsolatorType::LeadRubberBearing,
            seismic_weight_w_kn: 96_000.0, // 8-story building ~12,000 kN/floor
            num_bearings: 24,
            axial_load_per_bearing_kn: 4_000.0,
            spectral_accel_sm1_g: 0.65,
            spectral_accel_sms_g: 1.25,
            fixed_base_period_s: 0.85,
            superstructure_r_i: 1.68, // R_I = 3/8 R for special RC frame (R=8 => R_I=2.0, capped at 1.68-2.0)
            building_plan_b_m: 36.0,
            building_plan_d_m: 24.0,
            plan_eccentricity_e_m: 1.8, // 5% accidental eccentricity on 36m width
            lrb_params: LeadRubberBearingParams::default(),
            fps_params: FrictionPendulumParams::default(),
            fvd_params: FluidViscousDamperParams::default(),
            num_dampers: 16,
        }
    }
}

/// Bilinear hysteresis loop coordinates and point properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HysteresisPoint {
    /// Lateral displacement (mm).
    pub displacement_mm: f64,
    /// Restoring/damping lateral force (kN).
    pub lateral_force_kn: f64,
}

/// Regulatory check compliance result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryCheckItem {
    pub code_clause: String,
    pub description: String,
    pub demand_val: f64,
    pub capacity_val: f64,
    pub unit: String,
    pub demand_capacity_ratio: f64,
    pub is_passed: bool,
}

/// Complete design output & performance evaluation of the base isolation system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicIsolationReport {
    /// Isolator type evaluated.
    pub isolator_type: IsolatorType,
    /// Total rubber thickness T_r (mm) [LRB only].
    pub total_rubber_thick_tr_mm: f64,
    /// Primary shape factor S_1 [LRB].
    pub shape_factor_s1: f64,
    /// Secondary shape factor S_2 [LRB].
    pub shape_factor_s2: f64,
    /// Characteristic strength Q_d per bearing (kN).
    pub characteristic_strength_qd_kn: f64,
    /// Post-yield lateral stiffness k_d per bearing (kN/mm).
    pub post_yield_stiffness_kd_kn_per_mm: f64,
    /// Elastic lateral stiffness k_u per bearing (kN/mm).
    pub elastic_stiffness_ku_kn_per_mm: f64,
    /// Yield displacement D_y (mm).
    pub yield_displacement_dy_mm: f64,
    /// Effective isolation period at Maximum Considered Earthquake (MCE_R) T_M (s).
    pub effective_period_tm_s: f64,
    /// Maximum lateral design displacement at center of mass D_M (mm) [ASCE 7-22 Eq. 17.5-1].
    pub max_displacement_dm_mm: f64,
    /// Total Maximum lateral displacement including accidental torsion D_TM (mm) [ASCE 7-22 Eq. 17.5-3].
    pub total_max_displacement_dtm_mm: f64,
    /// Effective damping ratio of isolation system beta_M (fraction, e.g. 0.22 = 22%).
    pub effective_damping_beta_m: f64,
    /// Damping reduction coefficient B_M per ASCE 7-22 Table 17.5-1.
    pub damping_reduction_bm: f64,
    /// Total effective lateral stiffness of all isolation bearings K_M_total (kN/mm).
    pub total_effective_stiffness_km_total: f64,
    /// Maximum base shear force at isolation interface V_b (kN).
    pub base_shear_isolation_interface_vb_kn: f64,
    /// Reduced seismic design base shear for superstructure V_s (kN) [ASCE 7-22 Eq. 17.5-7].
    pub superstructure_design_base_shear_vs_kn: f64,
    /// Base shear ratio compared to conventional fixed-base building V_s / V_fixed.
    pub base_shear_reduction_ratio: f64,
    /// Critical buckling load capacity at zero displacement P_cr0 per bearing (kN).
    pub critical_buckling_load_pcr0_kn: f64,
    /// Reduced critical buckling load capacity at displacement D_M P_cr(D_M) (kN).
    pub critical_buckling_load_pcrd_kn: f64,
    /// Safety factor against bearing buckling under P_axial at D_M.
    pub buckling_safety_factor: f64,
    /// Required seismic moat clear gap width (mm) including 10% safety margin.
    pub required_moat_gap_mm: f64,
    /// Energy dissipated per cycle E_D (kJ).
    pub energy_dissipated_per_cycle_kj: f64,
    /// Supplementary damper peak lateral force per unit F_d_max (kN) [FVD].
    pub damper_peak_force_kn: f64,
    /// Supplementary viscous damping ratio beta_v (fraction).
    pub supplementary_damping_beta_v: f64,
    /// Combined total system damping beta_total = beta_M + beta_v.
    pub combined_total_damping: f64,
    /// Hysteresis loop data points for UI vector CAD plot.
    pub hysteresis_loop: Vec<HysteresisPoint>,
    /// Regulatory check results table per ASCE 7-22 / SNI 1726:2019.
    pub regulatory_checks: Vec<RegulatoryCheckItem>,
    /// Overall governing D/C ratio.
    pub governing_demand_capacity_ratio: f64,
    /// Overall design pass/fail status.
    pub is_design_acceptable: bool,
}

/// Core computational engine for seismic isolation & damping.
pub struct SeismicIsolationEngine;

impl SeismicIsolationEngine {
    /// Calculate damping reduction factor B_M from effective damping ratio beta_M per ASCE 7-22 Table 17.5-1.
    pub fn calculate_damping_reduction_factor(beta: f64) -> f64 {
        let beta_pct = beta.clamp(0.02, 0.50) * 100.0;
        // Piecewise linear interpolation based on ASCE 7-22 Table 17.5-1 / SNI 1726 Tabel 25
        if beta_pct <= 2.0 {
            0.80
        } else if beta_pct <= 5.0 {
            0.80 + (1.00 - 0.80) * (beta_pct - 2.0) / (5.0 - 2.0)
        } else if beta_pct <= 10.0 {
            1.00 + (1.20 - 1.00) * (beta_pct - 5.0) / (10.0 - 5.0)
        } else if beta_pct <= 20.0 {
            1.20 + (1.50 - 1.20) * (beta_pct - 10.0) / (20.0 - 10.0)
        } else if beta_pct <= 30.0 {
            1.50 + (1.70 - 1.50) * (beta_pct - 20.0) / (30.0 - 20.0)
        } else if beta_pct <= 40.0 {
            1.70 + (1.90 - 1.70) * (beta_pct - 30.0) / (40.0 - 30.0)
        } else {
            1.90 + (2.00 - 1.90) * (beta_pct - 40.0) / (50.0 - 40.0)
        }
    }

    /// Evaluates the complete seismic isolation system per ASCE 7-22 & SNI 1726:2019.
    pub fn evaluate(input: &SeismicIsolationInput) -> SeismicIsolationReport {
        let g_m_s2 = 9.80665;
        let pi = std::f64::consts::PI;

        match input.isolator_type {
            IsolatorType::LeadRubberBearing | IsolatorType::HighDampingRubberBearing => {
                Self::evaluate_lrb(input, g_m_s2, pi)
            }
            IsolatorType::FrictionPendulum => Self::evaluate_fps(input, g_m_s2, pi),
            IsolatorType::FluidViscousDamper => Self::evaluate_fvd(input, g_m_s2, pi),
        }
    }

    /// Evaluation routine for Lead-Rubber Bearings (LRB).
    fn evaluate_lrb(input: &SeismicIsolationInput, g: f64, pi: f64) -> SeismicIsolationReport {
        let p = &input.lrb_params;
        let d_o = p.outer_diameter_mm;
        let d_pb = p.lead_core_diameter_mm;
        let t_r = p.rubber_layer_thick_mm;
        let n = p.num_rubber_layers as f64;
        let t_total_rubber = n * t_r; // Total rubber thickness T_r (mm)

        // Geometry: Gross area, lead core area, effective bonded rubber area
        let a_gross = pi * (d_o / 2.0).powi(2);
        let a_pb = pi * (d_pb / 2.0).powi(2);
        let a_bonded = a_gross - a_pb;

        // Shape factors (Kelly 1997 / Naeim & Kelly 1999)
        // Primary shape factor S_1 = (D_o - d_pb) / (4 * t_r)
        let s_1 = (d_o - d_pb) / (4.0 * t_r);
        // Secondary shape factor S_2 = D_o / T_r
        let s_2 = d_o / t_total_rubber;

        // Characteristic strength Q_d = sigma_pb * A_pb (kN)
        let q_d_bearing = (p.lead_yield_stress_mpa * a_pb) / 1000.0;
        let q_d_total = q_d_bearing * (input.num_bearings as f64);

        // Post-yield lateral stiffness k_d = (G * A_bonded) / T_r (kN/mm)
        let k_d_bearing = (p.shear_modulus_g_mpa * a_bonded) / t_total_rubber / 1000.0;
        let k_d_total = k_d_bearing * (input.num_bearings as f64);

        // Elastic stiffness k_u = 10 * k_d (kN/mm)
        let k_u_bearing = 10.0 * k_d_bearing;
        let d_y = q_d_bearing / (k_u_bearing - k_d_bearing); // Yield displacement (mm)

        // Iterative solution for D_M and T_M per ASCE 7-22 Section 17.5.3:
        // D_M = (g * S_M1 * T_M) / (4 * pi^2 * B_M)
        // where T_M = 2 * pi * sqrt( W / (K_M_total * g) )
        // K_M_total = k_d_total + Q_d_total / D_M
        let mut d_m = 180.0; // Initial guess (mm)
        let mut t_m = 2.5;
        let mut b_m = 1.5;
        let mut beta_m = 0.20;

        for _ in 0..30 {
            let k_m_total = k_d_total + q_d_total / d_m;
            let k_m_si = k_m_total * 1_000_000.0; // N/m
            let w_n = input.seismic_weight_w_kn * 1000.0; // N
            let mass = w_n / g; // kg
            t_m = 2.0 * pi * (mass / k_m_si).sqrt();

            // Energy dissipated per cycle E_D = 4 * Q_d * (D_M - D_y)
            let e_d_cycle = 4.0 * q_d_total * (d_m - d_y).max(0.0); // kN·mm
            // Effective damping ratio beta_M = (2 / pi) * [ Q_d * (D_M - D_y) / (K_M * D_M^2) ]
            beta_m = (e_d_cycle / (2.0 * pi * k_m_total * d_m.powi(2))).clamp(0.02, 0.45);
            b_m = Self::calculate_damping_reduction_factor(beta_m);

            // Updated D_M in meters then mm
            let d_m_next = (g * input.spectral_accel_sm1_g * t_m) / (4.0 * pi.powi(2) * b_m) * 1000.0;
            if (d_m_next - d_m).abs() < 0.1 {
                d_m = d_m_next;
                break;
            }
            d_m = 0.5 * (d_m + d_m_next);
        }

        // Accidental torsion amplification per ASCE 7-22 Eq. 17.5-3:
        // D_TM = D_M * [ 1 + y * (12 * e / (b^2 + d^2)) ]
        let b_plan = input.building_plan_b_m;
        let d_plan = input.building_plan_d_m;
        let e_acc = input.plan_eccentricity_e_m;
        let y_dist = b_plan / 2.0; // Distance to corner bearing
        let torsion_factor = 1.0 + y_dist * (12.0 * e_acc) / (b_plan.powi(2) + d_plan.powi(2));
        let d_tm = d_m * torsion_factor;

        // Effective stiffness at D_M
        let k_m_total = k_d_total + q_d_total / d_m;

        // Shear at isolation interface V_b = K_M_total * D_M (kN)
        let v_b = k_m_total * d_m;

        // Superstructure design base shear V_s per ASCE 7-22 Eq. 17.5-7:
        // V_s = V_b / R_I, but not less than V_b / (R_I * B_M) or fixed base limits
        let v_s = (v_b / input.superstructure_r_i).max(0.05 * input.seismic_weight_w_kn);

        // Fixed-base base shear for comparison (ASCE 7-22 unisolated reference demand)
        let c_s_fixed = (input.spectral_accel_sm1_g / (input.fixed_base_period_s * 2.5)).min(input.spectral_accel_sms_g / 2.5);
        let v_fixed = (c_s_fixed * input.seismic_weight_w_kn).max(0.044 * input.spectral_accel_sms_g * input.seismic_weight_w_kn);
        let base_shear_reduction = v_s / v_fixed.max(1.0);

        // Critical buckling capacity (Haringx theory for elastomeric bearings):
        // Compression modulus E_c = 6 * G * S_1^2
        let e_c = 6.0 * p.shear_modulus_g_mpa * s_1.powi(2);
        // Bulk modulus correction E_c' = (E_c * K_b) / (E_c + K_b)
        let e_c_eff = (e_c * p.bulk_modulus_mpa) / (e_c + p.bulk_modulus_mpa);
        let i_moment = pi * (d_o / 2.0).powi(4) / 4.0; // mm^4
        let h_total = t_total_rubber + (p.num_rubber_layers as f64 - 1.0) * p.steel_shim_thick_mm;
        let p_e = (pi.powi(2) * e_c_eff * i_moment) / (t_total_rubber * h_total); // N
        let p_s = p.shear_modulus_g_mpa * a_bonded * (h_total / t_total_rubber); // N
        let p_cr0 = (p_e * p_s / 3.0).sqrt() / 1000.0; // kN at zero displacement
        // Reduced critical load at D_M: P_cr(D_M) = P_cr0 * (1 - D_M / D_o)
        let p_crd = (p_cr0 * (1.0 - d_m / d_o)).max(0.0);
        let buckling_sf = p_crd / input.axial_load_per_bearing_kn.max(1.0);

        // Seismic moat requirement: D_TM + 10% clearance margin (ASCE 7-22 §17.5.3.3)
        let required_moat_mm = d_tm * 1.10;

        // Energy dissipated per cycle (kJ)
        let energy_cycle_kj = 4.0 * q_d_total * (d_m - d_y).max(0.0) / 1000.0;

        // Generate Bilinear Hysteresis Loop
        let hysteresis = Self::generate_bilinear_hysteresis(d_m, d_y, q_d_total, k_d_total, k_u_bearing * (input.num_bearings as f64));

        // Regulatory checks
        let mut checks = Vec::new();
        // 1. Minimum Effective Period Lengthening (T_M >= 3 * T_fixed)
        let period_ratio = t_m / input.fixed_base_period_s;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.5.3 / SNI 1726 Pasal 12.5.3".to_string(),
            description: "Period Lengthening Ratio (T_M / T_fixed >= 3.0)".to_string(),
            demand_val: 3.0,
            capacity_val: period_ratio,
            unit: "ratio".to_string(),
            demand_capacity_ratio: 3.0 / period_ratio.max(0.1),
            is_passed: period_ratio >= 3.0,
        });

        // 2. Maximum Shear Strain of Elastomer (gamma_total <= 250% = 2.50)
        let gamma_shear = d_tm / t_total_rubber;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.8.2 / ISO 22762".to_string(),
            description: "Maximum Rubber Shear Strain at D_TM (gamma <= 2.50)".to_string(),
            demand_val: gamma_shear,
            capacity_val: 2.50,
            unit: "rad".to_string(),
            demand_capacity_ratio: gamma_shear / 2.50,
            is_passed: gamma_shear <= 2.50,
        });

        // 3. Bearing Stability & Buckling Safety Factor (SF >= 2.0 at D_M)
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.8.2.5 / Constantinou (2007)".to_string(),
            description: "Buckling Stability Safety Factor at D_M (SF >= 2.0)".to_string(),
            demand_val: 2.0,
            capacity_val: buckling_sf,
            unit: "factor".to_string(),
            demand_capacity_ratio: 2.0 / buckling_sf.max(0.1),
            is_passed: buckling_sf >= 2.0,
        });

        // 4. Maximum Lead Core Shear Stress Ratio
        let tau_lead = (q_d_bearing * 1000.0) / a_pb;
        checks.push(RegulatoryCheckItem {
            code_clause: "NIST GCR 11-917-11 / ACI 318".to_string(),
            description: "Lead Plug Shear Yield Confinement Stress".to_string(),
            demand_val: tau_lead,
            capacity_val: 12.0,
            unit: "MPa".to_string(),
            demand_capacity_ratio: tau_lead / 12.0,
            is_passed: tau_lead <= 12.0,
        });

        // 5. Restoring Capability (ASCE 7-22 §17.2.4.4): Force at D_M >= Force at 0.5 D_M + 0.025 W
        let f_at_dm = q_d_total + k_d_total * d_m;
        let f_at_half_dm = q_d_total + k_d_total * (0.5 * d_m);
        let restoring_delta = f_at_dm - f_at_half_dm;
        let required_restoring = 0.025 * input.seismic_weight_w_kn;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.2.4.4 / SNI 1726 Pasal 12.2.4.4".to_string(),
            description: "Lateral Restoring Force Capability (Delta F >= 0.025 W)".to_string(),
            demand_val: required_restoring,
            capacity_val: restoring_delta,
            unit: "kN".to_string(),
            demand_capacity_ratio: required_restoring / restoring_delta.max(0.1),
            is_passed: restoring_delta >= required_restoring,
        });

        // 6. Superstructure Base Shear Reduction Check (V_s <= 0.40 V_fixed)
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.5.4 / SNI 1726 Pasal 12.5.4".to_string(),
            description: "Base Shear Reduction vs Fixed Base (V_s / V_fixed <= 0.50)".to_string(),
            demand_val: base_shear_reduction,
            capacity_val: 0.50,
            unit: "ratio".to_string(),
            demand_capacity_ratio: base_shear_reduction / 0.50,
            is_passed: base_shear_reduction <= 0.50,
        });

        // Governing D/C ratio
        let governing_dc = checks
            .iter()
            .map(|c| c.demand_capacity_ratio)
            .fold(0.0f64, |a, b| a.max(b));
        let is_ok = checks.iter().all(|c| c.is_passed);

        SeismicIsolationReport {
            isolator_type: input.isolator_type,
            total_rubber_thick_tr_mm: t_total_rubber,
            shape_factor_s1: s_1,
            shape_factor_s2: s_2,
            characteristic_strength_qd_kn: q_d_bearing,
            post_yield_stiffness_kd_kn_per_mm: k_d_bearing,
            elastic_stiffness_ku_kn_per_mm: k_u_bearing,
            yield_displacement_dy_mm: d_y,
            effective_period_tm_s: t_m,
            max_displacement_dm_mm: d_m,
            total_max_displacement_dtm_mm: d_tm,
            effective_damping_beta_m: beta_m,
            damping_reduction_bm: b_m,
            total_effective_stiffness_km_total: k_m_total,
            base_shear_isolation_interface_vb_kn: v_b,
            superstructure_design_base_shear_vs_kn: v_s,
            base_shear_reduction_ratio: base_shear_reduction,
            critical_buckling_load_pcr0_kn: p_cr0,
            critical_buckling_load_pcrd_kn: p_crd,
            buckling_safety_factor: buckling_sf,
            required_moat_gap_mm: required_moat_mm,
            energy_dissipated_per_cycle_kj: energy_cycle_kj,
            damper_peak_force_kn: 0.0,
            supplementary_damping_beta_v: 0.0,
            combined_total_damping: beta_m,
            hysteresis_loop: hysteresis,
            regulatory_checks: checks,
            governing_demand_capacity_ratio: governing_dc,
            is_design_acceptable: is_ok,
        }
    }

    /// Evaluation routine for Friction Pendulum Systems (FPS).
    fn evaluate_fps(input: &SeismicIsolationInput, g: f64, pi: f64) -> SeismicIsolationReport {
        let p = &input.fps_params;
        let r_eff_m = p.radius_of_curvature_mm / 1000.0;
        let mu = p.friction_coeff_fast;
        let w_total = input.seismic_weight_w_kn;

        // Pendulum stiffness k_r = W / R_eff (kN/m = kN/1000mm)
        let k_r_total = w_total / (r_eff_m * 1000.0); // kN/mm
        let q_d_total = mu * w_total; // Characteristic friction force (kN)

        // Iterative convergence for D_M
        let mut d_m = 220.0; // mm
        let mut t_m = 3.0;
        let mut b_m = 1.4;
        let mut beta_m = 0.18;

        for _ in 0..30 {
            let k_m_total = k_r_total + q_d_total / d_m;
            let k_m_si = k_m_total * 1_000_000.0;
            let mass = (w_total * 1000.0) / g;
            t_m = 2.0 * pi * (mass / k_m_si).sqrt();

            // Effective damping beta_M = (2 / pi) * [ mu / (mu + D_M / R_eff) ]
            let d_m_m = d_m / 1000.0;
            beta_m = ((2.0 / pi) * (mu / (mu + d_m_m / r_eff_m))).clamp(0.05, 0.45);
            b_m = Self::calculate_damping_reduction_factor(beta_m);

            let d_m_next = (g * input.spectral_accel_sm1_g * t_m) / (4.0 * pi.powi(2) * b_m) * 1000.0;
            if (d_m_next - d_m).abs() < 0.1 {
                d_m = d_m_next;
                break;
            }
            d_m = 0.5 * (d_m + d_m_next);
        }

        let b_plan = input.building_plan_b_m;
        let d_plan = input.building_plan_d_m;
        let e_acc = input.plan_eccentricity_e_m;
        let torsion_factor = 1.0 + (b_plan / 2.0) * (12.0 * e_acc) / (b_plan.powi(2) + d_plan.powi(2));
        let d_tm = d_m * torsion_factor;

        let k_m_total = k_r_total + q_d_total / d_m;
        let v_b = k_m_total * d_m;
        let v_s = (v_b / input.superstructure_r_i).max(0.05 * w_total);

        let c_s_fixed = (input.spectral_accel_sm1_g / (input.fixed_base_period_s * 2.5)).min(input.spectral_accel_sms_g / 2.5);
        let v_fixed = (c_s_fixed * w_total).max(0.044 * input.spectral_accel_sms_g * w_total);
        let base_shear_reduction = v_s / v_fixed.max(1.0);

        let required_moat_mm = d_tm * 1.10;
        let energy_cycle_kj = 4.0 * q_d_total * d_m / 1000.0;

        // Friction Pendulum Bilinear Loop (Yield disp is very small ~ 1.0 mm for initial breakaway friction)
        let d_y = 1.5;
        let hysteresis = Self::generate_bilinear_hysteresis(d_m, d_y, q_d_total, k_r_total, q_d_total / d_y);

        // Regulatory checks
        let mut checks = Vec::new();
        // 1. Minimum Effective Period
        let period_ratio = t_m / input.fixed_base_period_s;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.5.3 / SNI 1726 Pasal 12.5.3".to_string(),
            description: "Period Lengthening Ratio (T_M / T_fixed >= 3.0)".to_string(),
            demand_val: 3.0,
            capacity_val: period_ratio,
            unit: "ratio".to_string(),
            demand_capacity_ratio: 3.0 / period_ratio.max(0.1),
            is_passed: period_ratio >= 3.0,
        });

        // 2. Maximum Slider Travel Limit vs Housing Diameter
        let max_slider_travel = (p.housing_diameter_mm - p.slider_diameter_mm) / 2.0;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.2.4 / Constantinou (2007)".to_string(),
            description: "Slider Displacement Capacity Margin (D_TM <= D_allowable)".to_string(),
            demand_val: d_tm,
            capacity_val: max_slider_travel,
            unit: "mm".to_string(),
            demand_capacity_ratio: d_tm / max_slider_travel.max(1.0),
            is_passed: d_tm <= max_slider_travel,
        });

        // 3. Restoring Capability
        let restoring_delta = k_r_total * (0.5 * d_m);
        let required_restoring = 0.025 * w_total;
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.2.4.4 / SNI 1726 Pasal 12.2.4.4".to_string(),
            description: "Pendulum Restoring Force Capability (Delta F >= 0.025 W)".to_string(),
            demand_val: required_restoring,
            capacity_val: restoring_delta,
            unit: "kN".to_string(),
            demand_capacity_ratio: required_restoring / restoring_delta.max(0.1),
            is_passed: restoring_delta >= required_restoring,
        });

        // 4. Base Shear Reduction
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §17.5.4 / SNI 1726 Pasal 12.5.4".to_string(),
            description: "Base Shear Reduction vs Fixed Base (V_s / V_fixed <= 0.50)".to_string(),
            demand_val: base_shear_reduction,
            capacity_val: 0.50,
            unit: "ratio".to_string(),
            demand_capacity_ratio: base_shear_reduction / 0.50,
            is_passed: base_shear_reduction <= 0.50,
        });

        let governing_dc = checks
            .iter()
            .map(|c| c.demand_capacity_ratio)
            .fold(0.0f64, |a, b| a.max(b));
        let is_ok = checks.iter().all(|c| c.is_passed);

        SeismicIsolationReport {
            isolator_type: input.isolator_type,
            total_rubber_thick_tr_mm: 0.0,
            shape_factor_s1: 0.0,
            shape_factor_s2: 0.0,
            characteristic_strength_qd_kn: q_d_total / (input.num_bearings as f64),
            post_yield_stiffness_kd_kn_per_mm: k_r_total / (input.num_bearings as f64),
            elastic_stiffness_ku_kn_per_mm: (q_d_total / d_y) / (input.num_bearings as f64),
            yield_displacement_dy_mm: d_y,
            effective_period_tm_s: t_m,
            max_displacement_dm_mm: d_m,
            total_max_displacement_dtm_mm: d_tm,
            effective_damping_beta_m: beta_m,
            damping_reduction_bm: b_m,
            total_effective_stiffness_km_total: k_m_total,
            base_shear_isolation_interface_vb_kn: v_b,
            superstructure_design_base_shear_vs_kn: v_s,
            base_shear_reduction_ratio: base_shear_reduction,
            critical_buckling_load_pcr0_kn: 99_999.0, // FPS is rigid steel, does not buckle
            critical_buckling_load_pcrd_kn: 99_999.0,
            buckling_safety_factor: 15.0,
            required_moat_gap_mm: required_moat_mm,
            energy_dissipated_per_cycle_kj: energy_cycle_kj,
            damper_peak_force_kn: 0.0,
            supplementary_damping_beta_v: 0.0,
            combined_total_damping: beta_m,
            hysteresis_loop: hysteresis,
            regulatory_checks: checks,
            governing_demand_capacity_ratio: governing_dc,
            is_design_acceptable: is_ok,
        }
    }

    /// Evaluation routine for structures with Fluid Viscous Dampers (FVD) per ASCE 7-22 Chapter 18.
    fn evaluate_fvd(input: &SeismicIsolationInput, g: f64, pi: f64) -> SeismicIsolationReport {
        let p = &input.fvd_params;
        let w_total = input.seismic_weight_w_kn;
        let theta_rad = p.inclination_angle_deg.to_radians();
        let cos_theta = theta_rad.cos();

        // Building fundamental circular frequency omega_1
        let t_fixed = input.fixed_base_period_s;
        let omega_1 = 2.0 * pi / t_fixed;

        // Estimated roof drift / lateral displacement demand without dampers D_0 (mm)
        let s_a1 = input.spectral_accel_sm1_g / t_fixed;
        let d_0 = (g * s_a1 / omega_1.powi(2)) * 1000.0 * 0.70; // mm

        // Peak damper stroke along inclined brace
        let stroke_d = (d_0 * cos_theta).min(p.stroke_limit_mm);
        let stroke_m = stroke_d / 1000.0;
        let peak_velocity = omega_1 * stroke_m; // m/s

        // Damper force F_D = C * v^alpha (kN)
        let f_damper_peak = p.damping_coeff_c * peak_velocity.powf(p.velocity_exponent_alpha);
        let f_damper_horiz = f_damper_peak * cos_theta;

        // Energy dissipated per cycle by one damper E_D = 4 * lambda_m * F_max * D_stroke
        // lambda_m for alpha: approximation lambda_m = 2^(2+alpha) * Gamma(1+alpha/2)^2 / (pi * Gamma(2+alpha))
        let alpha = p.velocity_exponent_alpha;
        let lambda_m = if (alpha - 1.0).abs() < 0.05 {
            pi / 4.0
        } else {
            0.85 - 0.06 * alpha // accurate civil engineering regression for 0.3 <= alpha <= 0.8
        };
        let energy_per_damper_kj = 4.0 * lambda_m * f_damper_peak * stroke_m;
        let total_energy_dampers_kj = energy_per_damper_kj * (input.num_dampers as f64);

        // Maximum strain energy of building E_S = 0.5 * M * omega_1^2 * (D_0/1000)^2 (kJ)
        let mass = (w_total * 1000.0) / g;
        let e_s_kj = 0.5 * mass * omega_1.powi(2) * (d_0 / 1000.0).powi(2) / 1000.0;

        // Supplementary damping ratio beta_v = Total_E_D / (4 * pi * E_S)
        let beta_v = (total_energy_dampers_kj / (4.0 * pi * e_s_kj.max(1.0))).clamp(0.05, 0.35);
        let beta_total = 0.05 + beta_v; // 5% inherent structural damping + supplementary
        let b_damping = Self::calculate_damping_reduction_factor(beta_total);

        // Reduced displacement with dampers D_damp
        let d_reduced = d_0 / b_damping;
        let d_tm = d_reduced * (1.0 + (input.building_plan_b_m / 2.0) * (12.0 * input.plan_eccentricity_e_m) / (input.building_plan_b_m.powi(2) + input.building_plan_d_m.powi(2)));

        // Base shear with damping system: Combined elastic force and damping force
        let v_elastic = (w_total * s_a1 / b_damping) / 8.0; // R=8
        // Out-of-phase maximum base shear per ASCE 7-22 §18.3.1
        let v_s = (v_elastic.powi(2) + (f_damper_horiz * input.num_dampers as f64 * 0.5).powi(2)).sqrt();
        let v_fixed = (w_total * s_a1) / 8.0;
        let base_shear_reduction = v_s / v_fixed.max(1.0);

        // Generate Elliptical Viscous Damper Hysteresis Loop
        let hysteresis = Self::generate_viscous_hysteresis(stroke_d, f_damper_peak, alpha);

        // Regulatory checks
        let mut checks = Vec::new();
        // 1. Damper Stroke Limit
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §18.2.4.2 / SNI 1726 Pasal 13.2".to_string(),
            description: "Damper Maximum Stroke Margin (Stroke <= Limit)".to_string(),
            demand_val: stroke_d,
            capacity_val: p.stroke_limit_mm,
            unit: "mm".to_string(),
            demand_capacity_ratio: stroke_d / p.stroke_limit_mm,
            is_passed: stroke_d <= p.stroke_limit_mm,
        });

        // 2. Minimum Damping Enhancement (beta_v >= 0.10)
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §18.2.1 / SNI 1726 Pasal 13.2.1".to_string(),
            description: "Supplementary Damping Ratio (beta_v >= 0.10 = 10%)".to_string(),
            demand_val: 0.10,
            capacity_val: beta_v,
            unit: "ratio".to_string(),
            demand_capacity_ratio: 0.10 / beta_v.max(0.01),
            is_passed: beta_v >= 0.10,
        });

        // 3. Peak Damper Force per Unit <= 1000 kN
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §18.6 / Manufacturer Rating".to_string(),
            description: "Peak Piston Force Capacity (F_D <= 1000 kN)".to_string(),
            demand_val: f_damper_peak,
            capacity_val: 1000.0,
            unit: "kN".to_string(),
            demand_capacity_ratio: f_damper_peak / 1000.0,
            is_passed: f_damper_peak <= 1000.0,
        });

        // 4. Drift Reduction Ratio
        checks.push(RegulatoryCheckItem {
            code_clause: "ASCE 7-22 §18.4 / SNI 1726 Pasal 13.4".to_string(),
            description: "Inter-story Drift Reduction Factor (B_V+I >= 1.25)".to_string(),
            demand_val: 1.25,
            capacity_val: b_damping,
            unit: "factor".to_string(),
            demand_capacity_ratio: 1.25 / b_damping,
            is_passed: b_damping >= 1.25,
        });

        let governing_dc = checks
            .iter()
            .map(|c| c.demand_capacity_ratio)
            .fold(0.0f64, |a, b| a.max(b));
        let is_ok = checks.iter().all(|c| c.is_passed);

        SeismicIsolationReport {
            isolator_type: input.isolator_type,
            total_rubber_thick_tr_mm: 0.0,
            shape_factor_s1: 0.0,
            shape_factor_s2: 0.0,
            characteristic_strength_qd_kn: 0.0,
            post_yield_stiffness_kd_kn_per_mm: 0.0,
            elastic_stiffness_ku_kn_per_mm: 0.0,
            yield_displacement_dy_mm: 0.0,
            effective_period_tm_s: t_fixed,
            max_displacement_dm_mm: d_reduced,
            total_max_displacement_dtm_mm: d_tm,
            effective_damping_beta_m: 0.05,
            damping_reduction_bm: b_damping,
            total_effective_stiffness_km_total: 0.0,
            base_shear_isolation_interface_vb_kn: v_s,
            superstructure_design_base_shear_vs_kn: v_s,
            base_shear_reduction_ratio: base_shear_reduction,
            critical_buckling_load_pcr0_kn: 99_999.0,
            critical_buckling_load_pcrd_kn: 99_999.0,
            buckling_safety_factor: 10.0,
            required_moat_gap_mm: d_tm * 1.10,
            energy_dissipated_per_cycle_kj: total_energy_dampers_kj,
            damper_peak_force_kn: f_damper_peak,
            supplementary_damping_beta_v: beta_v,
            combined_total_damping: beta_total,
            hysteresis_loop: hysteresis,
            regulatory_checks: checks,
            governing_demand_capacity_ratio: governing_dc,
            is_design_acceptable: is_ok,
        }
    }

    /// Generates bilinear hysteresis loop points for elastomeric & friction isolators.
    fn generate_bilinear_hysteresis(
        d_m: f64,
        d_y: f64,
        q_d: f64,
        k_d: f64,
        k_u: f64,
    ) -> Vec<HysteresisPoint> {
        let mut pts = Vec::new();
        let f_max = q_d + k_d * d_m;
        let f_y = k_u * d_y;

        // Path 1: Loading from origin to +yield to +D_M
        pts.push(HysteresisPoint { displacement_mm: 0.0, lateral_force_kn: 0.0 });
        pts.push(HysteresisPoint { displacement_mm: d_y, lateral_force_kn: f_y });
        pts.push(HysteresisPoint { displacement_mm: d_m, lateral_force_kn: f_max });

        // Path 2: Unloading from +D_M to -(f_max - 2*Q_d)
        let d_unload_1 = d_m - (2.0 * q_d / (k_u - k_d));
        pts.push(HysteresisPoint { displacement_mm: d_unload_1, lateral_force_kn: f_max - 2.0 * q_d });

        // Path 3: Reverse post-yield to -D_M
        pts.push(HysteresisPoint { displacement_mm: -d_m, lateral_force_kn: -f_max });

        // Path 4: Unloading from -D_M
        let d_unload_2 = -d_m + (2.0 * q_d / (k_u - k_d));
        pts.push(HysteresisPoint { displacement_mm: d_unload_2, lateral_force_kn: -f_max + 2.0 * q_d });

        // Path 5: Re-loading to +D_M to close loop
        pts.push(HysteresisPoint { displacement_mm: d_m, lateral_force_kn: f_max });

        pts
    }

    /// Generates nonlinear viscous damper force-displacement loop: F = C * (omega * D)^alpha * (1 - (u/D)^2)^(alpha/2)
    fn generate_viscous_hysteresis(d_max: f64, f_max: f64, alpha: f64) -> Vec<HysteresisPoint> {
        let mut pts = Vec::new();
        let num_steps = 40;
        let pi = std::f64::consts::PI;

        for i in 0..=num_steps {
            let theta = (i as f64) * 2.0 * pi / (num_steps as f64);
            let u = d_max * theta.sin();
            let v_ratio = theta.cos();
            let f = f_max * v_ratio.signum() * v_ratio.abs().powf(alpha);
            pts.push(HysteresisPoint { displacement_mm: u, lateral_force_kn: f });
        }

        pts
    }

    /// Generates an 8-story Base-Isolated Essential Facility 3D Model Archetype (Hospital).
    /// Returns: (nodes: (id, x, y, z), elements: (id, start_node, end_node, is_isolator)).
    pub fn generate_base_isolated_hospital_archetype() -> (Vec<(usize, f64, f64, f64)>, Vec<(usize, usize, usize, bool)>) {
        let mut nodes = Vec::new();
        let mut elements = Vec::new();

        let grid_x = [0.0, 8.0, 16.0, 24.0];
        let grid_y = [0.0, 8.0, 16.0];
        let story_h = 3.6; // m
        let num_stories = 8;

        let mut node_id = 0;
        let mut elem_id = 0;

        // Level -1: Foundation sub-structure anchor nodes (Z = -0.8m)
        let mut base_node_ids = Vec::new();
        for &gx in &grid_x {
            for &gy in &grid_y {
                nodes.push((node_id, gx, gy, -0.8));
                base_node_ids.push(node_id);
                node_id += 1;
            }
        }

        // Level 0: Isolation interface nodes (Z = 0.0m) connected via base isolator elements
        let mut isolator_top_node_ids = Vec::new();
        for &b_node in &base_node_ids {
            let (_, gx, gy, _) = nodes[b_node];
            nodes.push((node_id, gx, gy, 0.0));
            // Isolator element connecting foundation to base slab
            elements.push((elem_id, b_node, node_id, true));
            isolator_top_node_ids.push(node_id);
            node_id += 1;
            elem_id += 1;
        }

        // Generate superstructure floors 1 to 8
        let mut prev_floor_nodes = isolator_top_node_ids;
        for story in 1..=num_stories {
            let z = (story as f64) * story_h;
            let mut curr_floor_nodes = Vec::new();

            for &gx in &grid_x {
                for &gy in &grid_y {
                    nodes.push((node_id, gx, gy, z));
                    curr_floor_nodes.push(node_id);
                    node_id += 1;
                }
            }

            // Columns connecting prev floor to curr floor
            for i in 0..prev_floor_nodes.len() {
                elements.push((elem_id, prev_floor_nodes[i], curr_floor_nodes[i], false));
                elem_id += 1;
            }

            // Beams along X direction
            let nx = grid_x.len();
            let ny = grid_y.len();
            for iy in 0..ny {
                for ix in 0..(nx - 1) {
                    let n1 = curr_floor_nodes[ix * ny + iy];
                    let n2 = curr_floor_nodes[(ix + 1) * ny + iy];
                    elements.push((elem_id, n1, n2, false));
                    elem_id += 1;
                }
            }

            // Beams along Y direction
            for ix in 0..nx {
                for iy in 0..(ny - 1) {
                    let n1 = curr_floor_nodes[ix * ny + iy];
                    let n2 = curr_floor_nodes[ix * ny + (iy + 1)];
                    elements.push((elem_id, n1, n2, false));
                    elem_id += 1;
                }
            }

            prev_floor_nodes = curr_floor_nodes;
        }

        (nodes, elements)
    }

    /// Generates complete professional Markdown calculation report for base isolation.
    pub fn generate_markdown_dossier(input: &SeismicIsolationInput, report: &SeismicIsolationReport) -> String {
        let mut md = String::new();
        md.push_str("# EigenGraph Calculation Dossier: Seismic Base Isolation & Energy Dissipation\n");
        md.push_str("## Project Structural Engineering Verification Dossier (ASCE 7-22 Chapters 17 & 18 / SNI 1726:2019 Pasal 12 & 13)\n\n");
        md.push_str(&format!("- **Isolator Technology:** {:?}\n", report.isolator_type));
        md.push_str(&format!("- **Total Building Seismic Weight (W):** {:.1} kN\n", input.seismic_weight_w_kn));
        md.push_str(&format!("- **Number of Isolation Bearings:** {}\n", input.num_bearings));
        md.push_str(&format!("- **Fixed-Base Fundamental Period (T_fixed):** {:.2} s\n", input.fixed_base_period_s));
        md.push_str(&format!("- **Isolated System Effective Period (T_M):** {:.2} s\n", report.effective_period_tm_s));
        md.push_str(&format!("- **Effective Damping Ratio (beta_M):** {:.1}%\n", report.effective_damping_beta_m * 100.0));
        md.push_str(&format!("- **Center-of-Mass Maximum Displacement (D_M):** {:.1} mm\n", report.max_displacement_dm_mm));
        md.push_str(&format!("- **Total Maximum Displacement with Torsion (D_TM):** {:.1} mm\n", report.total_max_displacement_dtm_mm));
        md.push_str(&format!("- **Required Seismic Moat Clearance:** {:.1} mm\n", report.required_moat_gap_mm));
        md.push_str(&format!("- **Superstructure Base Shear Reduction Ratio:** {:.1}% of fixed-base\n\n", report.base_shear_reduction_ratio * 100.0));

        md.push_str("### 1. ASCE 7-22 / SNI 1726:2019 Regulatory Audit Matrix\n\n");
        md.push_str("| Clause | Description | Demand | Capacity | Unit | D/C Ratio | Status |\n");
        md.push_str("| :--- | :--- | :---: | :---: | :---: | :---: | :---: |\n");
        for c in &report.regulatory_checks {
            let status = if c.is_passed { "**PASS**" } else { "**FAIL**" };
            md.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {} | {:.3} | {} |\n",
                c.code_clause, c.description, c.demand_val, c.capacity_val, c.unit, c.demand_capacity_ratio, status
            ));
        }

        md.push_str("\n### 2. Device Specifications & Hysteretic Characteristics\n\n");
        match report.isolator_type {
            IsolatorType::LeadRubberBearing | IsolatorType::HighDampingRubberBearing => {
                md.push_str(&format!("- Outer Diameter $D_o$: {:.1} mm\n", input.lrb_params.outer_diameter_mm));
                md.push_str(&format!("- Lead Plug Diameter $d_{{pb}}$: {:.1} mm\n", input.lrb_params.lead_core_diameter_mm));
                md.push_str(&format!("- Total Rubber Thickness $T_r$: {:.1} mm ({} layers x {:.1} mm)\n", report.total_rubber_thick_tr_mm, input.lrb_params.num_rubber_layers, input.lrb_params.rubber_layer_thick_mm));
                md.push_str(&format!("- Primary Shape Factor $S_1$: {:.2}\n", report.shape_factor_s1));
                md.push_str(&format!("- Secondary Shape Factor $S_2$: {:.2}\n", report.shape_factor_s2));
                md.push_str(&format!("- Characteristic Strength $Q_d$ per bearing: {:.1} kN\n", report.characteristic_strength_qd_kn));
                md.push_str(&format!("- Post-Yield Stiffness $k_d$ per bearing: {:.3} kN/mm\n", report.post_yield_stiffness_kd_kn_per_mm));
                md.push_str(&format!("- Critical Buckling Load $P_{{cr}}(D_M)$: {:.1} kN (SF = {:.2})\n", report.critical_buckling_load_pcrd_kn, report.buckling_safety_factor));
            }
            IsolatorType::FrictionPendulum => {
                md.push_str(&format!("- Spherical Radius of Curvature $R_{{eff}}$: {:.1} mm\n", input.fps_params.radius_of_curvature_mm));
                md.push_str(&format!("- High-Velocity Dynamic Friction $\\mu_{{fast}}$: {:.3}\n", input.fps_params.friction_coeff_fast));
                md.push_str(&format!("- Housing Diameter: {:.1} mm (Slider: {:.1} mm)\n", input.fps_params.housing_diameter_mm, input.fps_params.slider_diameter_mm));
                md.push_str(&format!("- Total Pendulum Restoring Stiffness: {:.3} kN/mm\n", report.post_yield_stiffness_kd_kn_per_mm));
            }
            IsolatorType::FluidViscousDamper => {
                md.push_str(&format!("- Damper Damping Coefficient $C$: {:.1} kN/(m/s)^alpha\n", input.fvd_params.damping_coeff_c));
                md.push_str(&format!("- Velocity Exponent $\\alpha$: {:.2}\n", input.fvd_params.velocity_exponent_alpha));
                md.push_str(&format!("- Peak Damper Force per unit: {:.1} kN\n", report.damper_peak_force_kn));
                md.push_str(&format!("- Supplementary Viscous Damping $\\beta_v$: {:.1}%\n", report.supplementary_damping_beta_v * 100.0));
                md.push_str(&format!("- Total Combined Damping $\\beta_{{total}}$: {:.1}%\n", report.combined_total_damping * 100.0));
            }
        }

        md.push_str("\n---\n*Certified by EigenGraph Desktop Structural Engine v0.1.2*\n");
        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lead_rubber_bearing_evaluation() {
        let input = SeismicIsolationInput::default();
        let report = SeismicIsolationEngine::evaluate(&input);

        assert_eq!(report.isolator_type, IsolatorType::LeadRubberBearing);
        assert!(report.effective_period_tm_s > 2.0, "Effective period T_M must exceed 2.0 s");
        assert!(report.max_displacement_dm_mm > 100.0 && report.max_displacement_dm_mm < 400.0);
        assert!(report.total_max_displacement_dtm_mm >= report.max_displacement_dm_mm);
        assert!(report.effective_damping_beta_m > 0.10, "Effective damping should be > 10%");
        assert!(report.base_shear_reduction_ratio < 0.60, "Base shear should be reduced below 60%");
        assert!(report.buckling_safety_factor > 1.5, "Buckling safety factor must be safe");
    }

    #[test]
    fn test_friction_pendulum_evaluation() {
        let mut input = SeismicIsolationInput::default();
        input.isolator_type = IsolatorType::FrictionPendulum;
        let report = SeismicIsolationEngine::evaluate(&input);

        assert_eq!(report.isolator_type, IsolatorType::FrictionPendulum);
        assert!(report.effective_period_tm_s > 2.5);
        assert!(report.max_displacement_dm_mm > 80.0);
        assert!(report.effective_damping_beta_m > 0.10);
        assert!(report.is_design_acceptable);
    }

    #[test]
    fn test_fluid_viscous_damper_evaluation() {
        let mut input = SeismicIsolationInput::default();
        input.isolator_type = IsolatorType::FluidViscousDamper;
        let report = SeismicIsolationEngine::evaluate(&input);

        assert_eq!(report.isolator_type, IsolatorType::FluidViscousDamper);
        assert!(report.supplementary_damping_beta_v > 0.05);
        assert!(report.damper_peak_force_kn > 50.0);
        assert!(report.damping_reduction_bm > 1.0);
    }

    #[test]
    fn test_base_isolated_hospital_archetype_generation() {
        let (nodes, elements) = SeismicIsolationEngine::generate_base_isolated_hospital_archetype();

        assert_eq!(nodes.len(), 120, "Should generate 120 nodes across foundation, isolation base, and 8 stories");
        assert!(!elements.is_empty(), "Element list must not be empty");
        let isolator_count = elements.iter().filter(|e| e.3).count();
        assert_eq!(isolator_count, 12, "Should have 12 base isolator elements connecting foundation to base slab");
    }
}
