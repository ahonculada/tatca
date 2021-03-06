use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;

/// This function deletes a to do item's status.
///
/// # Arguments
/// * to_do_item (web::Json<ToDoItem>): This serializes the JSON body
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);
    return HttpResponse::Ok().json(return_state())
}
