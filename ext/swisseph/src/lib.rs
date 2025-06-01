#[link(name = "swe", kind = "static")]
extern "C" {
    pub fn swe_version(ver: *mut ::std::os::raw::c_char);
    pub fn swe_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32, xx: *mut f64, serr: *mut ::std::os::raw::c_char) -> i32;
    pub fn swe_set_ephe_path(path: *const ::std::os::raw::c_char);
    pub fn swe_set_jpl_file(fname: *const ::std::os::raw::c_char);
    pub fn swe_set_topo(geolon: f64, geolat: f64, geoalt: f64);
    pub fn swe_close();
    pub fn swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64;
}

// Planet numbers
pub const SE_SUN: i32 = 0;
pub const SE_MOON: i32 = 1;
pub const SE_MERCURY: i32 = 2;
pub const SE_VENUS: i32 = 3;
pub const SE_MARS: i32 = 4;
pub const SE_JUPITER: i32 = 5;
pub const SE_SATURN: i32 = 6;
pub const SE_URANUS: i32 = 7;
pub const SE_NEPTUNE: i32 = 8;
pub const SE_PLUTO: i32 = 9;
pub const SE_MEAN_NODE: i32 = 10;
pub const SE_TRUE_NODE: i32 = 11;
pub const SE_MEAN_APOG: i32 = 12;
pub const SE_OSCU_APOG: i32 = 13;
pub const SE_EARTH: i32 = 14;

// Calculation flags
pub const SEFLG_SWIEPH: i32 = 2;
pub const SEFLG_JPLEPH: i32 = 4;
pub const SEFLG_MOSEPH: i32 = 8;
pub const SEFLG_HELCTR: i32 = 0x0008;
pub const SEFLG_TRUEPOS: i32 = 0x0010;
pub const SEFLG_J2000: i32 = 0x0020;
pub const SEFLG_NONUT: i32 = 0x0040;
pub const SEFLG_SPEED3: i32 = 0x0080;
pub const SEFLG_SPEED: i32 = 0x0100;
pub const SEFLG_NOGDEFL: i32 = 0x0200;
pub const SEFLG_NOABERR: i32 = 0x0400;
pub const SEFLG_AST_OFFSET: i32 = 0x0800;
pub const SEFLG_EQUATORIAL: i32 = 0x1000;
pub const SEFLG_XYZ: i32 = 0x2000;
pub const SEFLG_RADIANS: i32 = 0x4000;
pub const SEFLG_BARYCTR: i32 = 0x8000;
pub const SEFLG_TOPOCTR: i32 = 0x10000;
pub const SEFLG_SIDEREAL: i32 = 0x20000;
pub const SEFLG_ICRS: i32 = 0x40000;
pub const SEFLG_DPSIDEPS_1980: i32 = 0x80000;
pub const SEFLG_JPLHOR: i32 = 0x100000;
pub const SEFLG_JPLHOR_APPROX: i32 = 0x200000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Planet {
    Sun = SE_SUN as isize,
    Moon = SE_MOON as isize,
    Mercury = SE_MERCURY as isize,
    Venus = SE_VENUS as isize,
    Mars = SE_MARS as isize,
    Jupiter = SE_JUPITER as isize,
    Saturn = SE_SATURN as isize,
    Uranus = SE_URANUS as isize,
    Neptune = SE_NEPTUNE as isize,
    Pluto = SE_PLUTO as isize,
    MeanNode = SE_MEAN_NODE as isize,
    TrueNode = SE_TRUE_NODE as isize,
    MeanApogee = SE_MEAN_APOG as isize,
    OscuApogee = SE_OSCU_APOG as isize,
    Earth = SE_EARTH as isize,
}

#[derive(Debug, Clone, Copy)]
pub struct Flags(pub i32);

impl Default for Flags {
    fn default() -> Self {
        Flags(SEFLG_SWIEPH | SEFLG_SPEED)
    }
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_heliocentric(mut self) -> Self {
        self.0 |= SEFLG_HELCTR;
        self
    }

    pub fn with_barycentric(mut self) -> Self {
        self.0 |= SEFLG_BARYCTR;
        self
    }

    pub fn with_topocentric(mut self) -> Self {
        self.0 |= SEFLG_TOPOCTR;
        self
    }

    pub fn with_sidereal(mut self) -> Self {
        self.0 |= SEFLG_SIDEREAL;
        self
    }

    pub fn with_equatorial(mut self) -> Self {
        self.0 |= SEFLG_EQUATORIAL;
        self
    }

    pub fn with_xyz(mut self) -> Self {
        self.0 |= SEFLG_XYZ;
        self
    }

    pub fn with_radians(mut self) -> Self {
        self.0 |= SEFLG_RADIANS;
        self
    }
}

#[derive(Debug, Clone)]
pub struct EphePath(pub String);

impl From<&str> for EphePath {
    fn from(path: &str) -> Self {
        EphePath(path.to_string())
    }
}

pub struct Swisseph {
    initialized: bool,
}

impl Swisseph {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn set_ephe_path(&mut self, path: EphePath) {
        let path = std::ffi::CString::new(path.0).unwrap();
        unsafe {
            swe_set_ephe_path(path.as_ptr());
        }
        self.initialized = true;
    }

    pub fn set_jpl_file(&mut self, fname: &str) {
        let fname = std::ffi::CString::new(fname).unwrap();
        unsafe {
            swe_set_jpl_file(fname.as_ptr());
        }
    }

    pub fn set_topo(&mut self, geolon: f64, geolat: f64, geoalt: f64) {
        unsafe {
            swe_set_topo(geolon, geolat, geoalt);
        }
    }

    pub fn julday(&self, year: i32, month: i32, day: i32, hour: f64, gregflag: bool) -> f64 {
        unsafe {
            swe_julday(year, month, day, hour, gregflag as i32)
        }
    }

    pub fn calc_ut(&self, tjd_ut: f64, planet: Planet, flags: Flags) -> Result<[f64; 6], String> {
        let mut xx = [0.0f64; 6];
        let mut serr = [0i8; 256];
        
        let ret = unsafe {
            swe_calc_ut(
                tjd_ut,
                planet as i32,
                flags.0,
                xx.as_mut_ptr(),
                serr.as_mut_ptr()
            )
        };

        if ret < 0 {
            let error = unsafe {
                std::ffi::CStr::from_ptr(serr.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            Err(error)
        } else {
            Ok(xx)
        }
    }
}

impl Drop for Swisseph {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                swe_close();
            }
        }
    }
}

pub fn get_version() -> String {
    let mut buf = [0i8; 256];
    unsafe {
        swe_version(buf.as_mut_ptr());
        let cstr = std::ffi::CStr::from_ptr(buf.as_ptr());
        cstr.to_string_lossy().into_owned()
    }
} 