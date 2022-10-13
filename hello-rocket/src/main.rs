use std::collections::HashMap;

use rocket::{State, get, post, put, delete, routes, serde, catchers, catch};
use rocket::serde::{Serialize, Deserialize, json};
use rocket::serde::json::{json, Json, Value};
use rocket::tokio::sync::Mutex;

// 请求体的结构体
// model database 的结构体
// response 结构体

// #[derive(Serialize, Deserialize, Clone)]
// #[serde(crate = "rocket::serde")]
// struct Task {
//     id: usize,
//     name: String
// }

// #[get("/")]
// async fn hello() -> Option<Json<Task>> {
//     Some(Json(Task{
//         id: 1,
//         name: "tom".to_string()
//     }))
// }

// // restful

// #[get("/ex")]
// async fn get_exs() -> Value {
//    json!({"res": "ex list"})
// }

// #[get("/ex/<_id>")]
// async fn get_ex(_id: usize) -> Value {
//     json!({"res": "ex"})
// }

// #[post("/ex", format = "json", data = "<task>")]
// async fn post_ex(task: Json<Task>) -> Value {
//     let task = task.into_inner();
//     format!("{} {}", task.id, task.name);
//     json!({"res": format!("{} {}", task.id, task.name)})
// }

// #[put("/ex/<_id>")]
// async fn put_ex(_id: usize) -> Value {
//     json!({"res": "ex"})
// }

// #[delete("/ex/<_id>")]
// async fn delete_ex(_id: usize) -> Value {
//     json!({"res": "ex"})
// }

// #[catch(404)]
// async fn not_rust() -> Value {
//     json!("404")
// }

// #[catch(404)]
// async fn not_base() -> Value {
//     json!("base 404")
// }

// #[rocket::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     rocket::build()
//         // routes
//         .mount("/hello", routes![hello])
//         .mount("/base", routes![get_ex, post_ex, put_ex, delete_ex, get_exs])
//         .mount("/second", routes![get_ex, post_ex, put_ex, delete_ex, get_exs])
//         // catch
//         .register("/", catchers!(not_rust))
//         .register("/base", catchers!(not_base))
//         .launch().await?;
//     Ok(())
// }


// map -> mutex -> state
type PeopleItems = Mutex<HashMap<usize, People>>;
type Messages<'r> = &'r State<PeopleItems>;
 

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct People {
    id: usize,
    name: String
}

#[get("/people/<id>")]
async fn get_people(id: usize, message: Messages<'_>) -> Json<People> {
    let people_map = message.lock().await;
    if id ==0 {
        return Json(People { id: 0, name: "_".to_string() });
    }

    match  people_map.get(&id) {None=>Json(People{id:0,name:"_".to_string()}),
    Some(p) => Json(p.to_owned()),}

}

#[post("/people", format="json", data="<person>")]
async fn create_people(person: Json<People>, message: Messages<'_>) -> Value {
    let mut people_map = message.lock().await;
    let new_person = person.into_inner();
    if people_map.contains_key(&new_person.id) {
        json!({"res": "err"})
    } else {
        people_map.insert(new_person.id, new_person);
        json!({"res": "ok"})
    }
}

#[put("/people/<id>", format = "json", data = "<person>")]
async fn put_people(id: usize, person: Json<People>, message: Messages<'_>) -> Value { 
    let mut people_map = message.lock().await;
    let new_person = person.into_inner();
    if id != new_person.id {
        return json!({"res": "err"});
    }
    if people_map.contains_key(&id) {
        people_map.insert(new_person.id, new_person);
        json!({"res": "ok"})
    } else {
        json!({"res": "err"})
    }
}

#[delete("/people/<id>")]
async fn delete_people(id: usize, message: Messages<'_>) -> Value {
    let mut people_map = message.lock().await;
    if people_map.contains_key(&id) {
        people_map.remove(&id);
        json!({"res": "ok"})
    } else {
        json!({"res": "err"})
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .manage(PeopleItems::new(HashMap::new()))
        .mount("/rust", routes![get_people, create_people, put_people, delete_people])
        .launch()
        .await?;
    Ok(())
}