use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;

/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (String): message to be sent back to the user
pub async fn create(req: HttpRequest) -> String {
    println!("reading from state.json");
    // load the date from the state JSON file
    let state: Map<String, Value> = read_file(String::from("./state.json"));

    println!("creating title");
    // get the tile from the http request
    let title: String = req.match_info().get("title")
        .unwrap().to_string();
    let title_reference: String = title.clone();

    println!("creating item");
    // create the to do item
    let item = to_do::to_do_factory(&String::from("pending"), title)
        .expect("create ");

    println!("processing item");
    // add the to do item from the state.json
    process_input(item, "create".to_string(), &state);

    println!("processed item");
    // return a message to the viewer
    return format!("{} created", title_reference)
}
