use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Définir le chemin vers vcpkg
    let vcpkg_path = Path::new("vcpkg");

    // Cloner et installer vcpkg s'il n'est pas déjà installé
    if !vcpkg_path.exists() {
        println!("cargo:warning=Cloning vcpkg");
        Command::new("git")
            .args(&["clone", "https://github.com/microsoft/vcpkg.git"])
            .status()
            .expect("Failed to clone vcpkg");
        Command::new("sh")
            .args(&["./vcpkg/bootstrap-vcpkg.sh"])
            .status()
            .expect("Failed to bootstrap vcpkg");
    } else {
        println!("cargo:warning=vcpkg already exists");
    }

    // Installer libheif via vcpkg
    println!("cargo:warning=Installing libheif via vcpkg");
    Command::new("sh")
        .args(&["./vcpkg/vcpkg", "install", "libheif"])
        .status()
        .expect("Failed to install libheif via vcpkg");

    // Définir PKG_CONFIG_PATH pour pkg-config
    let pkg_config_path = vcpkg_path.join("installed").join("x64-osx").join("lib").join("pkgconfig");
    env::set_var("PKG_CONFIG_PATH", pkg_config_path.to_str().unwrap());

    // Configurer les flags d'environnement pour Rust
    println!("cargo:rustc-link-search=native=./vcpkg/installed/x64-osx/lib");
    println!("cargo:rustc-link-lib=dylib=heif");

    // Marquer le script pour rerun si build.rs change
    println!("cargo:rerun-if-changed=build.rs");
}