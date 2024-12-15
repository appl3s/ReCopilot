use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <title>Rust Demo Web Server</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        max-width: 800px;
                        margin: 0 auto;
                        padding: 20px;
                        background-color: #f5f5f5;
                    }
                    .container {
                        background-color: white;
                        padding: 20px;
                        border-radius: 8px;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    }
                    h1 {
                        color: #333;
                        text-align: center;
                    }
                    .demo-section {
                        margin-top: 20px;
                        padding: 15px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                    }
                </style>
            </head>
            <body>
                <div class="container">
                    <h1>Welcome to Rust Demo Web Server!</h1>
                    <div class="demo-section">
                        <h2>About This Demo</h2>
                        <p>This is a simple web server built with Rust and Actix-web framework. It demonstrates:</p>
                        <ul>
                            <li>Basic web server setup with Actix-web</li>
                            <li>HTML and CSS styling</li>
                            <li>Responsive design principles</li>
                        </ul>
                    </div>
                </div>
            </body>
        </html>"#,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
