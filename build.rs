use std::process::Command;
use std::fs;

// export REPO=$(pwd); git submodule --quiet foreach 'export NAME="third_party/${PWD##*/}"; git add --all -N :/; git --no-pager diff --src-prefix="a/${NAME}/" --dst-prefix="b/${NAME}/" --no-color >| "${REPO}/patches/${NAME#"third_party/"}-$(date +%s).patch"'

fn main() {
    println!("cargo:rerun-if-changed=patches");

    for path in fs::read_dir("patches").unwrap() {
        let output = Command::new("git").arg("apply").arg(path.unwrap().path().to_str().unwrap()).output().expect("failed to apply patch");
        if !output.status.success() {
            println!("cargo:warning={}", std::str::from_utf8(&output.stderr).unwrap());
        }
    }
}