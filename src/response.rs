use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SquareResponse {
    Payment {
        id: String,
        status: String,
        order_id: String,
        receipt_number: String,
        receipt_url: String,
    },
    Order {
        random_name: String,
    },
}
