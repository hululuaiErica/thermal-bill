use crate::{command::*};
use crate::context::Context;

#[derive(Clone)]
struct Handler;

impl CommandHandler for Handler {
    fn get_device_command(&self, _command: &Command, _context: &Context) -> Option<Vec<DeviceCommand>> {
        Some(vec![DeviceCommand::EndPrint])
    }
}

pub fn new() -> Command {
    Command::new(
      "End Print",
      vec![],
      CommandType::Control,
      DataType::Empty,
      Box::new(Handler {}),
    )
}


//Arc::new(Handler{}
