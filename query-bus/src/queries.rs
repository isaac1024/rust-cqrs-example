use std::any::TypeId;
use crate::bus::{BusError, Query, QueryBusDispatch, QueryHandler, MyQueryBus, QueryResponse, QueryResult};
use crate::query_bus_ask;

pub struct MyQuery {
    pub string_data: String,
    pub integer_data: i32
}

impl Query for MyQuery {
}

pub struct MyQueryResponse {
    pub string_data: String,
    pub integer_data: i32
}

impl QueryResponse for MyQueryResponse {
}

#[derive(Default)]
pub struct MyQueryHandler {}

impl QueryHandler<MyQuery, MyQueryResponse> for MyQueryHandler {
    fn ask(&self, query: MyQuery) -> QueryResult<MyQueryResponse> {

        Ok(MyQueryResponse{string_data: query.string_data, integer_data: query.integer_data })
    }
}

pub struct MySecondQuery {
    pub bool_data: bool,
    pub float_data: f32
}

impl Query for MySecondQuery {
}

pub struct MySecondQueryResponse {
    pub bool_data: bool,
    pub float_data: f32
}

impl QueryResponse for MySecondQueryResponse {
}


#[derive(Default)]
pub struct MySecondQueryHandler {}

impl QueryHandler<MySecondQuery, MySecondQueryResponse> for MySecondQueryHandler {
    fn ask(&self, query: MySecondQuery) -> QueryResult<MySecondQueryResponse> {
        Ok(MySecondQueryResponse{ bool_data: query.bool_data, float_data: query.float_data })
    }
}

query_bus_ask!(MyQueryBus, MyQueryHandler, MyQuery, MyQueryResponse);
query_bus_ask!(MyQueryBus, MySecondQueryHandler, MySecondQuery, MySecondQueryResponse);
