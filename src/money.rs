/*!
Representation of Money for the crate.

We follow the same convention as the [Square API](https://developer.squareup.com),
that is [Money](Money) consists of an amount and a currency in a valid currency code.
*/
use serde::{Deserialize, Serialize};

/// Representation of Money for the crate.
/// The amount is given in the lowest possible denomination.
/// So for GBP the ammount is in pence.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

/// The Currency code corresponding to the ammount of Money.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    GBP,
}
