fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "adventure_proto/user.proto",
                "adventure_proto/session.proto",
                "adventure_proto/chat.proto",
            ],
            &["adventure_proto"],
        )?;
    Ok(())
}
