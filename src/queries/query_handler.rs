use crate::error::ApplicationError;

pub trait IQuery {}

pub trait QueryHandler<TRequest, TResult>
where
    TRequest: IQuery,
{
    fn handle(&self) -> Result<Option<TResult>, ApplicationError>;
}

pub struct QueryProcessor<TRequest, TResult>
where
    TRequest: IQuery,
{
    command: TRequest,
    handler: Box<dyn QueryHandler<TRequest, TResult>>,
}

impl<TRequest, TResult> QueryProcessor<TRequest, TResult>
where
    TRequest: IQuery,
{
    pub fn execute(
        handler: Box<dyn QueryHandler<TRequest, TResult>>,
    ) -> Result<Option<TResult>, ApplicationError> {
        println!("Inside QueryProcessor:");
        return handler.handle();
    }
}
