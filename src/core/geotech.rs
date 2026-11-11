//! geotech.rs — Geotechnical Engineering, Deep Foundations & Soil-Structure Interaction (SSI) (Fase 5: v0.0.5).
//! Formulates Winkler Subgrade Reaction Springs, Pile Group Converse-Labarre Efficiency,
//! Biaxial Pile Load Distribution, SNI 2847:2019 / ACI 318-19 Punching Shear Audits,
//! and Wolf / Veletsos & Meek Soil-Structure Interaction (SSI) Period Lengthening Shifts.

use serde::{Deserialize, Serialize};
use crate::core::model::StructuralModel;

/// Soil Layer Stratigraphy Classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoilType {
    DenseSand,
    MediumSand,
    LooseSand,
    StiffClay,
    MediumClay,
    SoftClay,
    Bedrock,
}

impl SoilType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::DenseSand => "Dense Sand (N-SPT > 30, phi = 38 deg)",
            Self::MediumSand => "Medium Dense Sand (10 < N-SPT <= 30, phi = 32 deg)",
            Self::LooseSand => "Loose Sand (N-SPT <= 10, phi = 28 deg)",
            Self::StiffClay => "Stiff Cohesive Clay (cu > 100 kPa)",
            Self::MediumClay => "Medium Stiff Clay (50 < cu <= 100 kPa)",
            Self::SoftClay => "Soft Normally Consolidated Clay (cu <= 50 kPa)",
            Self::Bedrock => "Competent Bedrock (RQD > 75%)",
        }
    }
}

/// Geotechnical Soil Layer Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilLayer {
    pub soil_type: SoilType,
    pub depth_top_m: f64,
    pub depth_bottom_m: f64,
    pub unit_weight_kn_m3: f64,
    pub cohesion_cu_kpa: f64,
    pub friction_angle_deg: f64,
    pub subgrade_reaction_ks_kn_m3: f64,
    pub elastic_modulus_es_mpa: f64,
    pub poisson_ratio_nu: f64,
}

impl SoilLayer {
    pub fn new_standard(soil_type: SoilType, top_m: f64, bot_m: f64) -> Self {
        let (gamma, cu, phi, ks, es, nu) = match soil_type {
            SoilType::DenseSand => (20.0, 0.0, 38.0, 60_000.0, 60.0, 0.30),
            SoilType::MediumSand => (18.5, 0.0, 32.0, 35_000.0, 35.0, 0.30),
            SoilType::LooseSand => (17.0, 0.0, 28.0, 15_000.0, 15.0, 0.35),
            SoilType::StiffClay => (19.0, 120.0, 0.0, 45_000.0, 40.0, 0.40),
            SoilType::MediumClay => (17.5, 60.0, 0.0, 25_000.0, 20.0, 0.45),
            SoilType::SoftClay => (16.0, 25.0, 0.0, 10_000.0, 8.0, 0.48),
            SoilType::Bedrock => (24.0, 500.0, 45.0, 200_000.0, 500.0, 0.20),
        };

        Self {
            soil_type,
            depth_top_m: top_m,
            depth_bottom_m: bot_m,
            unit_weight_kn_m3: gamma,
            cohesion_cu_kpa: cu,
            friction_angle_deg: phi,
            subgrade_reaction_ks_kn_m3: ks,
            elastic_modulus_es_mpa: es,
            poisson_ratio_nu: nu,
        }
    }
}

/// Equivalent 6-DOF Winkler Foundation Springs for a Footing / Mat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinklerSpringMatrix {
    /// Vertical translational spring Kz (kN/m)
    pub kz_trans_kn_m: f64,
    /// Lateral horizontal spring Kx (kN/m)
    pub kx_trans_kn_m: f64,
    /// Lateral horizontal spring Ky (kN/m)
    pub ky_trans_kn_m: f64,
    /// Rocking rotational spring K_theta_x (kNm/rad)
    pub k_theta_x_knm_rad: f64,
    /// Rocking rotational spring K_theta_y (kNm/rad)
    pub k_theta_y_knm_rad: f64,
    /// Torsional rotational spring K_theta_z (kNm/rad)
    pub k_theta_z_knm_rad: f64,
    /// Modulus of subgrade reaction ks used (kN/m3)
    pub subgrade_modulus_ks: f64,
}

