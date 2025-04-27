//! Build script for the distributed cache service
//! 
//! This script compiles the protobuf definitions into Rust code.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Protobuf compilation is temporarily disabled to fix build issues
    // We're using simplified type definitions instead

    // tonic_build::compile_protos("src/proto/cache.proto")?;
    // println!("cargo:rerun-if-changed=src/proto/cache.proto");

    Ok(())
}
