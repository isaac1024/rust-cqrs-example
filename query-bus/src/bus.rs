use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type QueryResult<R> = Result<R, BusError>;

pub enum BusError {
    DispatchError
}

impl Debug for BusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::DispatchError => write!(f, "Dispatch error"),
        }
    }
}

impl Display for BusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::DispatchError => write!(f, "Dispatch error"),
        }
    }
}

impl Error for BusError {}

pub trait Query {}
pub trait QueryResponse {}

pub trait QueryHandler<Q: Query, R: QueryResponse> {
    fn ask(&self, command: Q) -> QueryResult<R>;
}

pub trait QueryBus {
    fn add_query<Q, H, R>(&mut self, handler: H) -> Result<(), BusError>
        where Q: Query + 'static, H: QueryHandler<Q, R> + 'static, R: QueryResponse + 'static;
}

pub trait QueryBusDispatch<Q: Query, R: QueryResponse> {
    fn ask(&self, query: Q) -> QueryResult<R>;
}

#[derive(Default)]
pub struct MyQueryBus {
    pub queries: HashMap<TypeId, Box<dyn Any>>
}

impl QueryBus for MyQueryBus {
    fn add_query<Q, H, R>(&mut self, handler: H) -> Result<(), BusError>
        where Q: Query + 'static, H: QueryHandler<Q, R> + 'static, R: QueryResponse + 'static
    {
        self.queries.insert(TypeId::of::<Q>(), Box::new(handler));

        Ok(())
    }
}

#[macro_export]
macro_rules! query_bus_ask {
    ($bus: ident, $handler: ident, $query: ident, $response: ident) => {
        impl QueryBusDispatch<$query, $response> for $bus {
            fn ask(&self, query: $query) -> QueryResult<$response> {
                let handler: &$handler = self.queries
                    .get(&TypeId::of::<$query>())
                    .ok_or(BusError::DispatchError)?
                    .downcast_ref()
                    .ok_or(BusError::DispatchError)?;

                handler.ask(query)
            }
        }
    };
}

