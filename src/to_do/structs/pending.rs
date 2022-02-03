use super::base::Base;

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
