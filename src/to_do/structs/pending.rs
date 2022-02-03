use super::base::Base;
use super::traits::create::Create;
use super::traits::edit::Edit;
use super::traits::get::Get;
use super::traits::delete::Delete;

pub struct Pending {
    /// This struct defines a to do item for a Pending to do item.
    ///
    /// # Attributes
    /// * super_struct (Base): Inherited struct for housing key attributess
    pub super_struct: Base
}

impl Pending {

   /// The constructor for the pending to do items.
   ///
   /// # Arguments
   /// * input_title (String): title of the to do item
   /// 
   /// # Return
   /// (Pending): the constructor Pending struct
    pub fn new(input_title: String) -> Pending {
        let input_status: String = String::from("pending");
        let base: Base = Base::new(input_title, input_status);
        return Pending{super_struct: base}
    }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
