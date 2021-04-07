use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Response<R> {
    r#return: R,
}

impl<R> Response<R> {
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
