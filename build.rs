use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the home directory
    let home = env::var("HOME").expect("HOME environment variable not set");
    
    // Define the Swiss Ephemeris directories
    let swisseph_lib = format!("{}/.swisseph/lib", home);
    let swisseph_include = format!("{}/.swisseph/include", home);
    let swisseph_ephe = format!("{}/.swisseph/ephe", home);
    
    // Create directories if they don't exist
    for dir in [&swisseph_lib, &swisseph_include, &swisseph_ephe].iter() {
        fs::create_dir_all(dir).expect(&format!("Failed to create directory: {}", dir));
    }
    
    // Copy Swiss Ephemeris files
    let external_dir = Path::new("external/swisseph");
    
    // Check if external directory exists
    if !external_dir.exists() {
        println!("cargo:warning=Swiss Ephemeris external directory not found at {:?}", external_dir);
        println!("cargo:warning=Please ensure the Swiss Ephemeris files are in the correct location");
        return;
    }
    
    // Copy header files
    let mut header_files_found = false;
    for entry in fs::read_dir(external_dir).expect("Failed to read external/swisseph directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "h") {
            let filename = path.file_name().expect("Failed to get filename");
            fs::copy(&path, Path::new(&swisseph_include).join(filename))
                .expect(&format!("Failed to copy header file: {:?}", path));
            header_files_found = true;
        }
    }
    
    if !header_files_found {
        println!("cargo:warning=No header files (*.h) found in {:?}", external_dir);
    }
    
    // Copy library files
    let mut lib_files_found = false;
    for entry in fs::read_dir(external_dir).expect("Failed to read external/swisseph directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "a") {
            let filename = path.file_name().expect("Failed to get filename");
            fs::copy(&path, Path::new(&swisseph_lib).join(filename))
                .expect(&format!("Failed to copy library file: {:?}", path));
            lib_files_found = true;
        }
    }
    
    if !lib_files_found {
        println!("cargo:warning=No library files (*.a) found in {:?}", external_dir);
    }
    
    // Copy ephemeris files
    let ephe_dir = external_dir.join("ephe");
    if ephe_dir.exists() {
        let mut ephe_files_found = false;
        for entry in fs::read_dir(&ephe_dir).expect("Failed to read ephe directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().expect("Failed to get filename");
                fs::copy(&path, Path::new(&swisseph_ephe).join(filename))
                    .expect(&format!("Failed to copy ephemeris file: {:?}", path));
                ephe_files_found = true;
            }
        }
        
        if !ephe_files_found {
            println!("cargo:warning=No ephemeris files found in {:?}", ephe_dir);
        }
    } else {
        println!("cargo:warning=Ephemeris directory not found at {:?}", ephe_dir);
    }
    
    // Set environment variables for the build
    println!("cargo:rustc-env=SWISSEPH_LIB={}", swisseph_lib);
    println!("cargo:rustc-env=SWISSEPH_INCLUDE={}", swisseph_include);
    println!("cargo:rustc-env=SWISSEPH_EPHE={}", swisseph_ephe);
    
    // Link against the Swiss Ephemeris library
    println!("cargo:rustc-link-search=native={}", swisseph_lib);
    println!("cargo:rustc-link-lib=static=swe");
}
