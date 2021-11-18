use crate::{error::ApplicationError, queries::query_handler::QueryHandler};

#[derive(Clone, Debug)]
pub struct GetPersonRequest {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: u16,
}

impl QueryHandler<GetPersonRequest, Person> for GetPersonRequest {
    fn handle(&self) -> Result<Option<Person>, ApplicationError> {
        println!("GetPersonQueryRequest: {:?}", self.id);

        // call person repository to get person
        let person = Person {
            id: self.id.clone(),
            name: "Daniele".to_string(),
            age: 20,
        };
        Ok(Some(person))
    }
}
