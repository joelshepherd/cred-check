use std::env::var;

#[tokio::main]
async fn main() {
    let port = var("PORT")
        .map(|x| x.parse().expect("Invalid port number supplied"))
        .unwrap_or(8080);

    println!("Server listening on port {}", &port);

    warp::serve(api::init().await)
        .run(([127, 0, 0, 1], port))
        .await;
}
