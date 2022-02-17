use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type CommandResult = Result<(), BusError>;

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

pub trait Command {}

pub trait CommandHandler<C: Command> {
    fn dispatch(&self, command: C) -> CommandResult;
}

pub trait CommandBus {
    fn add_command<C, H>(&mut self, handler: H) -> Result<(), BusError>
        where C: Command + 'static, H: CommandHandler<C> + 'static;
}

pub trait CommandBusDispatch<C: Command> {
    fn dispatch(&self, command: C) -> CommandResult;
}

#[derive(Default)]
pub struct MyCommandBus {
    pub commands: HashMap<TypeId, Box<dyn Any>>
}

impl CommandBus for MyCommandBus {
    fn add_command<C, H>(&mut self, handler: H) -> Result<(), BusError>
        where C: Command + 'static, H: CommandHandler<C> + 'static
    {
        self.commands.insert(TypeId::of::<C>(), Box::new(handler));

        Ok(())
    }
}

#[macro_export]
macro_rules! command_bus_dispatch {
    ($bus: ident, $handler: ident, $command: ident) => {
        impl CommandBusDispatch<$command> for $bus {
            fn dispatch(&self, command: $command) -> CommandResult {
                let handler: &$handler = self.commands
                    .get(&TypeId::of::<$command>())
                    .ok_or(BusError::DispatchError)?
                    .downcast_ref()
                    .ok_or(BusError::DispatchError)?;

                handler.dispatch(command)
            }
        }
    };
}

