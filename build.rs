fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    
    println!("cargo:rustc-link-lib=readline");
}
