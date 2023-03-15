use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, context: &mut Context) {
        context.graphics.stored_graphics.retain(|k, _| {
            k.storage != ImageRefStorage::Ram
        });
    }
}

pub fn new() -> Command {
    Command::new(
        "Clears Download RAM Graphics Data",
        vec![81],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
