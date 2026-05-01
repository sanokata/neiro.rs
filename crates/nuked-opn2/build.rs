fn main() {
    cc::Build::new()
        .file("c-src/ym3438.c")
        .include("c-src")
        .compile("nuked-opn2-c");
}
