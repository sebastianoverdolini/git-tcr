pub fn wip(_diff: &str) -> String {
    "WIP".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wip_returns_wip() {
        assert_eq!(wip("ignored"), "WIP");
    }
}

