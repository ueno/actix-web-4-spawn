use actix_web::{get, web, App, HttpServer, Responder};

async fn do_stuff_async() -> std::io::Result<()> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        println!("do_stuff_async");
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(greet))
        .bind("127.0.0.1:8080")?
        .run();
    let server_task = tokio::spawn(server);
    let worker_task = tokio::spawn(do_stuff_async());
    let _ = tokio::try_join!(server_task, worker_task)
        .expect("unable to join tasks");
    Ok(())
}
