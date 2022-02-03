use serde_json::Map;
use serde_json::value::Value;

/// Trait for getting to do items.
pub trait Get {
    /// Gets a to do item.
    ///
    /// # Arguments
    /// * title (&String): the title of the to do item being fetched
    ///
    /// # Returns
    /// None
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}", result);
            },
            None => println!("item: {} was not found", title)
        }
    }
}
