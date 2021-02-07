use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use serde::{Deserialize, Serialize};
use actix_web::web::Path;

#[derive(Serialize, Deserialize)]
struct AppState {
    todo_list: Mutex<HashMap<u32, Todo>>,
    count: Mutex<u32>,
}

#[derive(Serialize, PartialEq, Debug, Deserialize, Clone, Copy)]
enum Status {
    ACTIVE,
    DONE,
}

#[derive(Serialize, PartialEq, Debug, Deserialize, Clone)]
struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    text: String,
    status: Status,
}

async fn list_todo(data: web::Data<AppState>) -> HttpResponse {
    return HttpResponse::Ok().json(data.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>());
}

async fn create_todo(data: web::Data<AppState>, todo: web::Json<Todo>) -> HttpResponse {
    let mut count = data.count.lock().unwrap();

    data.todo_list.lock().unwrap()
        .insert(*count, Todo {
            id: Option::Some(*count),
            text: todo.text.clone(),
            status: todo.status,
        });

    *count += 1;

    return HttpResponse::Ok().json(data.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>());
}

async fn update_todo(data: web::Data<AppState>, id: Path<u32>, todo: web::Json<Todo>) -> HttpResponse {
    data.todo_list.lock().unwrap()
        .insert(id.count_ones(), Todo {
            id: Option::Some(id.count_ones()),
            text: todo.text.clone(),
            status: todo.status,
        });

    return HttpResponse::Ok().json(data.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>());
}

async fn delete_todo(data: web::Data<AppState>, id: Path<u32>) -> HttpResponse {
    data.todo_list.lock().unwrap()
        .remove(&id);

    return HttpResponse::Ok().json(data.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>());
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("8080"),
    };

    println!("Server started at {}", port);

    let store: HashMap<u32, Todo> = HashMap::new();
    let count: u32 = 1;

    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(store),
        count: Mutex::new(count),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .app_data(app_state.clone())
            .service(web::scope("/v1")
                .route("/todo-list", web::get().to(list_todo))
                .route("/todo-list", web::post().to(create_todo))
                .route("/todo-list/{id}", web::patch().to(update_todo))
                .route("/todo-list/{id}", web::delete().to(delete_todo))
            )
    })
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    use actix_web::{App, test::{self, TestRequest}, web};

    use super::*;

    #[actix_rt::test]
    async fn test_get() {
        let mut store: HashMap<u32, Todo> = HashMap::new();
        store.insert(1, Todo { id: Option::Some(1), text: String::from("Eat Pizza"), status: Status::ACTIVE });
        let count: u32 = 1;

        let app_state = web::Data::new(AppState {
            todo_list: Mutex::new(store),
            count: Mutex::new(count),
        });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(web::scope("/v1")
                    .route("/todo-list", web::get().to(list_todo))
                )
        ).await;

        let req = TestRequest::get().uri("/v1/todo-list").to_request();
        let result: Vec<Todo> = test::read_response_json(&mut app, req).await;

        assert_eq!(app_state.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>()[0], result[0]);
    }

    #[actix_rt::test]
    async fn test_post() {
        let store: HashMap<u32, Todo> = HashMap::new();
        let count: u32 = 1;

        let app_state = web::Data::new(AppState {
            todo_list: Mutex::new(store),
            count: Mutex::new(count),
        });

        let data = Todo {
            id: None,
            text: String::from("Sleep"),
            status: Status::ACTIVE,
        };

        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(web::scope("/v1")
                    .route("/todo-list", web::post().to(create_todo))
                )
        ).await;

        let req = TestRequest::post().uri("/v1/todo-list").set_json(&data).to_request();

        let result: Vec<Todo> = test::read_response_json(&mut app, req).await;

        assert_eq!(data.text, result[0].text);
        assert_eq!(data.status, result[0].status);
        assert_eq!(data.text, app_state.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>()[0].text);
        assert_eq!(data.status, app_state.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>()[0].status);
    }

    #[actix_rt::test]
    async fn test_put() {
        let mut store: HashMap<u32, Todo> = HashMap::new();
        store.insert(1, Todo { id: Option::Some(1), text: String::from("Eat Pizza"), status: Status::ACTIVE });
        let count: u32 = 1;

        let app_state = web::Data::new(AppState {
            todo_list: Mutex::new(store),
            count: Mutex::new(count),
        });

        let data = Todo {
            id: Option::Some(1),
            text: String::from("Sleep"),
            status: Status::DONE,
        };

        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(web::scope("/v1")
                    .route("/todo-list", web::put().to(update_todo))
                )
        ).await;

        let req = TestRequest::put().uri("/v1/todo-list").set_json(&data).to_request();

        let result: Vec<Todo> = test::read_response_json(&mut app, req).await;

        assert_eq!(data, result[0]);
        assert_eq!(data, app_state.todo_list.lock().unwrap().values().cloned().collect::<Vec<Todo>>()[0]);
    }
}