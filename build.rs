fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &["proto/helloworld.proto", "proto/authentication.proto"],
            &["proto", "proto"],
        )?;

    // Generate the configuration schema
    config::write_schema_to_disk("config_schema.json")?;

    Ok(())
}
