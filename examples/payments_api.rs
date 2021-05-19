use square_rs::client::SquareClient;
use square_rs::error::PaymentBuildError;
use square_rs::money::Currency;
use square_rs::payment::PaymentBuilder;

use std::env;
use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logging
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    let port = "8080";
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(process_payment)
            .data(AppState::init())
            // Serve the static files, to make testing easier.
            .service(actix_files::Files::new("/", "examples/static/").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

struct AppState {
    client: SquareClient,
}

impl AppState {
    fn init() -> Self {
        let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN to be set");
        let client = SquareClient::new(&access_token);

        Self { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentForm {
    nonce: String,
    idempotency_key: String,
    location_id: String,
    amount: String,
}

impl PaymentForm {
    fn get_price(&self) -> i64 {
        self.amount
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0)
    }
}

#[post("/process-payment")]
async fn process_payment(
    state: web::Data<AppState>,
    form: web::Json<PaymentForm>,
) -> impl Responder {
    // Get access to the square client
    let client = &state.client;

    // Serialize the square payment form from the request
    let payment_form = form.into_inner();

    // The total amount is the number of hours multiplied by
    //   the cost per hour. Multipled by 100 since the value
    //   is given in the lowest denomination so pence.
    let amount = payment_form.get_price() * 100;

    // Buld a payment using the infomation from the form
    let payment = match PaymentBuilder::new()
        .amount(amount as i64, Currency::GBP)
        .nonce(payment_form.nonce)
        .idempotency_key(payment_form.idempotency_key)
        .build()
        .await
    {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    // Create the payment and check the response
    match client.create_payment(payment).await {
        Ok(r) => HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .json(r),
        Err(_) => HttpResponse::BadRequest().json(PaymentBuildError),
    }
}
