fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &["proto/helloworld.proto", "proto/authentication.proto"],
            &["proto", "proto"],
        )?;
    Ok(())
}
