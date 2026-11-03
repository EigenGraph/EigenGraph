//! concrete.rs — Reinforced Concrete Column P-M Interaction Diagram Engine (SNI 2847:2019 / ACI 318-19).
//! Implements Whitney stress block, strain compatibility, strain-dependent strength reduction factor phi,
//! balanced failure criteria, pure bending, pure axial limits, and radial Demand-to-Capacity Ratio (DCR).

use serde::{Deserialize, Serialize};

/// Concrete column cross-section definition with longitudinal reinforcement layers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConcreteSection {
    pub name: String,
    /// Cross-section width b in mm (parallel to bending axis)
    pub width_mm: f64,
    /// Cross-section depth h in mm (perpendicular to bending axis)
    pub height_mm: f64,
    /// Concrete specified compressive strength f'c in MPa (e.g. 30.0)
    pub fc_mpa: f64,
    /// Reinforcing steel yield strength fy in MPa (e.g. 420.0 for BJTS 420B)
    pub fy_mpa: f64,
    /// Steel modulus of elasticity Es in MPa (default 200,000.0)
    pub es_mpa: f64,
    /// Concrete clear cover in mm (default 40.0)
    pub cover_mm: f64,
    /// Longitudinal rebar layers (depth d_i from top compression fiber in mm)
    pub rebar_layers: Vec<RebarLayer>,
    /// Column confinement type (tied = false, spiral = true)
    pub is_spiral: bool,
}

/// A horizontal layer of reinforcing bars within the column cross section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebarLayer {
    /// Distance from extreme compression fiber to centroid of this bar layer (mm)
    pub depth_d_mm: f64,
    /// Number of bars in this layer
    pub bar_count: usize,
    /// Diameter of bars in mm (e.g. 22.0)
    pub bar_dia_mm: f64,
    /// Total steel area in this layer (mm²)
    pub total_area_mm2: f64,
}

impl RebarLayer {
    pub fn new(depth_d_mm: f64, bar_count: usize, bar_dia_mm: f64) -> Self {
        let single_area = std::f64::consts::PI * (bar_dia_mm * 0.5).powi(2);
        Self {
            depth_d_mm,
            bar_count,
            bar_dia_mm,
            total_area_mm2: bar_count as f64 * single_area,
        }
    }
}

/// A discrete point on the P-M interaction diagram
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InteractionPoint {
    /// Neutral axis depth c in mm
    pub c_mm: f64,
    /// Nominal axial capacity P_n in kN (positive = compression, negative = tension)
    pub pn_kn: f64,
    /// Nominal moment capacity M_n in kNm
    pub mn_knm: f64,
    /// Strength reduction factor phi per SNI 2847:2019 Pasal 21.2
    pub phi: f64,
    /// Factored design axial capacity phi*P_n in kN (with upper compression cutoff applied)
    pub phi_pn_kn: f64,
    /// Factored design moment capacity phi*M_n in kNm
    pub phi_mn_knm: f64,
    /// Net tensile strain in extreme tension reinforcement layer (epsilon_t)
    pub eps_t: f64,
}

/// Cardinal benchmark points along the P-M curve
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardinalPoints {
    /// Pure axial compression (P0 in kN, Pn_max in kN, phi_Pn_max in kN)
    pub p0_kn: f64,
    pub pn_max_kn: f64,
    pub phi_pn_max_kn: f64,
    /// Balanced failure point (simultaneous concrete crushing eps_cu=0.003 and steel yielding eps_s=eps_y)
    pub balanced: InteractionPoint,
    /// Pure flexure point (Pn = 0.0, zero axial load)
    pub pure_flexure: InteractionPoint,
    /// Pure axial tension point (Pnt in kN, phi_Pnt in kN, Mn = 0.0)
    pub pnt_kn: f64,
    pub phi_pnt_kn: f64,
}

/// Complete report of the SNI 2847:2019 / ACI 318-19 P-M interaction evaluation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConcretePmReport {
    pub section: ConcreteSection,
    pub ag_mm2: f64,
    pub ast_mm2: f64,
    pub rebar_ratio_pct: f64,
    pub plastic_centroid_y_mm: f64,
    pub beta1: f64,
    pub cardinal: CardinalPoints,
    /// Curve points sorted from pure compression to pure tension
    pub nominal_curve: Vec<(f64, f64)>,  // (M_n, P_n) in kNm, kN
    pub design_curve: Vec<(f64, f64)>,   // (phi*M_n, phi*P_n) in kNm, kN
    /// Current demand check if provided (Pu in kN, Mu in kNm)
    pub demand_check: Option<DemandAudit>,
}

