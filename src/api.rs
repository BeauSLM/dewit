use rocket::fairing::AdHoc;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{Connection, Database};
use rocket::response::status::*; // TODO: prune

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("items")]
struct Items(rocket_db_pools::sqlx::SqlitePool);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct Item {
    #[serde(skip_deserializing)] // if you're adding a todo item it isn't done yet by definition
    done: bool,
    name: String,
    priority: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<String>,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")] // NOTE: do I really need to skip serializing at all????
    id: Option<i64>,
}

#[get("/items")]
async fn get_all_items(mut db: Connection<Items>) -> Result<Json<Vec<Item>>> {
    let items = sqlx::query_as("select * from items")
        .fetch_all(&mut *db)
        .await?;
    Ok(Json(items))
}

#[get("/items/<id>")]
async fn get_item(mut db: Connection<Items>, id: i64) -> Result<Json<Item>> {
    // should I include ID in this?
    let item = sqlx::query_as("select * from items where id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await?;
    Ok(Json(item))
}

#[delete("/items")]
async fn delete_all_items(mut db: Connection<Items>) -> Result<NoContent> {
    sqlx::query("delete from items")
        .execute(&mut *db)
        .await?;
    // for correctness
    Ok(NoContent)
}

#[delete("/items/<id>")]
async fn delete_item(mut db: Connection<Items>, id: i64) -> Result<Option<NoContent>> {
    let result = sqlx::query("delete from items where id = ?")
        .bind(id)
        .execute(&mut *db)
        .await?;
    // for correctness
    Ok((result.rows_affected() == 1).then(|| NoContent))
}

#[post("/items", data = "<item>")]
async fn add_item(mut db: Connection<Items>, item: Json<Item>) -> Result<Created<Json<Item>>> {
    let Item { name, priority, description, deadline, .. } = &item.0;
    let id = sqlx::query("
            insert into items
            (name, priority, description, deadline)
            values (?, ?, ?, ?);
        ")
        // TODO: don't call bind four times lmao
        .bind(name)
        .bind(priority)
        .bind(description)
        .bind(deadline)
        .execute(&mut *db)
        .await?
        .last_insert_rowid();

    let uri = uri!(crate::HOST, get_item(id)).to_string();
    Ok(Created::new(uri).body(item))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("api", |rocket| async {
        rocket.attach(Items::init()).mount(
            "/",
            routes![
                get_item,
                get_all_items,
                delete_item,
                delete_all_items,
                add_item
            ],
        )
    })
}
