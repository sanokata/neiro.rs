fn main() {
    if std::env::var("CARGO_FEATURE_C_REFERENCE").is_ok() {
        cc::Build::new()
            .file("c-src/ym3438.c")
            .include("c-src")
            .compile("nuked-opn2-c");
    }
}
