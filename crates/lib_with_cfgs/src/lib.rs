#[cfg(lib_selection = "what_naughty_lib_wanted")]
pub fn selection() {
    println!("Naughty lib won!");
}

#[cfg(lib_selection = "what_good_bin_wanted")]
pub fn selection() {
    println!("Good bin got what it wanted!");
}

#[cfg(lib_selection = "sensible_default")]
pub fn selection() {
    println!("Sensible defaults!");
}

#[cfg(all(not(lib_selection = "what_good_bin_wanted"), not(lib_selection = "what_naughty_lib_wanted"), not(lib_selection = "sensible_default")))]
pub fn selection() {
    println!("Nobody wanted anything");
}

