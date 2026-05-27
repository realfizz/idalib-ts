fn main() -> Result<(), Box<dyn std::error::Error>> {
    napi_build::setup();
    idalib_build::configure_linkage()?;
    Ok(())
}
