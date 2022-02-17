use crate::bus::{CommandBus, CommandBusDispatch, MyCommandBus};
use crate::commands::{MyCommandHandler, MySecondCommandHandler};
use crate::commands::MyCommand;
use crate::commands::MySecondCommand;

mod bus;
mod commands;

fn main() {
    let mut bus = MyCommandBus::default();
    add_commands(&mut bus);

    println!();
    println!("===== Command bus =====");

    handler_command_1(&bus);
    handler_command_2(&bus);

    println!();
}

fn add_commands(bus: &mut impl CommandBus) {
    let handler1 = MyCommandHandler{};
    let handler2 = MySecondCommandHandler{};

    bus.add_command::<MyCommand, MyCommandHandler>(handler1)
        .unwrap_or_else(|error| println!("{}", error));
    bus.add_command::<MySecondCommand, MySecondCommandHandler>(handler2)
        .unwrap_or_else(|error| println!("{}", error));
}

fn handler_command_1(bus: &impl CommandBusDispatch<MyCommand>) {
    let command = MyCommand{string_data: "a string".to_string(), integer_data: 5};
    bus.dispatch(command).unwrap_or_else(|error| println!("{}", error));
}

fn handler_command_2(bus: &impl CommandBusDispatch<MySecondCommand>) {
    let command = MySecondCommand{bool_data: true, float_data: 3.14};
    bus.dispatch(command).unwrap_or_else(|error| println!("{}", error));
}