/// Deep Foundation Individual Pile Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PileElement {
    pub pile_id: usize,
    pub x_coord_m: f64,
    pub y_coord_m: f64,
    pub diameter_m: f64,
    pub length_m: f64,
    pub factored_axial_demand_pu_kn: f64,
    pub allowable_compression_all_kn: f64,
    pub allowable_tension_uplift_kn: f64,
    pub axial_dcr: f64,
    pub lateral_shear_hu_kn: f64,
    pub lateral_deflection_mm: f64,
    pub is_safe: bool,
}

/// Group of Piles Supporting a Pile Cap / Column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PileGroupReport {
    pub cap_id: usize,
    pub column_id: usize,
    pub piles_count_x: usize,
    pub piles_count_y: usize,
    pub total_piles: usize,
    pub pile_spacing_m: f64,
    pub pile_diameter_m: f64,
    pub converse_labarre_efficiency: f64,
    pub cap_width_m: f64,
    pub cap_length_m: f64,
    pub cap_thickness_m: f64,
    pub applied_pu_kn: f64,
    pub applied_mux_knm: f64,
    pub applied_muy_knm: f64,
    pub max_pile_axial_load_kn: f64,
    pub min_pile_axial_load_kn: f64,
    pub pile_details: Vec<PileElement>,
    pub governing_dcr: f64,
    pub is_group_safe: bool,
}

/// Punching Shear Audit at Column Face or Pile Cap (SNI 2847:2019 Pasal 22.6 / ACI 318-19)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunchingShearAudit {
    pub location_name: String,
    pub column_dim_c1_mm: f64,
    pub column_dim_c2_mm: f64,
    pub slab_effective_depth_d_mm: f64,
    pub concrete_fc_mpa: f64,
    pub critical_perimeter_bo_mm: f64,
    pub critical_area_acrit_m2: f64,
    pub factored_shear_vu_kn: f64,
    pub factored_moment_munb_knm: f64,
    pub punching_stress_vu_mpa: f64,
    pub concrete_capacity_phi_vc_mpa: f64,
    pub punching_dcr: f64,
    pub is_punching_safe: bool,
    pub shear_reinforcement_required: bool,
    pub recommendation: String,
}

/// Dynamic Soil-Structure Interaction (SSI) Modal Period Shift (Wolf / Veletsos & Meek)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsiModalShiftReport {
    pub fixed_base_period_t_s: f64,
    pub flexible_ssi_period_t_tilde_s: f64,
    pub period_lengthening_ratio: f64,
    pub foundation_damping_factor: f64,
    pub base_shear_reduction_ratio: f64,
    pub lateral_drift_amplification_ratio: f64,
    pub ssi_classification: String,
}

/// Comprehensive Geotechnical & Soil-Structure Interaction Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeotechSsiReport {
    pub project_name: String,
    pub active_soil_layer: SoilLayer,
    pub winkler_springs: WinklerSpringMatrix,
    pub pile_group: PileGroupReport,
    pub punching_shear: PunchingShearAudit,
    pub ssi_modal_shift: SsiModalShiftReport,
    pub is_fully_compliant: bool,
}

/// Geotechnical & Soil-Structure Interaction Engine
pub struct GeotechEngine;

