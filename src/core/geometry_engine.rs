//! geometry_engine.rs — Advanced Modeling & Computational Geometry Engine.
//! Provides Green's Theorem arbitrary cross-section integration (Area, Centroids, Principal Inertias I1/I2,
//! Torsional constants J/Jt, Plastic section moduli Zx/Zy), Parametric Spatial Structure Generators
//! (3D Double-Layer Space Trusses, Geodesic Lattice Domes, Hypar Ruled Shells, Multi-Story Building Frames),
//! Topological Mesh Consolidation, and 3D Wavefront OBJ / 3D DXF Mesh Exporters.

use serde::{Deserialize, Serialize};
use crate::core::model::{
    BoundaryFixity, CrossSection, Element3D, ElementType, Material, Node3D, ShellElement4N,
    StructuralModel,
};

/// 2D Point on a Cross-Section Polygon Boundary (in millimetres)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PolygonPoint2D {
    pub x: f64,
    pub y: f64,
}

impl PolygonPoint2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Comprehensive Arbitrary Cross-Section Properties evaluated via Green's Theorem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionPropertiesReport {
    pub section_name: String,
    pub area_mm2: f64,
    pub perimeter_mm: f64,
    pub centroid_x_mm: f64,
    pub centroid_y_mm: f64,
    pub ixx_centroidal_mm4: f64,
    pub iyy_centroidal_mm4: f64,
    pub ixy_centroidal_mm4: f64,
    pub i1_principal_mm4: f64,
    pub i2_principal_mm4: f64,
    pub principal_angle_deg: f64,
    pub radius_gyration_rx_mm: f64,
    pub radius_gyration_ry_mm: f64,
    pub elastic_modulus_sx_top_mm3: f64,
    pub elastic_modulus_sx_bot_mm3: f64,
    pub elastic_modulus_sy_left_mm3: f64,
    pub elastic_modulus_sy_right_mm3: f64,
    pub plastic_modulus_zx_mm3: f64,
    pub plastic_modulus_zy_mm3: f64,
    pub polar_inertia_j_mm4: f64,
    pub torsional_constant_jt_mm4: f64,
    pub bounding_width_mm: f64,
    pub bounding_height_mm: f64,
    pub boundary_points: Vec<PolygonPoint2D>,
}

/// Cross-Section Geometry Processor using Exact Line Integrals (Green's Theorem)
pub struct SectionIntegrator;

