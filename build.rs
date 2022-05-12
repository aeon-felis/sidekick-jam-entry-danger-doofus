extern crate embed_resource;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build/windows/icon.ico");
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc");
    }
}
