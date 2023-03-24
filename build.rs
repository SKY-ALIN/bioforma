fn main() {
    println!(
        "cargo:rustc-env=PROFILE={}",
        std::env::var("PROFILE").unwrap(),
    );
}
