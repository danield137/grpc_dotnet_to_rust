use std::path::PathBuf;
use tonic_build;

fn main() {
    let out_dir = PathBuf::from("src/generated"); // Custom output directory
    std::fs::create_dir_all(&out_dir).unwrap(); // Ensure the directory exists

    tonic_build::configure()
        .out_dir(out_dir) // Set output directory
        .build_server(true) // Ensure server-side code is generated
        .build_client(true) // Ensure client-side code is generated (optional)
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();
}
