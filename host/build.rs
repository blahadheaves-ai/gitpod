// build.rs
fn main() {
    // Compiles thermite_teardown.c with maximum optimization (-O3)
    cc::Build::new()
        .file("src/thermite_teardown.c")
        .opt_level(3)
        .warnings(true)
        .compile("thermite_teardown");

    println!("cargo:rerun-if-changed=src/thermite_teardown.c");
}