pub mod account;
pub mod service_request;

pub mod error_messages {
    pub const INVALID_PAYLOAD: &str = "INVALID PAYLOAD";
    pub const UNKNOWN: &str = "AN ERROR HAS OCCURED";
    pub const ALREADY_EXISTS: &str = "ITEM ALREADY EXISTS";
    pub const MISSING_ARGUMENT: &str = "EXPECTED ARGUMENT MISSING";
}

pub mod util {

    use postgrest::Postgrest;

    pub fn create_postgrest_client() -> std::result::Result<Postgrest, Box<dyn std::error::Error>> {
        Ok(Postgrest::new(dotenv::var("SUPABASE_ENDPOINT").unwrap())
            .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap()))
    }
}

pub type Result<T> = std::result::Result<T, tonic::Status>;

// pub fn get_request_payload<T>(request: Request<T>) ->
