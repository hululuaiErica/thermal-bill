use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, _command: &Command, _context: &mut Context) {
        //TODO No support for this yet
        //context.code2d.symbol_storage = Some(Cod2D)
    }
}

pub fn new() -> Command {
    Command::new(
        "PDF417 Store the Code2D data",
        vec![53, 80],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
