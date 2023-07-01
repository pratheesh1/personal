use zero2prod::run;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    println!("Listening on port {}", port);

    Ok(())
}