impl SectionIntegrator {
    /// Evaluates exact section properties for any arbitrary closed 2D polygon.
    /// Vertices can be ordered counter-clockwise or clockwise (orientation automatically adjusted).
    pub fn evaluate_polygon(name: impl Into<String>, points: &[PolygonPoint2D]) -> Result<SectionPropertiesReport, String> {
        let n = points.len();
        if n < 3 {
            return Err("A valid polygon requires at least 3 vertices".to_string());
        }

        let section_name = name.into();
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut perimeter = 0.0;

        for i in 0..n {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % n];
            min_x = min_x.min(p1.x);
            max_x = max_x.max(p1.x);
            min_y = min_y.min(p1.y);
            max_y = max_y.max(p1.y);
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        // Green's Theorem Line Integrals for Area, First Moments (Qx, Qy), and Second Moments (Ixx0, Iyy0, Ixy0)
        let mut a_signed = 0.0;
        let mut qx_raw = 0.0;
        let mut qy_raw = 0.0;
        let mut ixx_raw = 0.0;
        let mut iyy_raw = 0.0;
        let mut ixy_raw = 0.0;

        for i in 0..n {
            let x0 = points[i].x;
            let y0 = points[i].y;
            let x1 = points[(i + 1) % n].x;
            let y1 = points[(i + 1) % n].y;

            let cross = x0 * y1 - x1 * y0;
            a_signed += cross;
            qx_raw += (y0 + y1) * cross;
            qy_raw += (x0 + x1) * cross;
            ixx_raw += (y0 * y0 + y0 * y1 + y1 * y1) * cross;
            iyy_raw += (x0 * x0 + x0 * x1 + x1 * x1) * cross;
            ixy_raw += (x0 * y1 + 2.0 * x0 * y0 + 2.0 * x1 * y1 + x1 * y0) * cross;
        }

        let area = 0.5 * a_signed.abs();
        if area < 1e-9 {
            return Err("Polygon has degenerate or near-zero area".to_string());
        }

        let sign = if a_signed >= 0.0 { 1.0 } else { -1.0 };
        let cx = (sign * qy_raw) / (6.0 * area);
        let cy = (sign * qx_raw) / (6.0 * area);

        // Moments of inertia about origin
        let ixx0 = (sign * ixx_raw) / 12.0;
        let iyy0 = (sign * iyy_raw) / 12.0;
        let ixy0 = (sign * ixy_raw) / 24.0;

        // Parallel Axis Theorem (Transfer to centroidal axes)
        let ixx = (ixx0 - area * cy * cy).max(0.0);
        let iyy = (iyy0 - area * cx * cx).max(0.0);
        let ixy = ixy0 - area * cx * cy;

        // Principal Moments of Inertia and Angle
        let diff = ixx - iyy;
        let r = ((0.5 * diff) * (0.5 * diff) + ixy * ixy).sqrt();
        let i1 = 0.5 * (ixx + iyy) + r;
        let i2 = (0.5 * (ixx + iyy) - r).max(0.0);

        let theta_p = if diff.abs() > 1e-9 || ixy.abs() > 1e-9 {
            0.5 * (-2.0 * ixy).atan2(diff)
        } else {
            0.0
        };

        let rx = (ixx / area).sqrt();
        let ry = (iyy / area).sqrt();

        // Elastic Section Moduli (S = I / c)
        let c_top = (max_y - cy).max(1e-6);
        let c_bot = (cy - min_y).max(1e-6);
        let c_right = (max_x - cx).max(1e-6);
        let c_left = (cx - min_x).max(1e-6);

        let sx_top = ixx / c_top;
        let sx_bot = ixx / c_bot;
        let sy_right = iyy / c_right;
        let sy_left = iyy / c_left;

        // Approximate Plastic Moduli Zx, Zy via sliced area integration
        let mut zx = 0.0;
        let mut zy = 0.0;
        let n_slices = 200;
        let dy_slice = (max_y - min_y) / n_slices as f64;
        let dx_slice = (max_x - min_x) / n_slices as f64;

        for s in 0..n_slices {
            let y_mid = min_y + (s as f64 + 0.5) * dy_slice;
            // Width of polygon at y_mid
            let mut x_intersects = Vec::new();
            for i in 0..n {
                let p1 = &points[i];
                let p2 = &points[(i + 1) % n];
                if (p1.y <= y_mid && p2.y > y_mid) || (p2.y <= y_mid && p1.y > y_mid) {
                    let t = (y_mid - p1.y) / (p2.y - p1.y);
                    x_intersects.push(p1.x + t * (p2.x - p1.x));
                }
            }
            x_intersects.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut width_at_y = 0.0;
            for chunk in x_intersects.chunks(2) {
                if chunk.len() == 2 {
                    width_at_y += chunk[1] - chunk[0];
                }
            }
            zx += width_at_y * dy_slice * (y_mid - cy).abs();
        }

        for s in 0..n_slices {
            let x_mid = min_x + (s as f64 + 0.5) * dx_slice;
            let mut y_intersects = Vec::new();
            for i in 0..n {
                let p1 = &points[i];
                let p2 = &points[(i + 1) % n];
                if (p1.x <= x_mid && p2.x > x_mid) || (p2.x <= x_mid && p1.x > x_mid) {
                    let t = (x_mid - p1.x) / (p2.x - p1.x);
                    y_intersects.push(p1.y + t * (p2.y - p1.y));
                }
            }
            y_intersects.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut height_at_x = 0.0;
            for chunk in y_intersects.chunks(2) {
                if chunk.len() == 2 {
                    height_at_x += chunk[1] - chunk[0];
                }
            }
            zy += height_at_x * dx_slice * (x_mid - cx).abs();
        }

        let polar_j = ixx + iyy;
        // Saint-Venant torsional constant estimate: Jt ~ (1/3) sum(b * t^3) or area^4 / (40 * J)
        let jt = (area * area * area * area) / (40.0 * polar_j.max(1.0));

        Ok(SectionPropertiesReport {
            section_name,
            area_mm2: area,
            perimeter_mm: perimeter,
            centroid_x_mm: cx,
            centroid_y_mm: cy,
            ixx_centroidal_mm4: ixx,
            iyy_centroidal_mm4: iyy,
            ixy_centroidal_mm4: ixy,
            i1_principal_mm4: i1,
            i2_principal_mm4: i2,
            principal_angle_deg: theta_p.to_degrees(),
            radius_gyration_rx_mm: rx,
            radius_gyration_ry_mm: ry,
            elastic_modulus_sx_top_mm3: sx_top,
            elastic_modulus_sx_bot_mm3: sx_bot,
            elastic_modulus_sy_left_mm3: sy_left,
            elastic_modulus_sy_right_mm3: sy_right,
            plastic_modulus_zx_mm3: zx,
            plastic_modulus_zy_mm3: zy,
            polar_inertia_j_mm4: polar_j,
            torsional_constant_jt_mm4: jt,
            bounding_width_mm: max_x - min_x,
            bounding_height_mm: max_y - min_y,
            boundary_points: points.to_vec(),
        })
    }

