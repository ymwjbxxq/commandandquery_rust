use commands::{
    command_handler::CommandProcessor,
    handlers::{add_person_handler::AddPersonCommand, delete_person_handler::DeletePersonCommand},
};
use error::ApplicationError;
use queries::{handlers::get_person_handler::GetPersonRequest, query_handler::QueryProcessor};

pub mod commands;
pub mod error;
pub mod queries;

fn main() -> Result<(), ApplicationError> {
    let add_person_command = AddPersonCommand {
        name: "Daniele".to_string(),
        age: 30,
    };
    let person_id = CommandProcessor::execute(Box::new(add_person_command))?;
    if let Some(id) = person_id {
        let get_person_request = GetPersonRequest { id: id.to_owned() };
        let person = QueryProcessor::execute(Box::new(get_person_request))?;
        if let Some(person) = person {
            let delete_person_command = DeletePersonCommand { id: person.id };
            CommandProcessor::execute(Box::new(delete_person_command))?;
        } else {
            return Err(ApplicationError::NotFound(id));
        }
    } else {
        return Err(ApplicationError::InternalError(String::from(
            "Cannot insert",
        )));
    }

    Ok(())
}
