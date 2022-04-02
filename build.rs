// build.rs

#[cfg(feature = "use_eframe")]
fn links() {

}

#[cfg(not(feature = "use_eframe"))]
fn links() {
    println!("cargo:rustc-flags=-l inkview -l hwconfig");
}

fn main() {
    links();
    println!("cargo:rerun-if-changed=build.rs");
}
