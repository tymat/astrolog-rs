#[link(name = "swe")]
extern "C" {
    pub fn swe_houses(
        tjd_ut: f64,
        geolat: f64,
        geolon: f64,
        hsys: i32,
        cusp: *mut f64,
        ascmc: *mut f64,
    ) -> i32;
} 