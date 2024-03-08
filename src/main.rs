fn main()
{
    println!("Hello, world!");
}

fn tcr() -> &'static str
{
    return "Hello, world!"
}

#[cfg(test)]
mod tests
{
    use crate::tcr;

    #[test]
    fn it_works()
    {
        assert_eq!(tcr(), "Hello, world!");
    }
}
