use crate::{commands::command_handler::CommandHandler, error::ApplicationError};

#[derive(Clone, Debug)]
pub struct DeletePersonCommand {
    pub id: String,
}

impl CommandHandler<DeletePersonCommand, Self> for DeletePersonCommand {
    fn handle(&self) ->  Result<Option<Self>, ApplicationError> {
        println!("DeletePersonCommand: {:?}", self.id);
        Ok(None)
    }
}
