const SERIAL_DESERIAL_ATTR: &str = "#[derive(serde::Serialize, serde::Deserialize)]";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(".", SERIAL_DESERIAL_ATTR)
        .include_file("proto.rs")
        .compile(
            &[
                "proto/auth.proto",
                "proto/account.proto",
                "proto/collection/service-rating.proto",
                "proto/collection/service-request.proto",
                "proto/collection/service-request-bid.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
