mod consts;
use consts::PROTOS_OUTPUT_DIR;

// the build.rs is like this so we can see the codegen and read it easily
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir_all(PROTOS_OUTPUT_DIR);

    tonic_build::configure()
        .out_dir(PROTOS_OUTPUT_DIR)
        .compile_protos(&["proto/helloworld.proto"], &["./proto/"])?;

    Ok(())
}
