mod to_do;
mod state;
mod processes;

use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use processes::process_input;

fn main() {
    // collects arguments passed into the program
    let args: Vec<String> = env::args().collect();

    // defines the commands from the environment
    let command: &String = &args[1];
    let title: &String = &args[2];

    // read the JSON file and print it using the debug notation
    let state: Map<String, Value> = read_file(
        String::from("./state.json")
    );

    // define outside the match scope to enable using status later on
    let status: String;

    // check to see if the title is alread there, setting status to pending if not
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = String::from("pending");
        }
    }
    // create a to do struct depending on status
    let item = to_do_factory(&status, title.to_string()).expect(&status);

    // update state based on struct and command
    process_input(item, command.to_string(), &state);
}
