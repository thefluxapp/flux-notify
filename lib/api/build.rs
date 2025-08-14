use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&["src/events.proto"], &["src"])
        .unwrap();

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("pushes_descriptor.bin"))
        .compile_protos(&["src/pushes.proto"], &["src"])
        .unwrap();
}
