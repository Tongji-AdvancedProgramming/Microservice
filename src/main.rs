use axum::{
    routing::{get, post},
    Router,
};

mod login_auth;

#[tokio::main]
async fn main() {
    login_auth::database::init_mysql_pool();

    let app = Router::new()
            .route("/", get(|| async { "This Service Do Not Offer An Frontend Interface." }))
            .route("/testcon", get(concur_test))
            .route("/loginauth",post(login_auth::login_auth));

    axum::Server::bind(&"0.0.0.0:8100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn concur_test() -> &'static str{
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    return "Ok";
} 