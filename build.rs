// build.rs


#[cfg(feature = "default")]
fn links() {
    println!("cargo:rustc-flags=-l inkview -l hwconfig");
}

#[cfg(feature = "use_eframe")]
fn links() {

}

#[cfg(feature = "use_sdl2")]
fn links() {

}

fn main() {
    links();
    println!("cargo:rerun-if-changed=build.rs");
}
