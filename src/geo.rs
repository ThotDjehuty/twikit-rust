use pyo3::prelude::*;

#[pyclass]
pub struct Geo {}

#[pymethods]
impl Geo {
    #[new]
    pub fn new() -> Self {
        Geo {}
    }

    /// Calculate haversine distance between two (lat, lon) pairs in kilometers
    pub fn distance(&self, a: (f64, f64), b: (f64, f64)) -> PyResult<f64> {
        let (lat1, lon1) = a;
        let (lat2, lon2) = b;
        let to_rad = |deg: f64| deg.to_radians();
        let dlat = to_rad(lat2 - lat1);
        let dlon = to_rad(lon2 - lon1);
        let lat1r = to_rad(lat1);
        let lat2r = to_rad(lat2);
        let sin_dlat = (dlat / 2.0).sin();
        let sin_dlon = (dlon / 2.0).sin();
        let a = sin_dlat * sin_dlat + lat1r.cos() * lat2r.cos() * sin_dlon * sin_dlon;
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        let earth_km = 6371.0;
        Ok(earth_km * c)
    }
}
