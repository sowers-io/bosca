fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../dc/src/proto/cache.proto")?;
    Ok(())
}
