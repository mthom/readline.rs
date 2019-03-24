fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/opt/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/lib");    
    println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu");    
    println!("cargo:rustc-link-lib=readline");
}
