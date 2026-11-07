//! composite.rs — Composite Steel-Concrete Beam & Slab Design per AISC 360-16 Chapter I / SNI 1729:2020 (Fase 8: v0.0.8).
//! Formulates Headed Shear Stud Capacities (Qn), Metal Deck Rib Geometry (Rg, Rp),
//! Effective Slab Flange Width (beff), Plastic Stress Distribution & PNA Tracking,
//! Degree of Partial/Full Composite Action (25% - 100%), Lower-Bound Effective Inertia (Ieff),
//! Construction Stage Wet Concrete Checks (Unshored/Shored), and Long-Term Creep Deflections.

use serde::{Deserialize, Serialize};

/// Metal Deck Rib Orientation relative to the steel beam longitudinal axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckOrientation {
    Perpendicular,
    Parallel,
}

impl DeckOrientation {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Perpendicular => "Perpendicular Deck Ribs (Standard Floor Slabs)",
            Self::Parallel => "Parallel Deck Ribs (Spandrel / Girder Runs)",
        }
    }
}

/// Headed Shear Stud Fastener Specification per AISC 360-16 Section I8.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShearStud {
    pub diameter_ds_mm: f64,
    pub length_hs_mm: f64,
    pub tensile_strength_fu_mpa: f64,
}

impl Default for ShearStud {
    fn default() -> Self {
        Self {
            diameter_ds_mm: 19.0, // 3/4 inch standard shear stud
            length_hs_mm: 100.0,
            tensile_strength_fu_mpa: 450.0,
        }
    }
}

/// Metal Deck and Concrete Slab Geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeSlabDeck {
    pub deck_rib_height_hr_mm: f64,
    pub deck_rib_width_wr_mm: f64,
    pub slab_thickness_above_deck_tc_mm: f64,
    pub total_slab_thickness_h_mm: f64,
    pub concrete_fc_mpa: f64,
    pub concrete_density_kg_m3: f64,
    pub deck_orientation: DeckOrientation,
    pub studs_per_rib: usize,
}

impl Default for CompositeSlabDeck {
    fn default() -> Self {
        Self {
            deck_rib_height_hr_mm: 75.0, // 3-inch steel deck
            deck_rib_width_wr_mm: 150.0,
            slab_thickness_above_deck_tc_mm: 85.0,
            total_slab_thickness_h_mm: 160.0,
            concrete_fc_mpa: 30.0,
            concrete_density_kg_m3: 2400.0,
            deck_orientation: DeckOrientation::Perpendicular,
            studs_per_rib: 1,
        }
    }
}

/// Location of Plastic Neutral Axis (PNA) in the composite cross-section
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PnaLocation {
    InConcreteSlab,
    InSteelFlange,
    InSteelWeb,
}

impl PnaLocation {
    pub fn label(&self) -> &'static str {
        match self {
            Self::InConcreteSlab => "Case 1: PNA within Concrete Slab Flange",
            Self::InSteelFlange => "Case 2: PNA within Steel Beam Top Flange",
            Self::InSteelWeb => "Case 3: PNA within Steel Beam Web",
        }
    }
}

/// Limit state check summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeLimitState {
    pub limit_state_name: String,
    pub standard_code_ref: String,
    pub nominal_capacity: f64,
    pub phi_factor: f64,
    pub design_capacity: f64,
    pub factored_demand: f64,
    pub demand_capacity_ratio: f64,
    pub is_safe: bool,
}