    /// Generates standard 2D Wide Flange / I-Beam polygon
    pub fn build_i_section(d_mm: f64, bf_mm: f64, tf_mm: f64, tw_mm: f64) -> Vec<PolygonPoint2D> {
        let half_bf = bf_mm * 0.5;
        let half_tw = tw_mm * 0.5;
        let half_d = d_mm * 0.5;
        let y_inner = half_d - tf_mm;

        vec![
            // Top Flange
            PolygonPoint2D::new(-half_bf, half_d),
            PolygonPoint2D::new(half_bf, half_d),
            PolygonPoint2D::new(half_bf, y_inner),
            PolygonPoint2D::new(half_tw, y_inner),
            // Web right
            PolygonPoint2D::new(half_tw, -y_inner),
            // Bottom Flange right
            PolygonPoint2D::new(half_bf, -y_inner),
            PolygonPoint2D::new(half_bf, -half_d),
            PolygonPoint2D::new(-half_bf, -half_d),
            PolygonPoint2D::new(-half_bf, -y_inner),
            // Web left
            PolygonPoint2D::new(-half_tw, -y_inner),
            PolygonPoint2D::new(-half_tw, y_inner),
            // Top Flange left
            PolygonPoint2D::new(-half_bf, y_inner),
        ]
    }

    /// Generates standard 2D Channel (C-Shape) polygon
    pub fn build_channel_section(d_mm: f64, bf_mm: f64, tf_mm: f64, tw_mm: f64) -> Vec<PolygonPoint2D> {
        vec![
            PolygonPoint2D::new(0.0, 0.0),
            PolygonPoint2D::new(bf_mm, 0.0),
            PolygonPoint2D::new(bf_mm, tf_mm),
            PolygonPoint2D::new(tw_mm, tf_mm),
            PolygonPoint2D::new(tw_mm, d_mm - tf_mm),
            PolygonPoint2D::new(bf_mm, d_mm - tf_mm),
            PolygonPoint2D::new(bf_mm, d_mm),
            PolygonPoint2D::new(0.0, d_mm),
        ]
    }

    /// Generates standard 2D Angle (L-Shape) polygon
    pub fn build_angle_section(h_mm: f64, b_mm: f64, t_mm: f64) -> Vec<PolygonPoint2D> {
        vec![
            PolygonPoint2D::new(0.0, 0.0),
            PolygonPoint2D::new(b_mm, 0.0),
            PolygonPoint2D::new(b_mm, t_mm),
            PolygonPoint2D::new(t_mm, t_mm),
            PolygonPoint2D::new(t_mm, h_mm),
            PolygonPoint2D::new(0.0, h_mm),
        ]
    }

    /// Generates standard 2D T-Shape polygon
    pub fn build_t_section(d_mm: f64, bf_mm: f64, tf_mm: f64, tw_mm: f64) -> Vec<PolygonPoint2D> {
        let half_bf = bf_mm * 0.5;
        let half_tw = tw_mm * 0.5;

        vec![
            PolygonPoint2D::new(-half_bf, d_mm),
            PolygonPoint2D::new(half_bf, d_mm),
            PolygonPoint2D::new(half_bf, d_mm - tf_mm),
            PolygonPoint2D::new(half_tw, d_mm - tf_mm),
            PolygonPoint2D::new(half_tw, 0.0),
            PolygonPoint2D::new(-half_tw, 0.0),
            PolygonPoint2D::new(-half_tw, d_mm - tf_mm),
            PolygonPoint2D::new(-half_bf, d_mm - tf_mm),
        ]
    }
}

/// 3D Spatial Geometry & Parametric Structural Topology Generator Wizard
pub struct ParametricSpatialWizard;