/// Audit result for a specific factored design demand (Pu, Mu)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DemandAudit {
    pub pu_kn: f64,
    pub mu_knm: f64,
    /// Demand-to-Capacity Ratio (DCR)
    pub dcr: f64,
    pub is_safe: bool,
    pub governing_limit: String,
    pub phi_pn_cap_kn: f64,
    pub phi_mn_cap_knm: f64,
}

pub struct ConcreteEngine;

impl ConcreteEngine {
    /// Stress block depth factor beta_1 per SNI 2847:2019 Pasal 22.2.2.4.3
    pub fn compute_beta1(fc_mpa: f64) -> f64 {
        if fc_mpa <= 28.0 {
            0.85
        } else if fc_mpa < 55.0 {
            (0.85 - 0.05 * (fc_mpa - 28.0) / 7.0).max(0.65)
        } else {
            0.65
        }
    }

    /// Creates a standard symmetric rectangular tied column section
    pub fn create_rectangular_column(
        name: &str,
        width_mm: f64,
        height_mm: f64,
        fc_mpa: f64,
        fy_mpa: f64,
        cover_mm: f64,
        bars_per_face_x: usize, // bars on top and bottom faces (min 2)
        bars_intermediate_y: usize, // intermediate bars along side faces
        bar_dia_mm: f64,
    ) -> ConcreteSection {
        let mut layers = Vec::new();
        let d_top = cover_mm + bar_dia_mm * 0.5 + 10.0; // 10mm tie allowance
        let d_bot = height_mm - d_top;

        // Top layer (compression face in positive moment)
        layers.push(RebarLayer::new(d_top, bars_per_face_x.max(2), bar_dia_mm));

        // Intermediate side layers
        if bars_intermediate_y > 0 {
            let dy = (d_bot - d_top) / (bars_intermediate_y + 1) as f64;
            for i in 1..=bars_intermediate_y {
                let d_curr = d_top + dy * i as f64;
                layers.push(RebarLayer::new(d_curr, 2, bar_dia_mm));
            }
        }

        // Bottom layer (tension face in positive moment)
        layers.push(RebarLayer::new(d_bot, bars_per_face_x.max(2), bar_dia_mm));

        ConcreteSection {
            name: name.to_string(),
            width_mm,
            height_mm,
            fc_mpa,
            fy_mpa,
            es_mpa: 200_000.0,
            cover_mm,
            rebar_layers: layers,
            is_spiral: false,
        }
    }