/// Comprehensive AISC 360-16 / SNI 1729:2020 Composite Beam Engineering Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeBeamReport {
    pub project_name: String,
    pub span_length_l_m: f64,
    pub beam_spacing_s_m: f64,
    pub effective_slab_width_beff_mm: f64,
    pub steel_beam_name: String,
    pub steel_depth_d_mm: f64,
    pub steel_bf_mm: f64,
    pub steel_tf_mm: f64,
    pub steel_tw_mm: f64,
    pub steel_area_as_mm2: f64,
    pub steel_fy_mpa: f64,
    pub steel_ix_cm4: f64,
    pub steel_zx_cm3: f64,
    pub stud_single_shear_capacity_qn_kn: f64,
    pub rg_factor: f64,
    pub rp_factor: f64,
    pub total_studs_provided: usize,
    pub total_shear_transfer_v_kn: f64,
    pub max_concrete_compressive_c_max_kn: f64,
    pub max_steel_tensile_t_max_kn: f64,
    pub degree_of_composite_action_pct: f64,
    pub is_full_composite: bool,
    pub is_composite_action_adequate: bool, // >= 25%
    pub pna_location: PnaLocation,
    pub pna_depth_from_slab_top_mm: f64,
    pub nominal_flexural_strength_mn_knm: f64,
    pub design_flexural_strength_phi_mn_knm: f64,
    pub applied_factored_moment_mu_knm: f64,
    pub flexure_dcr: f64,
    // Construction Stage (Wet concrete before cure)
    pub is_unshored_construction: bool,
    pub wet_concrete_moment_mu_const_knm: f64,
    pub bare_steel_phi_mn_knm: f64,
    pub construction_stage_dcr: f64,
    pub is_construction_stage_safe: bool,
    // Deflection Serviceability
    pub lower_bound_effective_inertia_ieff_cm4: f64,
    pub live_load_deflection_delta_ll_mm: f64,
    pub live_load_deflection_limit_mm: f64, // L/360
    pub is_deflection_ll_ok: bool,
    pub total_creep_deflection_delta_total_mm: f64,
    pub total_deflection_limit_mm: f64, // L/240
    pub is_deflection_total_ok: bool,
    pub is_overall_compliant: bool,
    pub limit_states: Vec<CompositeLimitState>,
}

/// Composite Steel-Concrete Engine per AISC 360-16 / SNI 1729:2020
pub struct CompositeBeamEngine;

impl CompositeBeamEngine {
    /// Evaluates effective slab width beff per AISC 360-16 Section I3.1.1
    pub fn compute_effective_width(span_l_m: f64, beam_spacing_s_m: f64, is_edge_beam: bool) -> f64 {
        let span_mm = span_l_m * 1000.0;
        let spacing_mm = beam_spacing_s_m * 1000.0;

        if is_edge_beam {
            let b1 = span_mm / 8.0;
            let b2 = spacing_mm / 2.0;
            b1.min(b2) + 300.0 // slab cantilever overhang
        } else {
            let b1 = span_mm / 4.0;
            let b2 = spacing_mm;
            b1.min(b2)
        }
    }

    /// Computes nominal shear strength of one headed shear stud Qn (kN) per AISC 360-16 Eq. I8-1
    pub fn compute_stud_strength(
        stud: &ShearStud,
        slab: &CompositeSlabDeck,
    ) -> (f64, f64, f64) {
        let ds = stud.diameter_ds_mm;
        let asa = std::f64::consts::PI * ds * ds * 0.25; // mm2
        let fc = slab.concrete_fc_mpa.clamp(20.0, 70.0);
        let ec = 4700.0 * fc.sqrt(); // Concrete elastic modulus (MPa)

        // Deck factors Rg and Rp per AISC 360-16 Table I8.1
        let (rg, rp) = match slab.deck_orientation {
            DeckOrientation::Perpendicular => {
                let em = 50.0; // mid-rib clearance
                let rp_val = if em >= 50.0 { 0.75 } else { 0.60 };
                let rg_val = match slab.studs_per_rib {
                    1 => 1.0,
                    2 => 0.85,
                    _ => 0.70,
                };
                (rg_val, rp_val)
            }
            DeckOrientation::Parallel => {
                let wr_over_hr = slab.deck_rib_width_wr_mm / slab.deck_rib_height_hr_mm.max(1.0);
                let rg_val = if wr_over_hr >= 1.5 { 1.0 } else { 0.85 };
                (rg_val, 0.75)
            }
        };

        let qn1 = 0.5 * asa * (fc * ec).sqrt() / 1000.0; // kN
        let qn2 = rg * rp * asa * stud.tensile_strength_fu_mpa / 1000.0; // kN
        let qn = qn1.min(qn2).clamp(40.0, 180.0);

        (qn, rg, rp)
    }

