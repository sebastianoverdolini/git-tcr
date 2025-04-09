pub type Test = fn(test: String) -> TestCommand;

pub fn test_command(test: String) -> TestCommand {
    test
}

pub type TestCommand = String;

#[cfg(test)]
mod test_command_test
{
    use crate::test::test_command;

    #[test]
    fn cmd()
    {
        let cmd = test_command("pnpm test".to_string());

        assert_eq!(cmd, "pnpm test");
    }
}
