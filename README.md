# cfg lottery demo

Why we moved to `--cfg` flags instead of using `feature` for non-additive selections.

* Allows the top-level / binary crate to control the setting
* Works well for non-additive selections which otherwise needs compile_error!()
* Sometimes feasible not let transitive dependencies to be burdened with configuration
* Ofcourse as with everything there are tradeoffs!
* TBC: Blog entry coming up!

$ cargo tree -i lib_with_cfgs
```
lib_with_cfgs v0.1.0 (./crates/lib_with_cfgs)
├── naughty_lib v0.1.0 (./crates/naughty_lib)
│   └── wheelie_bin v0.1.0 (./crates/wheelie_bin)
└── okay_lib v0.1.0 (./crates/okay_lib)
    └── wheelie_bin v0.1.0 (./crates/wheelie_bin)
```

**Don't let the naughty lib get what it wants!**

$ cat crates/naughty_lib/build.rs 
```rust
fn main() {
    println!("cargo:rustc-cfg=lib_selection=\"what_naughty_lib_wanted\"");
}
```

$ env RUSTFLAGS cargo run
```
Sensible defaults!
```

env RUSTFLAGS='--cfg lib_selection="what_good_bin_wanted"' cargo run
```
Good bin got what it wanted!
```