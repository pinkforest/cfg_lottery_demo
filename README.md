# cfg lottery demo

Why we moved to `--cfg` flags instead of using `feature` for non-additive selections.

* Allows the top-level / binary crate to control the setting
* Works well for non-additive selections which otherwise needs compile_error!()
* Sometimes feasible not let transitive dependencies to be burdened with configuration
* Ofcourse as with everything there are tradeoffs!
* TBC: Blog entry coming up!

## Setup

$ cargo tree -i lib_with_cfgs
```
lib_with_cfgs v0.1.0 (./crates/lib_with_cfgs)
├── naughty_lib v0.1.0 (./crates/naughty_lib)
│   └── wheelie_bin v0.1.0 (./crates/wheelie_bin)
└── okay_lib v0.1.0 (./crates/okay_lib)
    └── wheelie_bin v0.1.0 (./crates/wheelie_bin)
```

## Naughty lib

**Don't let the naughty lib get what it wants!**

$ cat crates/naughty_lib/build.rs 
```rust
fn main() {
    #[cfg(all(
        not(lib_selection = "what_good_bin_wanted"),
        not(lib_selection = "what_naughty_lib_wanted")
    ))]
    println!("cargo:rustc-cfg=lib_selection=\"sensible_default\"");
}
```

## Sensible defaults

**Set some sensible defaults..**

$ cat crates/lib_with_cfgs/build.rs 
```rust
fn main() {
    #[cfg(all(not(lib_selection = "what_good_bin_wanted"), not(lib_selection = "what_naughty_lib_wanted")))]
    println!("cargo:rustc-cfg=lib_selection=\"sensible_default\"");
}
```

$ env RUSTFLAGS cargo run
```
Sensible defaults!
```

## Various Ways

env RUSTFLAGS='--cfg lib_selection="what_good_bin_wanted"' cargo run
```
Good bin got what it wanted!
```

Also [.cargo/config \[build\] section.](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)

## Real World

- `curve25519_dalek_[bits|backend]`: https://github.com/dalek-cryptography/curve25519-dalek/
