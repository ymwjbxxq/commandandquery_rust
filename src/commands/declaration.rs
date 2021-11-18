use super::{command_handler::ICommnad, handlers::{add_person_handler::AddPersonCommand, delete_person_handler::DeletePersonCommand}};

impl ICommnad for AddPersonCommand {}

impl ICommnad for DeletePersonCommand {}
