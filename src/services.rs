pub mod account;
pub mod auth;
pub mod service_request;

pub mod error_messages {
    pub const INVALID_PAYLOAD: &str = "INVALID PAYLOAD";
    pub const UNKNOWN: &str = "AN ERROR HAS OCCURED";
    pub const ALREADY_EXISTS: &str = "ITEM ALREADY EXISTS";
    pub const MISSING_ARGUMENT: &str = "EXPECTED ARGUMENT MISSING";
    pub const TOO_MANY_REQUESTS: &str = "TOO MANY REQUESTS";
}

pub mod util {
    use postgrest::Postgrest;
    use reqwest::{header::HeaderMap, IntoUrl, RequestBuilder};

    pub fn create_postgrest_client() -> std::result::Result<Postgrest, Box<dyn std::error::Error>> {
        Ok(Postgrest::new(dotenv::var("SUPABASE_ENDPOINT").unwrap())
            .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap()))
    }

    pub struct HTTP {}

    impl HTTP {
        fn new<U: IntoUrl>(method: reqwest::Method, url: U) -> RequestBuilder {
            let supabase_key = dotenv::var("SUPABASE_API_KEY").unwrap();

            let mut headers = HeaderMap::new();
            headers.insert("apiKey", supabase_key.parse().unwrap());

            reqwest::Client::new()
                .request(method, url)
                .headers(headers)
                .bearer_auth(supabase_key)
        }

        pub fn get<U: IntoUrl>(url: U) -> RequestBuilder {
            Self::new(reqwest::Method::GET, url)
        }

        pub fn post<U: IntoUrl>(url: U) -> RequestBuilder {
            Self::new(reqwest::Method::POST, url)
        }
    }
}

pub type Result<T> = std::result::Result<T, tonic::Status>;

// pub fn get_request_payload<T>(request: Request<T>) ->