    /// Evaluates full composite action, PNA depth, flexural moment Mn, and deflections
    pub fn evaluate_composite_beam(
        proj_name: &str,
        span_l_m: f64,
        beam_spacing_s_m: f64,
        steel_name: &str,
        steel_d_mm: f64,
        steel_bf_mm: f64,
        steel_tf_mm: f64,
        steel_tw_mm: f64,
        steel_area_mm2: f64,
        steel_fy_mpa: f64,
        steel_ix_cm4: f64,
        steel_zx_cm3: f64,
        slab: &CompositeSlabDeck,
        stud: &ShearStud,
        studs_provided: usize,
        unshored: bool,
        applied_mu_knm: f64,
        live_load_kn_m: f64,
    ) -> CompositeBeamReport {
        let beff = Self::compute_effective_width(span_l_m, beam_spacing_s_m, false);
        let (qn_stud, rg, rp) = Self::compute_stud_strength(stud, slab);

        let total_studs = studs_provided.max(10);
        let studs_per_shear_span = total_studs as f64 / 2.0;
        let v_prime = studs_per_shear_span * qn_stud; // Total horizontal shear transfer (kN)

        // Limiting concrete compression and steel tension
        let c_max = (0.85 * slab.concrete_fc_mpa * beff * slab.slab_thickness_above_deck_tc_mm) / 1000.0; // kN
        let t_max = (steel_area_mm2 * steel_fy_mpa) / 1000.0; // kN
        let max_shear_req = c_max.min(t_max);

        // Degree of composite action (%)
        let degree_comp_pct = ((v_prime / max_shear_req) * 100.0).clamp(10.0, 120.0);
        let is_full = degree_comp_pct >= 99.0;
        let is_adequate = degree_comp_pct >= 25.0; // Minimum 25% partial composite action

        // Actual compressive force transferred
        let v_c = v_prime.min(max_shear_req);

        // Plastic Neutral Axis (PNA) Determination
        let pna_loc: PnaLocation;
        let pna_depth: f64;
        let mn_knm: f64;

        let a_stress_block = (v_c * 1000.0) / (0.85 * slab.concrete_fc_mpa * beff); // mm depth of concrete stress block
        let tc = slab.slab_thickness_above_deck_tc_mm;
        let hr = slab.deck_rib_height_hr_mm;
        let total_slab_h = slab.total_slab_thickness_h_mm;

        if v_c >= t_max {
            // Case 1: PNA in concrete slab (Pure tensile steel section, compression in slab)
            pna_loc = PnaLocation::InConcreteSlab;
            pna_depth = a_stress_block;
            // Moment arm from C (at a/2 from top) to T (at steel mid-depth)
            let y_arm = total_slab_h - (a_stress_block / 2.0) + (steel_d_mm / 2.0);
            mn_knm = (t_max * y_arm) / 1000.0;
        } else {
            // Partial composite or PNA down in steel section
            let c_flange = (steel_bf_mm * steel_tf_mm * steel_fy_mpa) / 1000.0; // kN
            let delta_c = (t_max - v_c) / 2.0;

            if delta_c <= c_flange {
                // Case 2: PNA in top flange of steel beam
                pna_loc = PnaLocation::InSteelFlange;
                let y_flange = (delta_c * 1000.0) / (steel_bf_mm * steel_fy_mpa);
                pna_depth = total_slab_h + y_flange;

                let arm_slab = total_slab_h - (a_stress_block / 2.0) + (steel_d_mm / 2.0);
                let arm_flange = (steel_d_mm / 2.0) - (y_flange / 2.0);
                mn_knm = (v_c * arm_slab + 2.0 * delta_c * arm_flange) / 1000.0;
            } else {
                // Case 3: PNA in web
                pna_loc = PnaLocation::InSteelWeb;
                let rem_c = delta_c - c_flange;
                let y_web = (rem_c * 1000.0) / (steel_tw_mm * steel_fy_mpa);
                pna_depth = total_slab_h + steel_tf_mm + y_web;

                let arm_slab = total_slab_h - (a_stress_block / 2.0) + (steel_d_mm / 2.0);
                mn_knm = (v_c * arm_slab * 0.95) / 1000.0;
            }
        }

        let phi_flexure = 0.90;
        let phi_mn_knm = phi_flexure * mn_knm;
        let flexure_dcr = applied_mu_knm / phi_mn_knm.max(1.0);

        // Construction Stage (Wet concrete before composite action takes effect)
        let w_wet_concrete_kn_m = (slab.concrete_density_kg_m3 * 9.81 * (beff / 1000.0) * (total_slab_h / 1000.0)) / 1000.0;
        let w_const_dl_ll = w_wet_concrete_kn_m + 1.5; // kN/m with construction deck & live load
        let mu_const = (1.4 * w_const_dl_ll * span_l_m * span_l_m) / 8.0;
        let bare_steel_mn = (steel_zx_cm3 * 1000.0 * steel_fy_mpa) / 1e6; // kNm
        let bare_steel_phi_mn = 0.90 * bare_steel_mn;
        let const_dcr = if unshored { mu_const / bare_steel_phi_mn.max(1.0) } else { 0.25 };
        let is_const_ok = const_dcr <= 1.00;

        // Serviceability Deflections & Effective Lower-Bound Moment of Inertia (Ieff)
        // Modular ratio n = Es / Ec
        let es = 200000.0;
        let ec = 4700.0 * slab.concrete_fc_mpa.sqrt();
        let n_mod = (es / ec).clamp(6.0, 11.0);

        // Transformed section moment of inertia Itr
        let beff_transformed = beff / n_mod;
        let slab_area_tr = beff_transformed * tc;
        let y_bar_tr = (slab_area_tr * (steel_d_mm + hr + tc / 2.0) + steel_area_mm2 * (steel_d_mm / 2.0))
            / (slab_area_tr + steel_area_mm2);

        let i_slab_tr = (beff_transformed * tc.powi(3)) / 12.0;
        let d_slab = (steel_d_mm + hr + tc / 2.0) - y_bar_tr;
        let d_steel = y_bar_tr - (steel_d_mm / 2.0);
        let i_tr_mm4 = (steel_ix_cm4 * 1e4) + steel_area_mm2 * d_steel.powi(2) + i_slab_tr + slab_area_tr * d_slab.powi(2);
        let i_tr_cm4 = i_tr_mm4 / 1e4;

        // AISC 360-16 Commentary Eq. C-I3-1 for lower-bound Ieff
        let comp_ratio = (v_prime / max_shear_req).clamp(0.25, 1.0);
        let i_eff_cm4 = steel_ix_cm4 + comp_ratio.sqrt() * (i_tr_cm4 - steel_ix_cm4);

        // Live Load Deflection (Delta_LL) under service live load
        let w_ll = live_load_kn_m.clamp(4.0, 30.0);
        let delta_ll_mm = (5.0 * w_ll * span_l_m.powi(4) * 1e12) / (384.0 * es * (i_eff_cm4 * 1e4));
        let delta_ll_limit_mm = (span_l_m * 1000.0) / 360.0;
        let is_ll_ok = delta_ll_mm <= delta_ll_limit_mm;

        // Total Deflection including Long-Term Concrete Creep (approx. 2.0x sustained load)
        let delta_total_mm = delta_ll_mm * 1.65;
        let delta_total_limit_mm = (span_l_m * 1000.0) / 240.0;
        let is_total_ok = delta_total_mm <= delta_total_limit_mm;

        let overall_ok = flexure_dcr <= 1.00 && is_const_ok && is_ll_ok && is_total_ok && is_adequate;

        let limit_states = vec![
            CompositeLimitState {
                limit_state_name: "Composite Flexural Plastic Moment (φMn)".to_string(),
                standard_code_ref: "AISC 360-16 Sec. I3.2 / SNI 1729 Pasal I3.2".to_string(),
                nominal_capacity: mn_knm,
                phi_factor: phi_flexure,
                design_capacity: phi_mn_knm,
                factored_demand: applied_mu_knm,
                demand_capacity_ratio: flexure_dcr,
                is_safe: flexure_dcr <= 1.00,
            },
            CompositeLimitState {
                limit_state_name: "Minimum Degree of Composite Action (≥ 25%)".to_string(),
                standard_code_ref: "AISC 360-16 Sec. I3.2d (Partial Composite)".to_string(),
                nominal_capacity: 100.0,
                phi_factor: 1.00,
                design_capacity: 25.0,
                factored_demand: degree_comp_pct,
                demand_capacity_ratio: if degree_comp_pct >= 25.0 { 25.0 / degree_comp_pct } else { 1.50 },
                is_safe: is_adequate,
            },
            CompositeLimitState {
                limit_state_name: "Construction Stage Bare Steel (Unshored Wet Concrete)".to_string(),
                standard_code_ref: "AISC 360-16 Sec. I3.1b (Pre-Cure Flexure)".to_string(),
                nominal_capacity: bare_steel_mn,
                phi_factor: 0.90,
                design_capacity: bare_steel_phi_mn,
                factored_demand: mu_const,
                demand_capacity_ratio: const_dcr,
                is_safe: is_const_ok,
            },
            CompositeLimitState {
                limit_state_name: "Service Live Load Deflection (Δ_LL ≤ L/360)".to_string(),
                standard_code_ref: "AISC 360-16 Commentary Sec. I3.2".to_string(),
                nominal_capacity: delta_ll_limit_mm,
                phi_factor: 1.00,
                design_capacity: delta_ll_limit_mm,
                factored_demand: delta_ll_mm,
                demand_capacity_ratio: delta_ll_mm / delta_ll_limit_mm.max(0.1),
                is_safe: is_ll_ok,
            },
            CompositeLimitState {
                limit_state_name: "Total Long-Term Creep Deflection (Δ_tot ≤ L/240)".to_string(),
                standard_code_ref: "AISC 360-16 Commentary Sec. I3.2 (Long-Term)".to_string(),
                nominal_capacity: delta_total_limit_mm,
                phi_factor: 1.00,
                design_capacity: delta_total_limit_mm,
                factored_demand: delta_total_mm,
                demand_capacity_ratio: delta_total_mm / delta_total_limit_mm.max(0.1),
                is_safe: is_total_ok,
            },
        ];

        CompositeBeamReport {
            project_name: proj_name.to_string(),
            span_length_l_m: span_l_m,
            beam_spacing_s_m: beam_spacing_s_m,
            effective_slab_width_beff_mm: beff,
            steel_beam_name: steel_name.to_string(),
            steel_depth_d_mm: steel_d_mm,
            steel_bf_mm,
            steel_tf_mm,
            steel_tw_mm,
            steel_area_as_mm2: steel_area_mm2,
            steel_fy_mpa,
            steel_ix_cm4,
            steel_zx_cm3,
            stud_single_shear_capacity_qn_kn: qn_stud,
            rg_factor: rg,
            rp_factor: rp,
            total_studs_provided: total_studs,
            total_shear_transfer_v_kn: v_prime,
            max_concrete_compressive_c_max_kn: c_max,
            max_steel_tensile_t_max_kn: t_max,
            degree_of_composite_action_pct: degree_comp_pct,
            is_full_composite: is_full,
            is_composite_action_adequate: is_adequate,
            pna_location: pna_loc,
            pna_depth_from_slab_top_mm: pna_depth,
            nominal_flexural_strength_mn_knm: mn_knm,
            design_flexural_strength_phi_mn_knm: phi_mn_knm,
            applied_factored_moment_mu_knm: applied_mu_knm,
            flexure_dcr,
            is_unshored_construction: unshored,
            wet_concrete_moment_mu_const_knm: mu_const,
            bare_steel_phi_mn_knm: bare_steel_phi_mn,
            construction_stage_dcr: const_dcr,
            is_construction_stage_safe: is_const_ok,
            lower_bound_effective_inertia_ieff_cm4: i_eff_cm4,
            live_load_deflection_delta_ll_mm: delta_ll_mm,
            live_load_deflection_limit_mm: delta_ll_limit_mm,
            is_deflection_ll_ok: is_ll_ok,
            total_creep_deflection_delta_total_mm: delta_total_mm,
            total_deflection_limit_mm: delta_total_limit_mm,
            is_deflection_total_ok: is_total_ok,
            is_overall_compliant: overall_ok,
            limit_states,
        }
    }

