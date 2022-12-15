fn main() {
    #[cfg(not(lib_selection = "what_good_bin_wanted"))]
    println!("cargo:rustc-cfg=lib_selection=\"sensible_default\"");
}

