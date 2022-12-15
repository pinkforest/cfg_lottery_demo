fn main() {
    #[cfg(all(
        not(lib_selection = "what_good_bin_wanted"),
        not(lib_selection = "what_naughty_lib_wanted")
    ))]
    println!("cargo:rustc-cfg=lib_selection=\"sensible_default\"");
}
