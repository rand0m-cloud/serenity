use serenity::builder::CreateApplicationCommand;

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("wonderful_command", "An amazing command")
}