    /// Evaluates internal equilibrium (Pn, Mn, phi) for an exact neutral axis depth c
    pub fn evaluate_section_at_c(sec: &ConcreteSection, c_mm: f64) -> InteractionPoint {
        let eps_cu = 0.003;
        let beta1 = Self::compute_beta1(sec.fc_mpa);
        let b = sec.width_mm;
        let h = sec.height_mm;
        let y_p = h * 0.5; // plastic centroid for symmetric sections
        let eps_y = sec.fy_mpa / sec.es_mpa;

        // Extreme tension steel layer depth
        let d_t = sec
            .rebar_layers
            .iter()
            .map(|l| l.depth_d_mm)
            .fold(0.0_f64, f64::max);

        // Concrete compression block
        let a = (beta1 * c_mm).min(h).max(0.0);
        let c_c_n = 0.85 * sec.fc_mpa * b * a;
        let m_c_nmm = c_c_n * (y_p - a * 0.5);

        // Steel layer contributions
        let mut f_s_total_n = 0.0;
        let mut m_s_total_nmm = 0.0;

        for layer in &sec.rebar_layers {
            let eps_s = if c_mm.abs() > 1e-6 {
                eps_cu * (c_mm - layer.depth_d_mm) / c_mm
            } else {
                -eps_cu * 100.0 // pure tension
            };

            let f_s_unreduced = (eps_s * sec.es_mpa).clamp(-sec.fy_mpa, sec.fy_mpa);

            // Subtract concrete displacement if rebar is inside the compression block
            let f_s_effective = if layer.depth_d_mm <= a && eps_s > 0.0 {
                f_s_unreduced - 0.85 * sec.fc_mpa
            } else {
                f_s_unreduced
            };

            let f_layer_n = layer.total_area_mm2 * f_s_effective;
            let m_layer_nmm = f_layer_n * (y_p - layer.depth_d_mm);

            f_s_total_n += f_layer_n;
            m_s_total_nmm += m_layer_nmm;
        }

        let p_n_n = c_c_n + f_s_total_n;
        let m_n_nmm = m_c_nmm + m_s_total_nmm;

        let pn_kn = p_n_n * 1e-3;
        let mn_knm = m_n_nmm * 1e-6;

        // Net tensile strain at extreme steel layer
        let eps_t = if c_mm > 1e-6 {
            eps_cu * (d_t - c_mm) / c_mm
        } else {
            1.0 // pure tension
        };

        // Strength reduction factor phi per SNI 2847:2019 Pasal 21.2.2
        let phi = if sec.is_spiral {
            if eps_t <= eps_y {
                0.75
            } else if eps_t < eps_y + 0.003 {
                0.75 + 0.15 * (eps_t - eps_y) / 0.003
            } else {
                0.90
            }
        } else {
            if eps_t <= eps_y {
                0.65
            } else if eps_t < eps_y + 0.003 {
                0.65 + 0.25 * (eps_t - eps_y) / 0.003
            } else {
                0.90
            }
        };

        // Compute pure axial cutoff P0 for cap
        let ag = b * h;
        let ast: f64 = sec.rebar_layers.iter().map(|l| l.total_area_mm2).sum();
        let p0_kn = (0.85 * sec.fc_mpa * (ag - ast) + sec.fy_mpa * ast) * 1e-3;
        let phi_comp = if sec.is_spiral { 0.75 } else { 0.65 };
        let pn_max_factor = if sec.is_spiral { 0.85 } else { 0.80 };
        let phi_pn_max_kn = phi_comp * pn_max_factor * p0_kn;

        let phi_pn_raw = phi * pn_kn;
        let phi_pn_kn = phi_pn_raw.min(phi_pn_max_kn);
        let phi_mn_knm = phi * mn_knm;

        InteractionPoint {
            c_mm,
            pn_kn,
            mn_knm,
            phi,
            phi_pn_kn,
            phi_mn_knm,
            eps_t,
        }
    }

