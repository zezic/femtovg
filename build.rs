
use std::env;
use std::fs::File;
use std::path::PathBuf;

use gl_generator::{Api, Fallbacks, Profile, Registry, GlobalGenerator};

fn main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let ft_probe = pkg_config::Config::new()
        .statik(true)
        .probe("freetype2");

    match ft_probe {
        Ok(_) => return,
        Err(_) => {
            println!("cargo:rustc-link-lib=static=freetype");
        }
    }
}
