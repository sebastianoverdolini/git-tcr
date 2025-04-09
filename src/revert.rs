pub type Revert = fn() -> RevertCommand;

pub fn revert_command() -> RevertCommand {
    "(git clean -fdq . && git reset --hard)".to_string()
}

pub type RevertCommand = String;

#[cfg(test)]
mod revert_command_test
{
    use crate::revert::revert_command;

    #[test]
    fn cmd()
    {
        let cmd = revert_command();

        assert_eq!(cmd, "(git clean -fdq . && git reset --hard)");
    }
}
