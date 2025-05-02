fn main() {
    println!("cargo:rustc-link-search=/usr/lib/llvm-14/lib");
    println!("cargo:rustc-link-lib=omp");
}
