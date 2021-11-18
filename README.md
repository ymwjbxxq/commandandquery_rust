# Command And Query

The idea is that we should divide an object's methods into two separated categories:

* Queries: Return a result and do not change the state.
* Commands: Change the state of a system.

A Commnad should not return a result but I have found useful when you need to add something and you need to return the ID for example.

The code example is simple:
```
fn main() -> Result<(), ApplicationError> {
    let add_person_command = AddPersonCommand {
        name: "Daniele".to_string(),
        age: 20,
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
        return Err(ApplicationError::InternalError(String::from("Cannot insert")));
    }

    Ok(())
}
```

* I Runng the first commnad
* Get the iphotethical ID back 
* Use the ID for the query
* If I find the person I run my second commnad

### COMMAND or QUERY ###
Each command extend the trait:
```
pub trait CommandHandler<TCommnad, TResult> where TCommnad: ICommnad {
    fn handle(&self) ->  Result<Option<TResult>, ApplicationError>;
}
```

And because each command must be of a type ICommnad  we need also to declare them:
```
impl ICommnad for AddPersonCommand {}
```

now we can implement the trait :
```
pub struct AddPersonCommand {
    pub name: String,
    pub age: u16,
}

impl CommandHandler<AddPersonCommand, String> for AddPersonCommand {
    fn handle(&self) ->  Result<Option<String>, ApplicationError> {
        println!("AddPersonCommand: {:?}", self.name);
        // call repository to add person
        Ok(Some("new_id".to_string()))
    }
}
```

and we could call the handler function in this way:
```
add_person_command.handle()?
```

But with the processor as wrapper we could run before or after some operation to all handlers.


### BUILD ###
```
cargo build
```

### RUN ###
```
cargo run
```
### RESULT ###
```
Inside CommandProcessor
AddPersonCommand: "Daniele"
Inside QueryProcessor:
GetPersonQueryRequest: "new_id"
Inside CommandProcessor
```