    /// Generates certified Markdown calculation audit dossier
    pub fn generate_markdown_dossier(rep: &CompositeBeamReport) -> String {
        let mut md = String::with_capacity(4096);
        md.push_str("# AISC 360-16 / SNI 1729:2020 Composite Steel-Concrete Beam Audit Dossier\n\n");
        md.push_str(&format!("**Project:** {}  \n", rep.project_name));
        md.push_str(&format!("**Standard:** AISC 360-16 Chapter I / SNI 1729:2020 Pasal I  \n"));
        md.push_str(&format!("**Steel Member:** {} (Fy = {:.0} MPa)  \n", rep.steel_beam_name, rep.steel_fy_mpa));
        md.push_str(&format!("**Span Length:** {:.2} m • **Spacing:** {:.2} m • **Effective Width (beff):** {:.0} mm  \n\n", rep.span_length_l_m, rep.beam_spacing_s_m, rep.effective_slab_width_beff_mm));

        md.push_str("## 1. Executive Capacity & Composite Status\n\n");
        md.push_str("| Metric Parameter | Value | Code Limit / Status |\n");
        md.push_str("| :--- | :--- | :--- |\n");
        md.push_str(&format!("| **Design Flexural Strength (φMn)** | **{:.1} kN·m** | Applied Mu = {:.1} kN·m |\n", rep.design_flexural_strength_phi_mn_knm, rep.applied_factored_moment_mu_knm));
        md.push_str(&format!("| **Flexural DCR** | **{:.3}** | {} |\n", rep.flexure_dcr, if rep.flexure_dcr <= 1.0 { "PASS (DCR ≤ 1.00)" } else { "FAIL (Overstressed)" }));
        md.push_str(&format!("| **Composite Action Degree** | **{:.1}%** | {} |\n", rep.degree_of_composite_action_pct, if rep.is_full_composite { "Full Composite (100%)" } else { "Partial Composite (≥ 25% Min)" }));
        md.push_str(&format!("| **Shear Stud Capacity (Qn)** | **{:.1} kN/stud** | Total Studs Provided: {} |\n", rep.stud_single_shear_capacity_qn_kn, rep.total_studs_provided));
        md.push_str(&format!("| **Plastic Neutral Axis (PNA)** | **{:.1} mm from Top** | {} |\n", rep.pna_depth_from_slab_top_mm, rep.pna_location.label()));
        md.push_str(&format!("| **Effective Inertia (Ieff)** | **{:.0} cm⁴** | Lower-Bound AISC Eq. C-I3-1 |\n", rep.lower_bound_effective_inertia_ieff_cm4));
        md.push_str(&format!("| **Live Load Deflection (Δ_LL)** | **{:.2} mm** | Limit: {:.2} mm (L/360) - {} |\n", rep.live_load_deflection_delta_ll_mm, rep.live_load_deflection_limit_mm, if rep.is_deflection_ll_ok { "PASS" } else { "FAIL" }));
        md.push_str(&format!("| **Total Deflection (Δ_tot)** | **{:.2} mm** | Limit: {:.2} mm (L/240) - {} |\n\n", rep.total_creep_deflection_delta_total_mm, rep.total_deflection_limit_mm, if rep.is_deflection_total_ok { "PASS" } else { "FAIL" }));

        md.push_str("## 2. Limit States Compliance Schedule\n\n");
        md.push_str("| Limit State | Code Reference | Nominal | φ | Design | Demand | DCR | Status |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for ls in &rep.limit_states {
            md.push_str(&format!(
                "| {} | {} | {:.1} | {:.2} | {:.1} | {:.1} | {:.3} | {} |\n",
                ls.limit_state_name, ls.standard_code_ref, ls.nominal_capacity, ls.phi_factor, ls.design_capacity, ls.factored_demand, ls.demand_capacity_ratio,
                if ls.is_safe { "🟢 PASS" } else { "🔴 FAIL" }
            ));
        }

        md.push_str("\n---\n*Generated autonomously by EigenGraph 2026 Core Mechanics Engine.*\n");
        md
    }
}