impl ParametricSpatialWizard {
    /// Generates a 3D Double-Layer Space Truss / Space Frame (MERO Ball-Node Octet Topology)
    /// - Lx, Ly: Overall plan dimensions in meters
    /// - Depth: Truss structural depth in meters
    /// - Nx, Ny: Number of grid divisions in X and Y directions
    pub fn generate_double_layer_space_truss(
        name: &str,
        lx: f64,
        ly: f64,
        depth: f64,
        nx: usize,
        ny: usize,
    ) -> StructuralModel {
        let mut model = StructuralModel::new(name);
        let dx = lx / nx as f64;
        let dy = ly / ny as f64;
        let material = Material::structural_steel();
        let top_section = CrossSection::tube_chs_300x10();
        let bot_section = CrossSection::pipe_chs_150x6();

        // 1. Top Chord Nodes (at z = depth)
        // Node indexing: 0 .. (nx+1)*(ny+1) - 1
        let mut top_node_ids = Vec::with_capacity((nx + 1) * (ny + 1));
        let mut node_counter = 0;

        for iy in 0..=ny {
            let mut row = Vec::with_capacity(nx + 1);
            for ix in 0..=nx {
                let x = ix as f64 * dx;
                let y = iy as f64 * dy;
                let z = depth;
                // Corner perimeter supports
                let fixity = if (ix == 0 || ix == nx) && (iy == 0 || iy == ny) {
                    BoundaryFixity::pinned()
                } else {
                    BoundaryFixity::free()
                };
                model.add_node(Node3D::new(node_counter, x, y, z).with_fixity(fixity));
                row.push(node_counter);
                node_counter += 1;
            }
            top_node_ids.push(row);
        }

        // 2. Bottom Chord Nodes (at z = 0, offset by dx/2, dy/2)
        // Node indexing: nx*ny nodes
        let mut bot_node_ids = Vec::with_capacity(nx * ny);
        for iy in 0..ny {
            let mut row = Vec::with_capacity(nx);
            for ix in 0..nx {
                let x = (ix as f64 + 0.5) * dx;
                let y = (iy as f64 + 0.5) * dy;
                let z = 0.0;
                model.add_node(Node3D::new(node_counter, x, y, z));
                row.push(node_counter);
                node_counter += 1;
            }
            bot_node_ids.push(row);
        }

        let mut elem_id = 0;

        // 3. Top Chord Members (Orthogonal Grid X & Y)
        for iy in 0..=ny {
            for ix in 0..nx {
                let n1 = top_node_ids[iy][ix];
                let n2 = top_node_ids[iy][ix + 1];
                model.add_element(
                    Element3D::new(elem_id, format!("TC-X-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Truss3D)
                        .with_material(material.clone())
                        .with_section(top_section.clone()),
                );
                elem_id += 1;
            }
        }
        for iy in 0..ny {
            for ix in 0..=nx {
                let n1 = top_node_ids[iy][ix];
                let n2 = top_node_ids[iy + 1][ix];
                model.add_element(
                    Element3D::new(elem_id, format!("TC-Y-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Truss3D)
                        .with_material(material.clone())
                        .with_section(top_section.clone()),
                );
                elem_id += 1;
            }
        }

        // 4. Bottom Chord Members (Orthogonal Grid X & Y)
        for iy in 0..ny {
            for ix in 0..(nx - 1) {
                let n1 = bot_node_ids[iy][ix];
                let n2 = bot_node_ids[iy][ix + 1];
                model.add_element(
                    Element3D::new(elem_id, format!("BC-X-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Truss3D)
                        .with_material(material.clone())
                        .with_section(bot_section.clone()),
                );
                elem_id += 1;
            }
        }
        for iy in 0..(ny - 1) {
            for ix in 0..nx {
                let n1 = bot_node_ids[iy][ix];
                let n2 = bot_node_ids[iy + 1][ix];
                model.add_element(
                    Element3D::new(elem_id, format!("BC-Y-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Truss3D)
                        .with_material(material.clone())
                        .with_section(bot_section.clone()),
                );
                elem_id += 1;
            }
        }

        // 5. Diagonal Web Bracing Members (Pyramidal Octet Unit)
        for iy in 0..ny {
            for ix in 0..nx {
                let bot_node = bot_node_ids[iy][ix];
                let top_corners = [
                    top_node_ids[iy][ix],
                    top_node_ids[iy][ix + 1],
                    top_node_ids[iy + 1][ix + 1],
                    top_node_ids[iy + 1][ix],
                ];
                for (k, &top_n) in top_corners.iter().enumerate() {
                    model.add_element(
                        Element3D::new(elem_id, format!("WEB-{}-{}-{}", ix, iy, k), bot_node, top_n)
                            .with_type(ElementType::Truss3D)
                            .with_material(material.clone())
                            .with_section(bot_section.clone()),
                    );
                    elem_id += 1;
                }
            }
        }

        model
    }

    /// Generates a 3D Geodesic / Schwedler Lattice Dome
    /// - Radius: Dome radius in meters
    /// - Rings: Number of latitude rings from base to apex
    /// - Sectors: Number of azimuthal sectors (e.g. 12 or 16)
    pub fn generate_geodesic_dome(
        name: &str,
        radius: f64,
        rings: usize,
        sectors: usize,
    ) -> StructuralModel {
        let mut model = StructuralModel::new(name);
        let material = Material::structural_steel();
        let section = CrossSection::pipe_chs_150x6();

        let mut node_counter = 0;
        let mut ring_nodes: Vec<Vec<usize>> = Vec::with_capacity(rings + 1);

        // Generate rings from base (phi = 0) to top (phi = pi/2)
        for r in 0..=rings {
            let mut current_ring = Vec::with_capacity(sectors);
            let phi = (r as f64 / rings as f64) * (std::f64::consts::FRAC_PI_2 * 0.85); // 0 to ~76.5 deg
            let z = radius * phi.sin();
            let r_ring = radius * phi.cos();

            if r == rings {
                // Crown apex single node
                let fixity = BoundaryFixity::free();
                let apex_node = Node3D::new(node_counter, 0.0, 0.0, radius).with_fixity(fixity);
                model.add_node(apex_node);
                current_ring.push(node_counter);
                node_counter += 1;
            } else {
                for s in 0..sectors {
                    let theta = (s as f64 / sectors as f64) * 2.0 * std::f64::consts::PI;
                    let x = r_ring * theta.cos();
                    let y = r_ring * theta.sin();

                    let fixity = if r == 0 {
                        BoundaryFixity::pinned()
                    } else {
                        BoundaryFixity::free()
                    };

                    model.add_node(Node3D::new(node_counter, x, y, z).with_fixity(fixity));
                    current_ring.push(node_counter);
                    node_counter += 1;
                }
            }
            ring_nodes.push(current_ring);
        }

        let mut elem_id = 0;

        // Hoop ring members (horizontal circles)
        for r in 0..rings {
            for s in 0..sectors {
                let n1 = ring_nodes[r][s];
                let n2 = ring_nodes[r][(s + 1) % sectors];
                model.add_element(
                    Element3D::new(elem_id, format!("HOOP-R{}-S{}", r, s), n1, n2)
                        .with_type(ElementType::Truss3D)
                        .with_material(material.clone())
                        .with_section(section.clone()),
                );
                elem_id += 1;
            }
        }

        // Meridional & Diagonal triangulation members
        for r in 0..rings {
            let next_r = r + 1;
            if next_r == rings {
                // Connect to apex
                let apex = ring_nodes[rings][0];
                for s in 0..sectors {
                    let n1 = ring_nodes[r][s];
                    model.add_element(
                        Element3D::new(elem_id, format!("APEX-S{}", s), n1, apex)
                            .with_type(ElementType::Truss3D)
                            .with_material(material.clone())
                            .with_section(section.clone()),
                    );
                    elem_id += 1;
                }
            } else {
                for s in 0..sectors {
                    let n1 = ring_nodes[r][s];
                    let n2 = ring_nodes[next_r][s];
                    let n3 = ring_nodes[next_r][(s + 1) % sectors];

                    // Meridional rib
                    model.add_element(
                        Element3D::new(elem_id, format!("RIB-R{}-S{}", r, s), n1, n2)
                            .with_type(ElementType::Truss3D)
                            .with_material(material.clone())
                            .with_section(section.clone()),
                    );
                    elem_id += 1;

                    // Cross diagonal
                    model.add_element(
                        Element3D::new(elem_id, format!("DIAG-R{}-S{}", r, s), n1, n3)
                            .with_type(ElementType::Truss3D)
                            .with_material(material.clone())
                            .with_section(section.clone()),
                    );
                    elem_id += 1;
                }
            }
        }

        model
    }

    /// Generates a Hyperbolic Paraboloid (Hypar Saddle / Inverted Umbrella Ruled Shell)
    /// Ruled equation: z = 4 * sagitta * (x/Lx - 0.5) * (y/Ly - 0.5)
    pub fn generate_hypar_shell(
        name: &str,
        lx: f64,
        ly: f64,
        sagitta: f64,
        nx: usize,
        ny: usize,
    ) -> StructuralModel {
        let mut model = StructuralModel::new(name);
        let dx = lx / nx as f64;
        let dy = ly / ny as f64;
        let material = Material::structural_steel();
        let section = CrossSection::wf_200x100();

        let mut node_grid = Vec::with_capacity(ny + 1);
        let mut node_counter = 0;

        for iy in 0..=ny {
            let mut row = Vec::with_capacity(nx + 1);
            let y = iy as f64 * dy;
            let eta = (y / ly) - 0.5;

            for ix in 0..=nx {
                let x = ix as f64 * dx;
                let xi = (x / lx) - 0.5;
                let z = 4.0 * sagitta * xi * eta;

                // Restrain corners
                let fixity = if (ix == 0 || ix == nx) && (iy == 0 || iy == ny) {
                    BoundaryFixity::pinned()
                } else {
                    BoundaryFixity::free()
                };

                model.add_node(Node3D::new(node_counter, x, y, z).with_fixity(fixity));
                row.push(node_counter);
                node_counter += 1;
            }
            node_grid.push(row);
        }

        let mut elem_id = 0;

        // X-Direction Ruled Lines
        for iy in 0..=ny {
            for ix in 0..nx {
                let n1 = node_grid[iy][ix];
                let n2 = node_grid[iy][ix + 1];
                model.add_element(
                    Element3D::new(elem_id, format!("HYPAR-X-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Frame3D)
                        .with_material(material.clone())
                        .with_section(section.clone()),
                );
                elem_id += 1;
            }
        }

        // Y-Direction Ruled Lines
        for iy in 0..ny {
            for ix in 0..=nx {
                let n1 = node_grid[iy][ix];
                let n2 = node_grid[iy + 1][ix];
                model.add_element(
                    Element3D::new(elem_id, format!("HYPAR-Y-{}-{}", ix, iy), n1, n2)
                        .with_type(ElementType::Frame3D)
                        .with_material(material.clone())
                        .with_section(section.clone()),
                );
                elem_id += 1;
            }
        }

        // Add 4-Node Quad Shell Panels
        let mut shell_id = 0;
        for iy in 0..ny {
            for ix in 0..nx {
                let n0 = node_grid[iy][ix];
                let n1 = node_grid[iy][ix + 1];
                let n2 = node_grid[iy + 1][ix + 1];
                let n3 = node_grid[iy + 1][ix];
                model.add_shell(ShellElement4N::new(
                    shell_id,
                    format!("PANEL-{}-{}", ix, iy),
                    [n0, n1, n2, n3],
                    0.08, // 80mm shell
                ));
                shell_id += 1;
            }
        }

        model
    }

    /// Generates a Multi-Story 3D Building Frame with Concrete Core Shear Walls
    pub fn generate_multi_story_tower(
        name: &str,
        stories: usize,
        story_height: f64,
        bays_x: usize,
        bays_y: usize,
        bay_width_x: f64,
        bay_width_y: f64,
    ) -> StructuralModel {
        let mut model = StructuralModel::new(name);
        let col_section = CrossSection::box_200x200x8();
        let beam_section = CrossSection::wf_300x150();
        let mat = Material::structural_steel();

        let mut node_counter = 0;
        let mut floor_grids = Vec::with_capacity(stories + 1);

        for level in 0..=stories {
            let z = level as f64 * story_height;
            let mut grid = Vec::with_capacity(bays_y + 1);

            for iy in 0..=bays_y {
                let y = iy as f64 * bay_width_y;
                let mut row = Vec::with_capacity(bays_x + 1);
                for ix in 0..=bays_x {
                    let x = ix as f64 * bay_width_x;
                    let fixity = if level == 0 {
                        BoundaryFixity::fixed()
                    } else {
                        BoundaryFixity::free()
                    };
                    model.add_node(Node3D::new(node_counter, x, y, z).with_fixity(fixity));
                    row.push(node_counter);
                    node_counter += 1;
                }
                grid.push(row);
            }
            floor_grids.push(grid);
        }

        let mut elem_id = 0;

        // Vertical Columns
        for level in 0..stories {
            for iy in 0..=bays_y {
                for ix in 0..=bays_x {
                    let bot_node = floor_grids[level][iy][ix];
                    let top_node = floor_grids[level + 1][iy][ix];
                    model.add_element(
                        Element3D::new(elem_id, format!("COL-L{}-{}-{}", level + 1, ix, iy), bot_node, top_node)
                            .with_type(ElementType::Frame3D)
                            .with_material(mat.clone())
                            .with_section(col_section.clone()),
                    );
                    elem_id += 1;
                }
            }
        }

        // Horizontal Floor Beams (X and Y)
        for level in 1..=stories {
            for iy in 0..=bays_y {
                for ix in 0..bays_x {
                    let n1 = floor_grids[level][iy][ix];
                    let n2 = floor_grids[level][iy][ix + 1];
                    model.add_element(
                        Element3D::new(elem_id, format!("BMX-L{}-{}-{}", level, ix, iy), n1, n2)
                            .with_type(ElementType::Frame3D)
                            .with_material(mat.clone())
                            .with_section(beam_section.clone()),
                    );
                    elem_id += 1;
                }
            }
            for iy in 0..bays_y {
                for ix in 0..=bays_x {
                    let n1 = floor_grids[level][iy][ix];
                    let n2 = floor_grids[level][iy + 1][ix];
                    model.add_element(
                        Element3D::new(elem_id, format!("BMY-L{}-{}-{}", level, ix, iy), n1, n2)
                            .with_type(ElementType::Frame3D)
                            .with_material(mat.clone())
                            .with_section(beam_section.clone()),
                    );
                    elem_id += 1;
                }
            }
        }

        // Central Shear Wall Shell Panels (in center bay)
        if bays_x >= 2 && bays_y >= 2 {
            let cx = bays_x / 2;
            let cy = bays_y / 2;
            let mut shell_id = 0;
            for level in 0..stories {
                let n0 = floor_grids[level][cy][cx];
                let n1 = floor_grids[level][cy][cx + 1];
                let n2 = floor_grids[level + 1][cy][cx + 1];
                let n3 = floor_grids[level + 1][cy][cx];
                model.add_shell(ShellElement4N::new(
                    shell_id,
                    format!("CORE-WALL-L{}-X", level + 1),
                    [n0, n1, n2, n3],
                    0.25, // 250mm concrete shear wall
                ));
                shell_id += 1;
            }
        }

        model
    }
}

/// Mesh Topology Cleaner & Exporter Suite
pub struct MeshTopologyEngine;

impl MeshTopologyEngine {
    /// Consolidates duplicate nodes located within a spatial distance tolerance (epsilon).
    /// Rewrites member and shell connectivity to eliminate zero-length elements and duplicate vertices.
    pub fn consolidate_nodes(model: &mut StructuralModel, tol_m: f64) -> usize {
        let n_orig = model.nodes.len();
        if n_orig == 0 {
            return 0;
        }

        let mut unique_nodes: Vec<Node3D> = Vec::new();
        let mut id_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

        for old_node in &model.nodes {
            let mut found_id = None;
            for u in &unique_nodes {
                let dist = old_node.distance_to(u);
                if dist <= tol_m {
                    found_id = Some(u.id);
                    break;
                }
            }
            if let Some(target_id) = found_id {
                id_map.insert(old_node.id, target_id);
            } else {
                let new_id = unique_nodes.len();
                id_map.insert(old_node.id, new_id);
                let mut copied = old_node.clone();
                copied.id = new_id;
                unique_nodes.push(copied);
            }
        }

        // Remap element connectivity
        for elem in &mut model.elements {
            if let Some(&new_start) = id_map.get(&elem.start_node) {
                elem.start_node = new_start;
            }
            if let Some(&new_end) = id_map.get(&elem.end_node) {
                elem.end_node = new_end;
            }
        }

        // Filter out zero-length elements created by node merger
        model.elements.retain(|e| e.start_node != e.end_node);

        // Remap shell connectivity
        for shell in &mut model.shells {
            for i in 0..4 {
                if let Some(&new_nid) = id_map.get(&shell.nodes[i]) {
                    shell.nodes[i] = new_nid;
                }
            }
        }

        model.nodes = unique_nodes;
        n_orig.saturating_sub(model.nodes.len())
    }

    /// Serializes structural topology into a certified Wavefront 3D OBJ file format.
    pub fn export_to_obj(model: &StructuralModel) -> String {
        let mut out = String::with_capacity(16384);
        out.push_str("# EigenGraph 3D Structural Mesh Export (Wavefront OBJ)\n");
        out.push_str(&format!("# Model: {}\n", model.name));
        out.push_str(&format!("# Nodes: {}, Elements: {}, Shells: {}\n\n", model.nodes.len(), model.elements.len(), model.shells.len()));

        // Write vertices
        for node in &model.nodes {
            out.push_str(&format!("v {:.6} {:.6} {:.6}\n", node.x, node.y, node.z));
        }
        out.push('\n');

        // Write line elements (1-indexed in OBJ)
        out.push_str("g Structural_Frame_Elements\n");
        for elem in &model.elements {
            out.push_str(&format!("l {} {}\n", elem.start_node + 1, elem.end_node + 1));
        }
        out.push('\n');

        // Write shell quad faces
        if !model.shells.is_empty() {
            out.push_str("g Structural_Shell_Panels\n");
            for shell in &model.shells {
                out.push_str(&format!(
                    "f {} {} {} {}\n",
                    shell.nodes[0] + 1,
                    shell.nodes[1] + 1,
                    shell.nodes[2] + 1,
                    shell.nodes[3] + 1
                ));
            }
        }

        out
    }

    /// Serializes structural model to AutoCAD 3D DXF format with LINE and 3DFACE entities.
    pub fn export_to_dxf_3d(model: &StructuralModel) -> String {
        let mut dxf = String::with_capacity(32768);
        dxf.push_str("0\nSECTION\n2\nHEADER\n0\nENDSEC\n");
        dxf.push_str("0\nSECTION\n2\nTABLES\n0\nENDSEC\n");
        dxf.push_str("0\nSECTION\n2\nBLOCKS\n0\nENDSEC\n");
        dxf.push_str("0\nSECTION\n2\nENTITIES\n");

        for elem in &model.elements {
            if let (Some(n1), Some(n2)) = (model.find_node(elem.start_node), model.find_node(elem.end_node)) {
                dxf.push_str("0\nLINE\n8\nFRAME_MEMBERS\n");
                dxf.push_str(&format!("10\n{:.4}\n20\n{:.4}\n30\n{:.4}\n", n1.x, n1.y, n1.z));
                dxf.push_str(&format!("11\n{:.4}\n21\n{:.4}\n31\n{:.4}\n", n2.x, n2.y, n2.z));
            }
        }

        for shell in &model.shells {
            if let (Some(n0), Some(n1), Some(n2), Some(n3)) = (
                model.find_node(shell.nodes[0]),
                model.find_node(shell.nodes[1]),
                model.find_node(shell.nodes[2]),
                model.find_node(shell.nodes[3]),
            ) {
                dxf.push_str("0\n3DFACE\n8\nSHELL_PANELS\n");
                dxf.push_str(&format!("10\n{:.4}\n20\n{:.4}\n30\n{:.4}\n", n0.x, n0.y, n0.z));
                dxf.push_str(&format!("11\n{:.4}\n21\n{:.4}\n31\n{:.4}\n", n1.x, n1.y, n1.z));
                dxf.push_str(&format!("12\n{:.4}\n22\n{:.4}\n32\n{:.4}\n", n2.x, n2.y, n2.z));
                dxf.push_str(&format!("13\n{:.4}\n23\n{:.4}\n33\n{:.4}\n", n3.x, n3.y, n3.z));
            }
        }

        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        dxf
    }

    /// Generates markdown audit report of cross-section properties and spatial structure metrics
    pub fn generate_markdown_dossier(
        model: &StructuralModel,
        section_rep: Option<&SectionPropertiesReport>,
    ) -> String {
        let mut md = String::with_capacity(4096);
        md.push_str("# 📐 EigenGraph Computational Geometry & Spatial Modeling Dossier\n\n");
        md.push_str(&format!("**Project / Model Name:** `{}`  \n", model.name));
        md.push_str(&format!("**Total Spatial Nodes:** `{}`  \n", model.nodes.len()));
        md.push_str(&format!("**Total Frame/Truss Elements:** `{}`  \n", model.elements.len()));
        md.push_str(&format!("**Total 4-Node Shell Panels:** `{}`  \n", model.shells.len()));
        md.push_str("\n---\n\n");

        if let Some(sec) = section_rep {
            md.push_str("## 1. Green's Theorem Arbitrary Cross-Section Properties\n\n");
            md.push_str(&format!("- **Section Profile:** `{}`\n", sec.section_name));
            md.push_str(&format!("- **Cross-Section Area ($A$):** `{:.1} mm²` ({:.2} cm²)\n", sec.area_mm2, sec.area_mm2 / 100.0));
            md.push_str(&format!("- **Perimeter:** `{:.1} mm`\n", sec.perimeter_mm));
            md.push_str(&format!("- **Centroid $(\\bar{{x}}, \\bar{{y}}):$** `({:.2}, {:.2}) mm`\n", sec.centroid_x_mm, sec.centroid_y_mm));
            md.push_str(&format!("- **Centroidal Inertia ($I_{{xx}}$):** `{:.2e} mm⁴`\n", sec.ixx_centroidal_mm4));
            md.push_str(&format!("- **Centroidal Inertia ($I_{{yy}}$):** `{:.2e} mm⁴`\n", sec.iyy_centroidal_mm4));
            md.push_str(&format!("- **Product of Inertia ($I_{{xy}}$):** `{:.2e} mm⁴`\n", sec.ixy_centroidal_mm4));
            md.push_str(&format!("- **Principal Moments ($I_1, I_2$):** `{:.2e} mm⁴` / `{:.2e} mm⁴`\n", sec.i1_principal_mm4, sec.i2_principal_mm4));
            md.push_str(&format!("- **Principal Axis Orientation ($\\theta_p$):** `{:.2}°`\n", sec.principal_angle_deg));
            md.push_str(&format!("- **Radius of Gyration ($r_x, r_y$):** `{:.2} mm` / `{:.2} mm`\n", sec.radius_gyration_rx_mm, sec.radius_gyration_ry_mm));
            md.push_str(&format!("- **Elastic Modulus ($S_x, S_y$):** `{:.1} mm³` / `{:.1} mm³`\n", sec.elastic_modulus_sx_top_mm3, sec.elastic_modulus_sy_right_mm3));
            md.push_str(&format!("- **Plastic Modulus ($Z_x, Z_y$):** `{:.1} mm³` / `{:.1} mm³`\n", sec.plastic_modulus_zx_mm3, sec.plastic_modulus_zy_mm3));
            md.push_str(&format!("- **Polar Moment of Inertia ($J$):** `{:.2e} mm⁴`\n", sec.polar_inertia_j_mm4));
            md.push_str(&format!("- **Torsional Constant ($J_t$):** `{:.2e} mm⁴`\n\n", sec.torsional_constant_jt_mm4));
        }

        md.push_str("## 2. Spatial Topology & Structural Elements Inventory\n\n");
        md.push_str("| Member ID | Tag | Type | Start Node | End Node |\n");
        md.push_str("| :---: | :--- | :---: | :---: | :---: |\n");
        for elem in model.elements.iter().take(20) {
            md.push_str(&format!(
                "| #{} | `{}` | `{:?}` | Node {} | Node {} |\n",
                elem.id, elem.name, elem.element_type, elem.start_node, elem.end_node
            ));
        }
        if model.elements.len() > 20 {
            md.push_str(&format!("| ... | *+ {} more members* | ... | ... | ... |\n", model.elements.len() - 20));
        }

        md.push_str("\n---\n\n");
        md.push_str("## 3. Mathematical Reference Formulas\n\n");
        md.push_str("$$\\text{Area: } A = \\frac{1}{2} \\sum_{i=0}^{n-1} (x_i y_{i+1} - x_{i+1} y_i)$$\n");
        md.push_str("$$\\text{Centroid: } \\bar{x} = \\frac{1}{6A} \\sum_{i=0}^{n-1} (x_i + x_{i+1}) (x_i y_{i+1} - x_{i+1} y_i), \\quad \\bar{y} = \\frac{1}{6A} \\sum_{i=0}^{n-1} (y_i + y_{i+1}) (x_i y_{i+1} - x_{i+1} y_i)$$\n");
        md.push_str("$$I_{xx} = \\frac{1}{12} \\sum_{i=0}^{n-1} (y_i^2 + y_i y_{i+1} + y_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i) - A \\bar{y}^2$$\n");
        md.push_str("$$I_{yy} = \\frac{1}{12} \\sum_{i=0}^{n-1} (x_i^2 + x_i x_{i+1} + x_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i) - A \\bar{x}^2$$\n");
        md.push_str("$$\\theta_p = \\frac{1}{2} \\operatorname{atan2}(-2 I_{xy}, \\; I_{xx} - I_{yy})$$\n");

        md
    }
}