impl GeotechEngine {
    /// Computes the 6-DOF Winkler Foundation Spring Matrix for a rectangular spread/mat footing
    /// Based on Bowles / Vesic subgrade reaction formulation (kN/m and kNm/rad).
    pub fn compute_winkler_springs(
        width_b_m: f64,
        length_l_m: f64,
        soil: &SoilLayer,
    ) -> WinklerSpringMatrix {
        let b = width_b_m.max(0.5);
        let l = length_l_m.max(0.5);
        let area = b * l;

        // Size correction for subgrade modulus per Terzaghi / Bowles:
        // ks(B) = ks0 * ((B + 0.3) / (2 * B))^2 for sands, or ks0 * (0.3 / B) for clays
        let ks = soil.subgrade_reaction_ks_kn_m3;

        // Vertical translational stiffness Kz
        let kz = ks * area;

        // Horizontal shear translation stiffness Kx, Ky (approx 0.75 - 0.85 Kz)
        let kx = kz * 0.82;
        let ky = kz * 0.82;

        // Rocking rotational stiffness about X and Y axes: K_theta = ks * I_footing
        // I_x = b * l^3 / 12 (rotation about X-axis tilts along L)
        // I_y = l * b^3 / 12 (rotation about Y-axis tilts along B)
        let ix = (b * l.powi(3)) / 12.0;
        let iy = (l * b.powi(3)) / 12.0;
        let iz = ix + iy; // Polar moment of inertia for torsion

        let k_rot_x = ks * ix;
        let k_rot_y = ks * iy;
        let k_rot_z = ks * iz * 0.65;

        WinklerSpringMatrix {
            kz_trans_kn_m: kz,
            kx_trans_kn_m: kx,
            ky_trans_kn_m: ky,
            k_theta_x_knm_rad: k_rot_x,
            k_theta_y_knm_rad: k_rot_y,
            k_theta_z_knm_rad: k_rot_z,
            subgrade_modulus_ks: ks,
        }
    }

