use crate::{commands::command_handler::CommandHandler, error::ApplicationError};

#[derive(Clone, Debug)]
pub struct AddPersonCommand {
    pub name: String,

    pub age: u16,
}


impl CommandHandler<AddPersonCommand, String> for AddPersonCommand {
    fn handle(&self) -> Result<Option<String>, ApplicationError> {
        println!("AddPersonCommand: {:?}", self.name);
        // call repository to add person
        Ok(Some("new_id".to_string()))
    }
}
