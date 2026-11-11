//! gis/mod.rs — GIS Site Inception & Geotechnical Foundation Suite (Phases 23–26).
//!
//! Provides Web Mercator Slippy Map Quadtree Engine (`walkers`), Geodetic CRS transformations (`proj4rs`),
//! Digital Elevation Model (DEM) TIN Surface Meshing, Borehole Stratigraphy, and PusGeN Seismic Spectrum Integration.

pub mod map_widget;
pub mod crs_projection;
pub mod tin_surface;

pub use map_widget::{
    SiteGeodeticPin, SlippyMapState, SlippyMapTelemetry, TileSource, WebMercator,
    EARTH_HALF_CIRCUMFERENCE, EARTH_RADIUS_METERS, MAX_LATITUDE, MIN_LATITUDE,
};

pub use crs_projection::{
    GeoProjectionEngine, IfcMapConversionMatrix, UtmZoneInfo,
    UTM_FALSE_EASTING, UTM_FALSE_NORTHING_SOUTH, UTM_SCALE_FACTOR_CENTRAL,
    WGS84_FIRST_ECCENTRICITY_SQ, WGS84_FLATTENING, WGS84_SECOND_ECCENTRICITY_SQ,
    WGS84_SEMI_MAJOR_AXIS, WGS84_SEMI_MINOR_AXIS,
};

pub use tin_surface::{
    ContourPolyline, CutFillVolumeReport, DemTerrainPreset, TinPoint, TinSurfaceMesh, TinTriangle,
};
