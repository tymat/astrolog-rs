fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib/astrolog");
    println!("cargo:rustc-link-lib=static=swe");
    println!("cargo:rerun-if-changed=/usr/local/lib/astrolog/libswe.a");
    println!("cargo:rerun-if-changed=/usr/local/include/astrolog/swephexp.h");
}
