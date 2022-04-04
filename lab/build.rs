fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .out_dir("src")
        .compile(&["proto/keeper.proto"], &["proto"])?;
    Ok(())
}
