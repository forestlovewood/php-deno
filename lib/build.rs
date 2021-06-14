use const_format::formatcp;
use cbindgen::{Builder, Config, Language, SortKey};
use std::{
    env, fs,
    path::PathBuf,
};

const LIBRARY: &'static str = "libdeno";
#[cfg(target_os = "linux")]
const LIBRARY_SUFFIX: &'static str = "so";
#[cfg(target_os = "macos")]
const LIBRARY_SUFFIX: &'static str = "dylib";
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("This platform is not supported");
const HEADER: &'static str = formatcp!(
    "#define FFI_LIB \"{}.{}\"\n#define FFI_SCOPE \"DENO\"",
    LIBRARY, LIBRARY_SUFFIX
);

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let mut crate_header_file = PathBuf::from(&crate_dir);
    crate_header_file.push(format!("target/{}/{}.h", profile, LIBRARY));

    let mut out_header_file = PathBuf::from(&out_dir);
    out_header_file.push(format!("{}.h", LIBRARY));

    Builder::new()
        .with_config(Config {
            sort_by: SortKey::Name,
            cpp_compat: false,
            ..Config::default()
        })
        .with_language(Language::C)
        .with_crate(&crate_dir)
        .with_no_includes()
        .with_documentation(false)
        .with_header(HEADER)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_header_file.as_path());

    fs::copy(out_header_file.as_path(), crate_header_file.as_path())
        .expect("Unable to copy the generated bindings");
}
