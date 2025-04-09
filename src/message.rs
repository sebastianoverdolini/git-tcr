pub fn message() -> String
{
    "WIP".to_string()
}

#[cfg(test)]
mod message_test
{
    use crate::message::message;

    #[test]
    fn msg()
    {
        let msg = message();

        assert_eq!(msg, "WIP");
    }
}
