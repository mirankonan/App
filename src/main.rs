use std::{sync::Mutex};
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use actix_cors::Cors; 


#[derive(Serialize, Deserialize)]
struct Addlist {
    id: i32,
    title: String,
}

struct AppState {
    new_data: Mutex<Vec<Addlist>>
}

#[get("/read/{id}")]
async fn get_user(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder  {
    let new_data = data.new_data.lock().unwrap();
    for user in new_data.iter() {
        if user.id == *id {
            return HttpResponse::Ok().json(user);
        }
    }
    HttpResponse::NotFound().json("User not found")
}

#[post("/create")]

async fn create_user(info: web::Json<Addlist>, data: web::Data<AppState>) -> impl Responder {
    let mut new_data = data.new_data.lock().unwrap();
    let user_data = Addlist {
        id: info.id,
        title: String::from(&info.title),
    };
    new_data.push(user_data);
    HttpResponse::Ok().json("List added")
}

#[put("/update/{id}")] 

async fn update_user(id : web::Path<i32>, info: web::Json<Addlist>, data: web::Data<AppState>) -> impl Responder {
    let mut new_data = data.new_data.lock().unwrap();
    for user in new_data.iter_mut() {
        if user.id == *id {
            user.title = String::from(&info.title);
            return HttpResponse::Ok().json("List updated");
        }
    }
    HttpResponse::NotFound().json("User not found")
}

#[get("/user_data")]

async fn get_all_users(data: web::Data<AppState>) -> impl Responder {
    let new_data = data.new_data.lock().unwrap();
    HttpResponse::Ok().json(&*new_data)
}

#[delete("/delete/{id}")]

async fn delete_user(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let mut new_data = data.new_data.lock().unwrap();
    let pos = new_data.iter().position(|r| r.id == *id);
    match pos {
        Some(index) => {
            new_data.remove(index);
            HttpResponse::Ok().json("List deleted")
        }
        None => HttpResponse::NotFound().json("List not found")
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let new_data = web::Data::new(AppState {
        new_data: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
        )
        .app_data(new_data.clone())
        .service(get_all_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
    })
    .bind("localhost:8080")?
    .run()
    .await
}

//let mut client = Client::connect("host=")