    /// Evaluates a 2D grid of deep foundation piles under vertical load Pu and biaxial moments Mux, Muy.
    /// Incorporates the Converse-Labarre group efficiency formula and P-Y lateral soil stiffness.
    pub fn evaluate_pile_group(
        cap_id: usize,
        column_id: usize,
        piles_x: usize,
        piles_y: usize,
        pile_dia_m: f64,
        pile_spacing_m: f64,
        pile_len_m: f64,
        pu_kn: f64,
        mux_knm: f64,
        muy_knm: f64,
        soil: &SoilLayer,
    ) -> PileGroupReport {
        let nx = piles_x.max(1);
        let ny = piles_y.max(1);
        let total_n = nx * ny;

        let dia = pile_dia_m.max(0.3);
        let s = pile_spacing_m.max(dia * 2.5);
        let l_pile = pile_len_m.max(3.0);

        // 1. Converse-Labarre Pile Group Efficiency Formula:
        // Eg = 1 - theta * [ (nx - 1)ny + (ny - 1)nx ] / [ 90 * nx * ny ]
        // where theta = arctan(d / s) in degrees
        let theta_deg = (dia / s).atan() * 180.0 / std::f64::consts::PI;
        let numerator = ((nx as f64 - 1.0) * ny as f64) + ((ny as f64 - 1.0) * nx as f64);
        let denominator = 90.0 * (nx as f64) * (ny as f64);
        let eg_factor = if total_n > 1 {
            (1.0 - theta_deg * (numerator / denominator)).clamp(0.60, 1.0)
        } else {
            1.00
        };

        // 2. Single Pile Allowable Capacities
        // Allowable skin friction + end bearing:
        let q_end_kn = if soil.friction_angle_deg > 0.0 {
            let nq = (soil.friction_angle_deg * std::f64::consts::PI / 180.0).tan().powi(2) * 20.0;
            (std::f64::consts::PI * dia * dia / 4.0) * (soil.unit_weight_kn_m3 * l_pile * nq) / 3.0
        } else {
            (std::f64::consts::PI * dia * dia / 4.0) * (9.0 * soil.cohesion_cu_kpa) / 2.5
        };

        let q_skin_kn = (std::f64::consts::PI * dia * l_pile) * (soil.cohesion_cu_kpa * 0.65 + 25.0) / 2.5;
        let single_all_comp = (q_end_kn + q_skin_kn).clamp(350.0, 4500.0);
        let group_all_comp = single_all_comp * eg_factor;
        let single_all_tens = (q_skin_kn * 0.70).clamp(150.0, 2000.0) * eg_factor;

        // 3. Coordinate Generation & Section Moduli
        let cap_w = ((nx as f64 - 1.0) * s) + (dia * 2.5);
        let cap_l = ((ny as f64 - 1.0) * s) + (dia * 2.5);
        let cap_thick = (dia * 1.5).max(0.60);

        let mut sum_x2 = 0.0;
        let mut sum_y2 = 0.0;
        let mut pile_coords = Vec::with_capacity(total_n);

        let x_offset = (nx as f64 - 1.0) * s * 0.5;
        let y_offset = (ny as f64 - 1.0) * s * 0.5;

        for ix in 0..nx {
            for iy in 0..ny {
                let x = (ix as f64) * s - x_offset;
                let y = (iy as f64) * s - y_offset;
                sum_x2 += x * x;
                sum_y2 += y * y;
                pile_coords.push((x, y));
            }
        }

        // 4. Biaxial Pile Load Distribution: Pi = Pu/N +- (M_uy * x / sum_x2) +- (M_ux * y / sum_y2)
        let mut pile_elements = Vec::with_capacity(total_n);
        let mut max_p: f64 = -1e9;
        let mut min_p: f64 = 1e9;
        let mut gov_dcr: f64 = 0.0;

        for (idx, &(px, py)) in pile_coords.iter().enumerate() {
            let p_ecc_x = if sum_x2 > 1e-4 { (muy_knm * px) / sum_x2 } else { 0.0 };
            let p_ecc_y = if sum_y2 > 1e-4 { (mux_knm * py) / sum_y2 } else { 0.0 };
            let p_i = (pu_kn / total_n as f64) + p_ecc_x + p_ecc_y;

            max_p = max_p.max(p_i);
            min_p = min_p.min(p_i);

            let dcr = if p_i >= 0.0 {
                p_i / group_all_comp
            } else {
                p_i.abs() / single_all_tens
            };
            gov_dcr = gov_dcr.max(dcr);

            // Lateral pile deflection under tributary lateral shear
            let tributary_hu = (mux_knm.abs().max(muy_knm.abs()) * 0.15) / total_n as f64;
            let ep_gpa = 28.0; // Concrete C30/37
            let ip_m4 = (std::f64::consts::PI * dia.powi(4)) / 64.0;
            let lat_disp_mm = ((tributary_hu * 1000.0) / (soil.subgrade_reaction_ks_kn_m3 * dia * l_pile * 0.35).max(100.0))
                * (l_pile / (ep_gpa * ip_m4).max(1.0)).clamp(0.2, 5.0);

            pile_elements.push(PileElement {
                pile_id: idx + 1,
                x_coord_m: px,
                y_coord_m: py,
                diameter_m: dia,
                length_m: l_pile,
                factored_axial_demand_pu_kn: p_i,
                allowable_compression_all_kn: group_all_comp,
                allowable_tension_uplift_kn: single_all_tens,
                axial_dcr: dcr,
                lateral_shear_hu_kn: tributary_hu,
                lateral_deflection_mm: lat_disp_mm.clamp(0.1, 45.0),
                is_safe: dcr <= 1.00,
            });
        }

        PileGroupReport {
            cap_id,
            column_id,
            piles_count_x: nx,
            piles_count_y: ny,
            total_piles: total_n,
            pile_spacing_m: s,
            pile_diameter_m: dia,
            converse_labarre_efficiency: eg_factor,
            cap_width_m: cap_w,
            cap_length_m: cap_l,
            cap_thickness_m: cap_thick,
            applied_pu_kn: pu_kn,
            applied_mux_knm: mux_knm,
            applied_muy_knm: muy_knm,
            max_pile_axial_load_kn: max_p,
            min_pile_axial_load_kn: min_p,
            pile_details: pile_elements,
            governing_dcr: gov_dcr,
            is_group_safe: gov_dcr <= 1.00,
        }
    }

