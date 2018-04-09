use std::process::Command;

fn main() {
    Command::new("make").current_dir("../graph/nauty-wrapper").output().expect("Failed to build");
    println!("cargo:rustc-link-search=../graph/nauty-wrapper")
}
