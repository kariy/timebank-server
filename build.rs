const SERIAL_DESERIAL_ATTR: &str = "#[derive(serde::Serialize, serde::Deserialize)]";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(".", SERIAL_DESERIAL_ATTR)
        .compile(
            &["proto/service-request.proto", "proto/auth.proto"],
            &["proto"],
        )?;

    Ok(())
}
