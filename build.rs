fn main() {
    println!("cargo:rerun-if-changed=polymur-ext");

    cc::Build::new()
        .file("polymur-ext/polymur-hash.c")
        .compile("polymur-hash");
}
