mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> =
        to_do_factory(
            String::from("pending"), String::from("make")
        );

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!("It's an item called {}.", item.super_struct.title),
        ItemTypes::Done(item) => println!("It's an item called {}.", item.super_struct.title)
    }
}