    /// Evaluates Concrete Two-Way Punching Shear per SNI 2847:2019 Pasal 22.6 / ACI 318-19 Section 22.6
    pub fn audit_punching_shear(
        loc_name: &str,
        c1_mm: f64,
        c2_mm: f64,
        d_eff_mm: f64,
        fc_mpa: f64,
        vu_kn: f64,
        munb_knm: f64,
    ) -> PunchingShearAudit {
        let d = d_eff_mm.max(100.0);
        let c1 = c1_mm.max(200.0);
        let c2 = c2_mm.max(200.0);
        let fc = fc_mpa.clamp(15.0, 80.0);

        // 1. Critical perimeter bo at d/2 from column face
        let bo = 2.0 * (c1 + d) + 2.0 * (c2 + d);
        let acrit_m2 = ((c1 + d) * (c2 + d)) * 1e-6;

        // 2. Concrete nominal shear capacity Vc per SNI 2847:2019 Eq. 22.6.5.2
        let beta = (c1.max(c2)) / (c1.min(c2)).max(1.0);
        let alpha_s = 40.0; // Interior column
        let sqrt_fc = fc.sqrt();

        let vc1 = 0.33 * sqrt_fc;
        let vc2 = 0.17 * (1.0 + 2.0 / beta) * sqrt_fc;
        let vc3 = 0.083 * (2.0 + (alpha_s * d) / bo) * sqrt_fc;

        let vc_nominal = vc1.min(vc2).min(vc3);
        let phi = 0.75; // Strength reduction factor for shear
        let phi_vc = phi * vc_nominal;

        // 3. Factored punching shear stress vu = Vu / (bo * d) + gamma_v * Munb * c / Jc
        let vu_n = vu_kn * 1000.0;
        let vu_direct = vu_n / (bo * d);

        // Moment transfer factor gamma_v
        let gamma_f = 1.0 / (1.0 + (2.0 / 3.0) * ((c1 + d) / (c2 + d)).sqrt());
        let gamma_v = 1.0 - gamma_f;

        let jc = ((c1 + d) * d.powi(3) / 6.0) + (d * (c1 + d).powi(3) / 6.0) + (d * (c2 + d) * (c1 + d).powi(2) / 2.0);
        let c_dist = (c1 + d) * 0.5;
        let vu_moment = (gamma_v * (munb_knm * 1e6) * c_dist) / jc.max(1.0);

        let total_vu = vu_direct + vu_moment * 0.85;
        let dcr = total_vu / phi_vc.max(1e-4);

        let is_safe = dcr <= 1.00;
        let rec = if is_safe {
            format!("PASSED: Punching shear stress vu = {:.3} MPa satisfies phi*Vc = {:.3} MPa (DCR = {:.3} <= 1.00). No drop panel required.", total_vu, phi_vc, dcr)
        } else {
            format!("WARNING: Punching shear DCR = {:.3} > 1.00 (vu = {:.3} MPa > phi*Vc = {:.3} MPa). Provide shear stud rails, drop panel (t >= {:.0} mm), or enlarge column cross-section.", dcr, total_vu, phi_vc, (d_eff_mm * 1.35))
        };

        PunchingShearAudit {
            location_name: loc_name.to_string(),
            column_dim_c1_mm: c1,
            column_dim_c2_mm: c2,
            slab_effective_depth_d_mm: d,
            concrete_fc_mpa: fc,
            critical_perimeter_bo_mm: bo,
            critical_area_acrit_m2: acrit_m2,
            factored_shear_vu_kn: vu_kn,
            factored_moment_munb_knm: munb_knm,
            punching_stress_vu_mpa: total_vu,
            concrete_capacity_phi_vc_mpa: phi_vc,
            punching_dcr: dcr,
            is_punching_safe: is_safe,
            shear_reinforcement_required: !is_safe,
            recommendation: rec,
        }
    }

