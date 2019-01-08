fn main() {
    println!("cargo:rustc-link-search={}", "/usr/local/opt/ruby/lib");
    println!("cargo:rustc-link-lib=dylib={}", "ruby");
}