    /// Evaluates the complete P-M interaction diagram including cardinal points & demand check
    pub fn evaluate_pm_diagram(
        sec: &ConcreteSection,
        demand_pu_kn: Option<f64>,
        demand_mu_knm: Option<f64>,
    ) -> ConcretePmReport {
        let b = sec.width_mm;
        let h = sec.height_mm;
        let ag = b * h;
        let ast: f64 = sec.rebar_layers.iter().map(|l| l.total_area_mm2).sum();
        let rho_pct = (ast / ag) * 100.0;
        let beta1 = Self::compute_beta1(sec.fc_mpa);
        let eps_cu = 0.003;
        let eps_y = sec.fy_mpa / sec.es_mpa;
        let d_t = sec
            .rebar_layers
            .iter()
            .map(|l| l.depth_d_mm)
            .fold(0.0_f64, f64::max);

        // 1. Pure Axial Compression Point P0
        let p0_kn = (0.85 * sec.fc_mpa * (ag - ast) + sec.fy_mpa * ast) * 1e-3;
        let pn_max_mult = if sec.is_spiral { 0.85 } else { 0.80 };
        let pn_max_kn = pn_max_mult * p0_kn;
        let phi_comp = if sec.is_spiral { 0.75 } else { 0.65 };
        let phi_pn_max_kn = phi_comp * pn_max_kn;

        // 2. Pure Axial Tension Point Pnt
        let pnt_kn = -(sec.fy_mpa * ast) * 1e-3;
        let phi_pnt_kn = 0.90 * pnt_kn;

        // 3. Balanced Point (c_b)
        let c_b = (eps_cu / (eps_cu + eps_y)) * d_t;
        let balanced_pt = Self::evaluate_section_at_c(sec, c_b);

        // 4. Pure Flexure Point (bisection search for Pn = 0.0)
        let mut c_low = 1.0;
        let mut c_high = c_b;
        for _ in 0..30 {
            let c_mid = (c_low + c_high) * 0.5;
            let pt = Self::evaluate_section_at_c(sec, c_mid);
            if pt.pn_kn > 0.0 {
                c_high = c_mid;
            } else {
                c_low = c_mid;
            }
        }
        let pure_flexure_pt = Self::evaluate_section_at_c(sec, (c_low + c_high) * 0.5);

        let cardinal = CardinalPoints {
            p0_kn,
            pn_max_kn,
            phi_pn_max_kn,
            balanced: balanced_pt.clone(),
            pure_flexure: pure_flexure_pt.clone(),
            pnt_kn,
            phi_pnt_kn,
        };

        // 5. Generate smooth curve across range of c values
        let mut nominal_curve = Vec::new();
        let mut design_curve = Vec::new();

        // Add top pure compression limit
        nominal_curve.push((0.0, p0_kn));
        design_curve.push((0.0, phi_pn_max_kn));

        // Sample c values logarithmically/exponentially from large down to near zero
        let mut c_samples = Vec::new();
        // Compression-dominated
        for i in 0..25 {
            let factor = (i as f64) / 24.0;
            let c = h * 3.0 * (1.0 - factor) + c_b * factor;
            c_samples.push(c);
        }
        // Transition and tension-dominated
        for i in 1..=25 {
            let factor = (i as f64) / 25.0;
            let c = c_b * (1.0 - factor) + (pure_flexure_pt.c_mm * 0.2) * factor;
            c_samples.push(c);
        }

        c_samples.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        for c in c_samples {
            let pt = Self::evaluate_section_at_c(sec, c);
            if pt.mn_knm >= -1e-3 {
                nominal_curve.push((pt.mn_knm.max(0.0), pt.pn_kn));
                design_curve.push((pt.phi_mn_knm.max(0.0), pt.phi_pn_kn));
            }
        }

        // Add pure flexure point explicitly
        nominal_curve.push((pure_flexure_pt.mn_knm, 0.0));
        design_curve.push((pure_flexure_pt.phi_mn_knm, 0.0));

        // Add pure tension point at the end
        nominal_curve.push((0.0, pnt_kn));
        design_curve.push((0.0, phi_pnt_kn));

        // 6. Demand Audit if specified
        let demand_check = if let (Some(pu), Some(mu)) = (demand_pu_kn, demand_mu_knm) {
            let abs_mu = mu.abs();
            let mut best_dcr = 0.0;
            let mut cap_pu = phi_pn_max_kn;
            let mut cap_mu = pure_flexure_pt.phi_mn_knm;
            let mut governing = "Within Combined P-M Envelope".to_string();

            // Direct axial check
            if pu > phi_pn_max_kn {
                best_dcr = pu / phi_pn_max_kn;
                governing = format!("Exceeds Maximum Factored Axial Limit (phi*Pn,max = {:.1} kN)", phi_pn_max_kn);
            } else if pu < phi_pnt_kn {
                best_dcr = pu.abs() / phi_pnt_kn.abs();
                governing = format!("Exceeds Pure Tension Limit (phi*Pnt = {:.1} kN)", phi_pnt_kn);
            } else {
                // Ray intersection from (0, 0) through (abs_mu, pu)
                let demand_mag = (abs_mu.powi(2) + pu.powi(2)).sqrt();
                if demand_mag > 1e-4 {
                    let ray_dx = abs_mu / demand_mag;
                    let ray_dy = pu / demand_mag;

                    // Search along design curve for segment intersection
                    let mut found_cap = false;
                    for w in design_curve.windows(2) {
                        let (m1, p1) = w[0];
                        let (m2, p2) = w[1];
                        // 2D line segment intersection with ray from origin
                        let denom = ray_dx * (p2 - p1) - ray_dy * (m2 - m1);
                        if denom.abs() > 1e-6 {
                            let t = (m1 * (p2 - p1) - p1 * (m2 - m1)) / denom;
                            let s = (m1 * ray_dy - p1 * ray_dx) / (-denom);
                            if t > 0.0 && (0.0..=1.0).contains(&s) {
                                cap_mu = ray_dx * t;
                                cap_pu = ray_dy * t;
                                best_dcr = demand_mag / t;
                                found_cap = true;
                                break;
                            }
                        }
                    }

                    if !found_cap {
                        // Fallback projection onto closest boundary point
                        let mut min_dist_ratio = f64::INFINITY;
                        for &(m_cap, p_cap) in &design_curve {
                            let cap_mag = (m_cap.powi(2) + p_cap.powi(2)).sqrt();
                            if cap_mag > 1e-3 {
                                let ratio = demand_mag / cap_mag;
                                if ratio < min_dist_ratio {
                                    min_dist_ratio = ratio;
                                    cap_mu = m_cap;
                                    cap_pu = p_cap;
                                    best_dcr = ratio;
                                }
                            }
                        }
                    }
                }
            }

            let is_safe = best_dcr <= 1.0;
            if is_safe {
                governing = format!("Aman: DCR = {:.3} ≤ 1.00 (Kapasitas Cadangan: {:.1}%)", best_dcr, (1.0 - best_dcr) * 100.0);
            } else if governing.is_empty() {
                governing = format!("Overstressed: DCR = {:.3} > 1.00 (Kelebihan Beban: {:.1}%)", best_dcr, (best_dcr - 1.0) * 100.0);
            }

            Some(DemandAudit {
                pu_kn: pu,
                mu_knm: abs_mu,
                dcr: best_dcr,
                is_safe,
                governing_limit: governing,
                phi_pn_cap_kn: cap_pu,
                phi_mn_cap_knm: cap_mu,
            })
        } else {
            None
        };

        ConcretePmReport {
            section: sec.clone(),
            ag_mm2: ag,
            ast_mm2: ast,
            rebar_ratio_pct: rho_pct,
            plastic_centroid_y_mm: h * 0.5,
            beta1,
            cardinal,
            nominal_curve,
            design_curve,
            demand_check,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sni_2847_beta1_calculation() {
        assert_eq!(ConcreteEngine::compute_beta1(25.0), 0.85);
        assert_eq!(ConcreteEngine::compute_beta1(28.0), 0.85);
        // At 35 MPa: 0.85 - 0.05 * (35-28)/7 = 0.80
        assert!((ConcreteEngine::compute_beta1(35.0) - 0.80).abs() < 1e-6);
        // At 55 MPa and above: 0.65
        assert_eq!(ConcreteEngine::compute_beta1(55.0), 0.65);
        assert_eq!(ConcreteEngine::compute_beta1(65.0), 0.65);
    }

    #[test]
    fn test_standard_400x400_column_cardinal_points() {
        // Standard 400x400 column, fc = 30 MPa, fy = 420 MPa, 8 D22 rebar
        let col = ConcreteEngine::create_rectangular_column(
            "K1-400x400",
            400.0,
            400.0,
            30.0,
            420.0,
            40.0,
            3, // 3 top, 3 bottom
            1, // 2 intermediate side bars
            22.0, // total = 8 bars
        );

        let report = ConcreteEngine::evaluate_pm_diagram(&col, Some(1500.0), Some(120.0));

        assert_eq!(report.section.rebar_layers.len(), 3);
        // Total 8 D22 bars: Area = 8 * pi * 11^2 = 3041 mm²
        assert!((report.ast_mm2 - 3041.06).abs() < 2.0);
        assert!((report.rebar_ratio_pct - 1.90).abs() < 0.1);

        // Pure compression P0
        assert!(report.cardinal.p0_kn > 5000.0);
        // Pn_max = 0.80 * P0
        assert!((report.cardinal.pn_max_kn - 0.80 * report.cardinal.p0_kn).abs() < 1e-4);
        // phi*Pn_max = 0.65 * Pn_max
        assert!((report.cardinal.phi_pn_max_kn - 0.65 * report.cardinal.pn_max_kn).abs() < 1e-4);

        // Balanced point
        assert!(report.cardinal.balanced.pn_kn > 1000.0);
        assert!(report.cardinal.balanced.mn_knm > 150.0);

        // Pure flexure
        assert!(report.cardinal.pure_flexure.pn_kn.abs() < 5.0);
        assert!(report.cardinal.pure_flexure.mn_knm > 150.0);

        // Demand audit
        let audit = report.demand_check.unwrap();
        assert!(audit.dcr > 0.0);
        assert!(audit.dcr < 1.0); // 1500 kN, 120 kNm is well within 400x400 capacity
        assert!(audit.is_safe);
    }

    #[test]
    fn test_demand_audit_overstressed_detection() {
        let col = ConcreteEngine::create_rectangular_column(
            "K1-300x300",
            300.0,
            300.0,
            25.0,
            420.0,
            40.0,
            2,
            0,
            16.0,
        );

        // Extreme demand that definitely exceeds small 300x300 section (4 D16)
        let report = ConcreteEngine::evaluate_pm_diagram(&col, Some(3500.0), Some(450.0));
        let audit = report.demand_check.unwrap();
        assert!(audit.dcr > 1.0);
        assert!(!audit.is_safe);
    }
}
