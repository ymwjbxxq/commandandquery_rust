use crate::error::ApplicationError;

pub trait ICommnad {}

pub trait CommandHandler<TCommnad, TResult>
where
    TCommnad: ICommnad,
{
    fn handle(&self) -> Result<Option<TResult>, ApplicationError>;
}

pub struct CommandProcessor<TCommnad, TResult>
where
    TCommnad: ICommnad,
{
    command: TCommnad,
    handler: Box<dyn CommandHandler<TCommnad, TResult>>,
}


impl<TCommnad, TResult> CommandProcessor<TCommnad, TResult>
where
    TCommnad: ICommnad,
{
    pub fn execute(handler: Box<dyn CommandHandler<TCommnad, TResult>>,
    ) -> Result<Option<TResult>, ApplicationError> {
        println!("Inside CommandProcessor");
        return handler.handle();
    }
}

