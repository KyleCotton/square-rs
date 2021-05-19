/*!
Payment functionality of the [Square API](https://developer.squareup.com).
*/

use crate::client::SquareClient;
use crate::endpoint::SquareEndpoint;
use crate::error::PaymentBuildError;
use crate::error::SquareError;
use crate::money::{Currency, Money};
use crate::response::SquareResponse;

use serde::{Deserialize, Serialize};

impl SquareClient {
    /// Create a payment with the given [Payment](Payment) to the Square API
    /// and get the response back
    ///
    /// # Arguments
    /// * `payment` - A [Payment](Payment) created from the [PaymentBuilder](PaymentBuilder)
    pub async fn create_payment(&self, payment: Payment) -> Result<SquareResponse, SquareError> {
        self.request(SquareEndpoint::Payments, &payment).await
    }
}

/// The representation of a payment to the square API
/// containing a minimal set of fields for a payment
/// to be successfully processed.
#[derive(Serialize, Debug, Deserialize)]
pub struct Payment {
    #[serde(rename(serialize = "source_id"))]
    nonce: String,
    idempotency_key: String,
    amount_money: Money,
}

/// The [PaymentBuilder](PaymentBuilder)
pub struct PaymentBuilder {
    nonce: Option<String>,
    idempotency_key: Option<String>,
    amount_money: Option<Money>,
}

impl Default for PaymentBuilder {
    fn default() -> Self {
        Self {
            nonce: None,
            idempotency_key: None,
            amount_money: None,
        }
    }
}

impl PaymentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn nonce(mut self, nonce: String) -> Self {
        self.nonce = Some(nonce);

        self
    }

    pub fn idempotency_key(mut self, idempotency_key: String) -> Self {
        self.idempotency_key = Some(idempotency_key);

        self
    }

    pub fn amount(mut self, amount: i64, currency: Currency) -> Self {
        self.amount_money = Some(Money { amount, currency });

        self
    }

    pub async fn build(&self) -> Result<Payment, PaymentBuildError> {
        let nonce = match &self.nonce {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        let idempotency_key = match &self.idempotency_key {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        let amount_money = match &self.amount_money {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        Ok(Payment {
            nonce,
            idempotency_key,
            amount_money,
        })
    }
}
