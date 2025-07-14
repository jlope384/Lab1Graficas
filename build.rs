fn main() {
    // Tell cargo where to find raylib
    println!("cargo:rustc-link-search=native=C:/msys64/mingw64/lib");
    println!("cargo:rustc-link-lib=static=raylib");
    
    // Additional libraries that raylib depends on
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=winmm");
}
