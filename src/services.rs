use postgrest::Postgrest;

pub mod account;
pub mod service_request;

pub fn create_postgrest_client() -> Result<Postgrest, Box<dyn std::error::Error>> {
    Ok(Postgrest::new(dotenv::var("SUPABASE_ENDPOINT").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap()))
}
