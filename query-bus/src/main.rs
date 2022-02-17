use crate::bus::{MyQueryBus, QueryBus, QueryBusDispatch};
use crate::queries::{MyQuery, MyQueryHandler, MyQueryResponse, MySecondQuery, MySecondQueryHandler, MySecondQueryResponse};

mod bus;
mod queries;

fn main() {
    let mut bus = MyQueryBus::default();
    add_queries(&mut bus);

    println!();
    println!("===== Query Bus =====");

    handler_query_1(&bus);
    handler_query_2(&bus);

    println!();
}

fn add_queries(bus: &mut impl QueryBus) {
    let handler1 = MyQueryHandler{};
    let handler2 = MySecondQueryHandler{};

    bus.add_query::<MyQuery, MyQueryHandler, MyQueryResponse>(handler1)
        .unwrap_or_else(|error| println!("{}", error));
    bus.add_query::<MySecondQuery, MySecondQueryHandler, MySecondQueryResponse>(handler2)
        .unwrap_or_else(|error| println!("{}", error));
}

fn handler_query_1(bus: &impl QueryBusDispatch<MyQuery, MyQueryResponse>) {
    let query = MyQuery{string_data: "a string".to_string(), integer_data: 5};
    match bus.ask(query) {
        Ok(response) => println!("String: {}, Integer: {}", response.string_data, response.integer_data),
        Err(error) => println!("{}", error)
    }
}

fn handler_query_2(bus: &impl QueryBusDispatch<MySecondQuery, MySecondQueryResponse>) {
    let query = MySecondQuery{bool_data: true, float_data: 3.14};
    match bus.ask(query) {
        Ok(response) => println!("Boolean: {}, Float: {}", response.bool_data, response.float_data),
        Err(error) => println!("{}", error)
    }
}
