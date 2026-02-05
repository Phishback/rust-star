fn main() {
    println!("cargo:rustc-link-search=/home/linuxbrew/.linuxbrew/opt/snappy/lib");
    println!("cargo:rustc-link-lib=snappy");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/linuxbrew/.linuxbrew/opt/snappy/lib");
}
