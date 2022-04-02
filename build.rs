// build.rs


fn link_libraries() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "default")] {
            println!("cargo:rustc-flags=-l inkview -l hwconfig");
        }
    }
}

fn main() {
    link_libraries();
    println!("cargo:rerun-if-changed=build.rs");
}
