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
use uuid::Uuid;

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
    source_id: String,
    idempotency_key: String,
    amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_token: Option<String>,
}

/// The [PaymentBuilder](PaymentBuilder)
pub struct PaymentBuilder {
    source_id: Option<String>,
    amount_money: Option<Money>,
    verification_token: Option<String>,
}

impl Default for PaymentBuilder {
    fn default() -> Self {
        Self {
            source_id: None,
            amount_money: None,
            verification_token: None,
        }
    }
}

impl PaymentBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn source_id(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);

        self
    }

    pub fn amount(mut self, amount: i64, currency: Currency) -> Self {
        self.amount_money = Some(Money { amount, currency });

        self
    }

    pub fn verification_token(mut self, token: String) -> Self {
        self.verification_token = Some(token);

        self
    }

    pub async fn build(&self) -> Result<Payment, PaymentBuildError> {
        let source_id = match &self.source_id {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        // The idempotency key just needs to be a random string
        // it is advised to use a v4 uuid by stripe
        let idempotency_key = Uuid::new_v4().to_string();

        let amount_money = match &self.amount_money {
            Some(n) => n.clone(),
            None => return Err(PaymentBuildError),
        };

        let verification_token = self.verification_token.clone();

        Ok(Payment {
            source_id,
            idempotency_key,
            amount_money,
            verification_token,
        })
    }
}
