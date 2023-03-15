use crate::{command::*, context::*, graphics::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        if let Some(img) = Image::from_column_data(&command.data) {
            context.graphics.buffer_graphics = Some(img)
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "Store Print Buffer Graphics Table Format",
        vec![113],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