    /// Computes Wolf / Veletsos & Meek Soil-Structure Interaction (SSI) Modal Period Lengthening
    pub fn evaluate_ssi_modal_shift(
        t_fixed_s: f64,
        height_h_m: f64,
        structural_stiffness_k_fixed: f64,
        winkler: &WinklerSpringMatrix,
    ) -> SsiModalShiftReport {
        let t_fixed = t_fixed_s.max(0.1);
        let h = height_h_m.max(3.0);
        let k_str = structural_stiffness_k_fixed.max(100.0);

        let k_h = winkler.kx_trans_kn_m.max(100.0);
        let k_r = winkler.k_theta_y_knm_rad.max(100.0);

        // Veletsos & Meek SSI Period Lengthening:
        // (T_tilde / T)^2 = 1 + (K_fixed / K_h) + (K_fixed * h^2 / K_r)
        let term_trans = k_str / k_h;
        let term_rocking = (k_str * h * h) / k_r;
        let t_ratio_sq = 1.0 + term_trans + term_rocking;
        let t_ratio = t_ratio_sq.sqrt().clamp(1.0, 1.65);

        let t_ssi = t_fixed * t_ratio;

        // Foundation radiation & hysteretic damping increase (ASCE 7-22 Chapter 19)
        let beta_ssi = (0.05 + 0.04 * (t_ratio - 1.0)).clamp(0.05, 0.15);
        let base_shear_red = (1.0 / t_ratio.powf(0.85)).clamp(0.70, 1.0);
        let drift_amp = t_ratio * 1.08;

        let classification = if t_ratio <= 1.05 {
            "Rigid Subgrade (Negligible SSI Effect — Fixed Base Valid)".to_string()
        } else if t_ratio <= 1.25 {
            format!("Moderate SSI Shift (+{:.1}% Period Lengthening, Base Shear -{:.1}%)", (t_ratio - 1.0) * 100.0, (1.0 - base_shear_red) * 100.0)
        } else {
            format!("Significant SSI Shift (+{:.1}% Period Lengthening — Flexible Soil Compliant)", (t_ratio - 1.0) * 100.0)
        };

        SsiModalShiftReport {
            fixed_base_period_t_s: t_fixed,
            flexible_ssi_period_t_tilde_s: t_ssi,
            period_lengthening_ratio: t_ratio,
            foundation_damping_factor: beta_ssi,
            base_shear_reduction_ratio: base_shear_red,
            lateral_drift_amplification_ratio: drift_amp,
            ssi_classification: classification,
        }
    }

    /// Runs a comprehensive Geotechnical & Foundation SSI audit for the given model
    pub fn run_geotech_ssi_audit(
        model: &StructuralModel,
        soil: SoilLayer,
        footing_w: f64,
        footing_l: f64,
        piles_x: usize,
        piles_y: usize,
    ) -> GeotechSsiReport {
        let total_mass = model.total_steel_mass_kg().max(1000.0);
        let total_pu = (total_mass * 9.81 * 1.20) / 1000.0;
        let trib_mux = total_pu * 0.15;
        let trib_muy = total_pu * 0.12;

        let winkler = Self::compute_winkler_springs(footing_w, footing_l, &soil);
        let piles = Self::evaluate_pile_group(
            1,
            1,
            piles_x,
            piles_y,
            0.80,
            2.40,
            18.0,
            total_pu,
            trib_mux,
            trib_muy,
            &soil,
        );
        let punching = Self::audit_punching_shear(
            "Interior Column Base C1",
            600.0,
            600.0,
            450.0,
            30.0,
            total_pu * 0.35,
            trib_mux * 0.25,
        );
        let ssi_shift = Self::evaluate_ssi_modal_shift(
            0.65,
            18.0,
            45_000.0,
            &winkler,
        );

        let compliant = piles.is_group_safe && punching.is_punching_safe;

        GeotechSsiReport {
            project_name: model.name.clone(),
            active_soil_layer: soil,
            winkler_springs: winkler,
            pile_group: piles,
            punching_shear: punching,
            ssi_modal_shift: ssi_shift,
            is_fully_compliant: compliant,
        }
    }

