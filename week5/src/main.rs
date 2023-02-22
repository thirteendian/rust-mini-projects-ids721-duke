/* An actix Microservice that has four routes:
 A. / that returns hello world
 B. /fruit that returns a random fruit
 C. /health that returns a 200 status code
 F. /version that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use week5::random_fruit;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

//create a function that returns a random fruit
#[get("/fruit")]
async fn fruit() -> impl Responder {
    //print the random fruit
    println!("Random fruit: {}", random_fruit());
    HttpResponse::Ok().body(random_fruit())
}

//create a function that returns a 200 status code
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: 1.0.0");
    HttpResponse::Ok().body("1.0.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Service is running");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fruit)
            .service(health)
            .service(version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
