use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n = *command.data.get(0).unwrap_or(&1u8);
        if n < 2 || n > 16 { return; }
        context.code2d.aztec_size = n;
    }
}

pub fn new() -> Command {
    Command::new(
        "Aztec Set Size",
        vec![53, 67],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
