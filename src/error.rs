use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SquareError;

impl From<reqwest::Error> for SquareError {
    fn from(r: reqwest::Error) -> Self {
        eprintln!("Reqwest Failed: {:?}", r);
        SquareError
    }
}

impl From<reqwest::header::InvalidHeaderValue> for SquareError {
    fn from(r: reqwest::header::InvalidHeaderValue) -> Self {
        eprintln!("Reqwest Header Failed: {:?}", r);
        SquareError
    }
}

impl From<serde_json::Error> for SquareError {
    fn from(s: serde_json::Error) -> Self {
        eprintln!("Serde JSON Failed: {:?}", s);
        SquareError
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentError {
    code: PaymentErrorCode,
    category: PaymentErrorCategory,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum PaymentErrorCode {
    // ADDRESS_VERIFICATION_FAILURE,
    // CARDHOLDER_INSUFFICIENT_PERMISSIONS,
    ErrorCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentErrorCategory {
    PaymentErrorCat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentBuildError;
