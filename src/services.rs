pub mod account;
pub mod auth;
pub mod collection;

pub type Result<T> = std::result::Result<T, tonic::Status>;

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

    #[allow(dead_code)]
    #[derive(serde::Deserialize, Default, Debug)]
    pub struct DatabaseErrorMessage {
        message: String,
        code: String,
        details: String,
        hint: String,
    }

    #[allow(dead_code)]
    #[derive(serde::Deserialize, Default, Debug)]
    pub struct DatabaseErrorResponse {
        response_status: u16,
        error: DatabaseErrorMessage,
    }

    pub struct HTTP {}

    impl HTTP {
        fn new<U: IntoUrl>(method: reqwest::Method, url: U) -> RequestBuilder {
            let supabase_key = dotenv::var("SUPABASE_API_KEY").expect("MISSING SUPABASE API KEY!");

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

    pub mod helper {}

    pub mod miscellaneous {

        pub fn create_postgrest_client() -> super::Postgrest {
            let (endpoint, api_key) = (
                dotenv::var("SUPABASE_ENDPOINT").expect("MISSING SUPBASE POSTGREST ENDPOINT!"),
                dotenv::var("SUPABASE_API_KEY").expect("MISSING SUPABASE API KEY!"),
            );

            super::Postgrest::new(endpoint).insert_header("apikey", api_key)
        }
    }
}

// pub fn get_request_payload<T>(request: Request<T>) ->