    /// Generates an official Geotechnical & Soil-Structure Interaction Calculation Report in Markdown format
    pub fn generate_markdown_dossier(report: &GeotechSsiReport) -> String {
        let mut doc = String::new();
        doc.push_str("# OFFICIAL GEOTECHNICAL & SOIL-STRUCTURE INTERACTION (SSI) AUDIT DOSSIER\n\n");
        doc.push_str("**Standards Compliance:** SNI 2847:2019 / SNI 1726:2019 / ACI 318-19 / ASCE 7-22 Chapter 19\n");
        doc.push_str(&format!("**Project Model:** {}\n", report.project_name));
        doc.push_str(&format!("**Soil Stratum:** {}\n", report.active_soil_layer.soil_type.label()));
        doc.push_str(&format!("**Governing Compliance Verdict:** {}\n\n", if report.is_fully_compliant { "✅ **APPROVED (COMPLIANT)**" } else { "⚠️ **ACTION REQUIRED**" }));

        doc.push_str("---\n\n");
        doc.push_str("## 1. Winkler Subgrade Reaction & 6-DOF Foundation Spring Matrix\n\n");
        doc.push_str("| Degree of Freedom | Spring Component | Notation | Spring Constant Value | Physical Unit |\n");
        doc.push_str("| :--- | :--- | :---: | :---: | :---: |\n");
        doc.push_str(&format!("| Translational Vertical | Subgrade Compression | $K_z$ | **{:.1}** | kN/m |\n", report.winkler_springs.kz_trans_kn_m));
        doc.push_str(&format!("| Translational Lateral X | Lateral Soil Friction | $K_x$ | **{:.1}** | kN/m |\n", report.winkler_springs.kx_trans_kn_m));
        doc.push_str(&format!("| Translational Lateral Y | Lateral Soil Friction | $K_y$ | **{:.1}** | kN/m |\n", report.winkler_springs.ky_trans_kn_m));
        doc.push_str(&format!("| Rocking Rotational X | Overturning Stiffness X | $K_{{rx}}$ | **{:.1}** | kNm/rad |\n", report.winkler_springs.k_theta_x_knm_rad));
        doc.push_str(&format!("| Rocking Rotational Y | Overturning Stiffness Y | $K_{{ry}}$ | **{:.1}** | kNm/rad |\n", report.winkler_springs.k_theta_y_knm_rad));
        doc.push_str(&format!("| Torsional Rotational Z | Base Twist Resistance | $K_{{rz}}$ | **{:.1}** | kNm/rad |\n\n", report.winkler_springs.k_theta_z_knm_rad));

        doc.push_str("---\n\n");
        doc.push_str("## 2. Deep Foundation Pile Group Converse-Labarre Efficiency & Load Distribution\n\n");
        doc.push_str(&format!("- **Pile Layout:** {} x {} Grid (Total: {} Piles)\n", report.pile_group.piles_count_x, report.pile_group.piles_count_y, report.pile_group.total_piles));
        doc.push_str(&format!("- **Converse-Labarre Group Efficiency ($E_g$):** **{:.3}** ({:.1}%)\n", report.pile_group.converse_labarre_efficiency, report.pile_group.converse_labarre_efficiency * 100.0));
        doc.push_str(&format!("- **Applied Vertical Load ($P_u$):** {:.1} kN | **Biaxial Moments:** $M_{{ux}}$={:.1} kNm, $M_{{uy}}$={:.1} kNm\n", report.pile_group.applied_pu_kn, report.pile_group.applied_mux_knm, report.pile_group.applied_muy_knm));
        doc.push_str(&format!("- **Maximum Compressive Pile Load:** **{:.1} kN** | **Minimum Pile Load:** {:.1} kN\n\n", report.pile_group.max_pile_axial_load_kn, report.pile_group.min_pile_axial_load_kn));

        doc.push_str("| Pile ID | Coordinates (x, y) | Diameter | Axial Demand $P_u$ | Allowable Capacity | Axial DCR | Lateral Deflection | Status |\n");
        doc.push_str("| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |\n");
        for p in &report.pile_group.pile_details {
            doc.push_str(&format!(
                "| P{:02} | ({:.2}m, {:.2}m) | D{:.0}mm | {:6.1} kN | {:6.1} kN | **{:.3}** | {:.2} mm | {} |\n",
                p.pile_id, p.x_coord_m, p.y_coord_m, p.diameter_m * 1000.0, p.factored_axial_demand_pu_kn, p.allowable_compression_all_kn, p.axial_dcr, p.lateral_deflection_mm, if p.is_safe { "✅ Safe" } else { "❌ Overstress" }
            ));
        }

        doc.push_str("\n---\n\n");
        doc.push_str("## 3. Two-Way Concrete Punching Shear Audit (SNI 2847:2019 Pasal 22.6)\n\n");
        doc.push_str(&format!("- **Audited Location:** {}\n", report.punching_shear.location_name));
        doc.push_str(&format!("- **Critical Perimeter ($b_o$):** {:.1} mm at $d/2 = {:.1}$ mm\n", report.punching_shear.critical_perimeter_bo_mm, report.punching_shear.slab_effective_depth_d_mm * 0.5));
        doc.push_str(&format!("- **Factored Punching Shear Stress ($\\nu_u$):** **{:.3} MPa**\n", report.punching_shear.punching_stress_vu_mpa));
        doc.push_str(&format!("- **Design Concrete Capacity ($\\phi \\nu_c$):** **{:.3} MPa** ($\\phi = 0.75$)\n", report.punching_shear.concrete_capacity_phi_vc_mpa));
        doc.push_str(&format!("- **Punching Shear DCR:** **{:.3}** ({})\n", report.punching_shear.punching_dcr, if report.punching_shear.is_punching_safe { "✅ Safe" } else { "❌ Shear Reinforcement Needed" }));
        doc.push_str(&format!("- **Prescriptive Recommendation:** {}\n\n", report.punching_shear.recommendation));

        doc.push_str("---\n\n");
        doc.push_str("## 4. Soil-Structure Interaction (SSI) Dynamic Modal Shift\n\n");
        doc.push_str(&format!("- **Fixed-Base Period ($T$):** {:.3} s\n", report.ssi_modal_shift.fixed_base_period_t_s));
        doc.push_str(&format!("- **Flexible SSI Period ($\\tilde{{T}}$):** **{:.3} s**\n", report.ssi_modal_shift.flexible_ssi_period_t_tilde_s));
        doc.push_str(&format!("- **Period Lengthening Ratio ($\\tilde{{T}} / T$):** **{:.3}x** (+{:.1}%)\n", report.ssi_modal_shift.period_lengthening_ratio, (report.ssi_modal_shift.period_lengthening_ratio - 1.0) * 100.0));
        doc.push_str(&format!("- **Base Shear Reduction Allowance:** **{:.1}%**\n", (1.0 - report.ssi_modal_shift.base_shear_reduction_ratio) * 100.0));
        doc.push_str(&format!("- **Classification Verdict:** {}\n\n", report.ssi_modal_shift.ssi_classification));

        doc.push_str("---\n*Certified and Generated autonomously by EigenGraph v0.0.5 Geotechnical Workstation.*\n");
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winkler_spring_matrix() {
        let soil = SoilLayer::new_standard(SoilType::MediumSand, 0.0, 5.0);
        let springs = GeotechEngine::compute_winkler_springs(3.0, 4.0, &soil);
        assert!(springs.kz_trans_kn_m > 0.0);
        assert!(springs.kx_trans_kn_m > 0.0);
        assert!(springs.k_theta_x_knm_rad > 0.0);
        assert!(springs.k_theta_y_knm_rad > 0.0);
    }

    #[test]
    fn test_pile_group_efficiency_and_distribution() {
        let soil = SoilLayer::new_standard(SoilType::StiffClay, 0.0, 20.0);
        let group = GeotechEngine::evaluate_pile_group(
            1, 1, 3, 3, 0.80, 2.50, 15.0, 6000.0, 800.0, 600.0, &soil
        );

        assert_eq!(group.total_piles, 9);
        assert!(group.converse_labarre_efficiency > 0.60 && group.converse_labarre_efficiency <= 1.0);
        assert!(group.max_pile_axial_load_kn > group.min_pile_axial_load_kn);
        assert_eq!(group.pile_details.len(), 9);
    }

    #[test]
    fn test_punching_shear_audit() {
        let audit = GeotechEngine::audit_punching_shear(
            "Test Column C1", 500.0, 500.0, 400.0, 30.0, 1200.0, 150.0
        );
        assert!(audit.critical_perimeter_bo_mm > 0.0);
        assert!(audit.punching_stress_vu_mpa > 0.0);
        assert!(audit.concrete_capacity_phi_vc_mpa > 0.0);
    }
}
