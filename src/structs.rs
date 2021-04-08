use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FunctionResponse<R> {
    r#return: R,
}

#[derive(Deserialize)]
pub struct Error {
    text: String,
}

impl Error {
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    error: Error,
}

impl ErrorResponse {
    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Response<R> {
    Success(FunctionResponse<R>),
    Error(ErrorResponse),
}

impl<R> FunctionResponse<R> {
    pub fn r#return(self) -> R {
        self.r#return
    }
}

#[derive(Deserialize, Serialize)]
pub struct Contract {
    remainder: String,
}

impl Contract {
    pub fn remainder(&self) -> &str {
        &self.remainder
    }
}

#[derive(Deserialize, Serialize)]
pub struct TripDetails {
    entry_place: String,
    cdt: String,
    amount: String,
    place: String,
    dt: String,
}

impl TripDetails {
    #[allow(dead_code)]
    pub fn entry_place(&self) -> &str {
        &self.entry_place
    }
    pub fn amount(&self) -> &str {
        &self.amount
    }
    pub fn dt(&self) -> &str {
        &self.dt
    }
}
