use backend::AsareApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8008;

    let app = AsareApp::new(port);
    app.run().await
}
