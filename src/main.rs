use ntex::web::{self, App, Error, HttpResponse};

#[derive(serde::Serialize)]
pub struct User {
    id: u16,
    age: u16,
    first_name: String,
    last_name: String,
    framework: String,
}

/// This handler manually load request payload and parse json-rust
async fn get_users() -> Result<HttpResponse, Error> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            id: index,
            age: 25,
            first_name: format!("First_Name{}", index),
            last_name: format!("Last_Name{}", index),
            framework: "Rust (ntex)".to_owned(),
        })
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(&users))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let _logical_cpus = num_cpus::get();
    let _physical_cpus = num_cpus::get_physical();
    let address = "127.0.0.1:8080";
    println!("Running at {address}");

    web::server(|| App::new().service((web::resource("/users").route(web::get().to(get_users)),)))
        .bind(address)?
        .workers(_logical_cpus)
        .run()
        .await
}
