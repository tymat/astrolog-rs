use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    
    // Compile the C source files
    let mut build = cc::Build::new();
    
    // Add all the source files
    build.file(manifest_path.join("sweph.c"))
         .file(manifest_path.join("swephlib.c"))
         .file(manifest_path.join("swemplan.c"))
         .file(manifest_path.join("swemmoon.c"))
         .file(manifest_path.join("swejpl.c"))
         .file(manifest_path.join("swedate.c"))
         .file(manifest_path.join("swecl.c"))
         .file(manifest_path.join("swehouse.c"))
         .file(manifest_path.join("swehel.c"));
    
    // Include the current directory for header files
    build.include(manifest_path);
    
    // Compile the library
    build.compile("swe");
    
    // Rebuild if any of the source files change
    println!("cargo:rerun-if-changed=sweph.c");
    println!("cargo:rerun-if-changed=swephlib.c");
    println!("cargo:rerun-if-changed=swemplan.c");
    println!("cargo:rerun-if-changed=swemmoon.c");
    println!("cargo:rerun-if-changed=swejpl.c");
    println!("cargo:rerun-if-changed=swedate.c");
    println!("cargo:rerun-if-changed=swecl.c");
    println!("cargo:rerun-if-changed=swehouse.c");
    println!("cargo:rerun-if-changed=swehel.c");
} 