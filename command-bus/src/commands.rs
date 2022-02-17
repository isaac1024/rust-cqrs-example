use std::any::TypeId;
use crate::bus::{BusError, Command, CommandBusDispatch, CommandHandler, CommandResult, MyCommandBus};
use crate::{command_bus_dispatch};

pub struct MyCommand{
    pub string_data: String,
    pub integer_data: i32
}

impl Command for MyCommand {
}

#[derive(Default)]
pub struct MyCommandHandler {}

impl CommandHandler<MyCommand> for MyCommandHandler {
    fn dispatch(&self, command: MyCommand) -> CommandResult {
        println!("String: {}, Integer: {}", command.string_data, command.integer_data);

        Ok(())
    }
}

pub struct MySecondCommand{
    pub bool_data: bool,
    pub float_data: f32
}

impl Command for MySecondCommand {
}

#[derive(Default)]
pub struct MySecondCommandHandler {}

impl CommandHandler<MySecondCommand> for MySecondCommandHandler {
    fn dispatch(&self, command: MySecondCommand) -> CommandResult {
        println!("Boolean: {}, Float: {}", command.bool_data, command.float_data);

        Ok(())
    }
}

command_bus_dispatch!(MyCommandBus, MyCommandHandler, MyCommand);
command_bus_dispatch!(MyCommandBus, MySecondCommandHandler, MySecondCommand);

#[derive(Debug)]
pub struct FailedCommand{
    pub data: String
}

impl Command for FailedCommand {
}
