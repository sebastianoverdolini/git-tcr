pub fn tcr(repository: &dyn Repository) -> bool {
    repository.stage();
    match repository.test() {
        true => { repository.commit(); true }
        false => { repository.revert(); false }
    }
}

pub trait Repository {
    fn stage(&self);
    fn revert(&self);
    fn commit(&self);
    fn test(&self) -> bool;
}

#[cfg(test)]
mod tcr_test {
    use std::cell::RefCell;
    use crate::tcr::Repository;
    use crate::tcr::tcr;

    struct FakeRepository {
        pub log: RefCell<Vec<String>>,
        pub test_result: bool,
        pub trailers: Vec<String>,
    }

    impl Repository for FakeRepository {
        fn stage(&self) {
            self.log.borrow_mut().push("stage".to_string());
        }
        fn revert(&self) {
            self.log.borrow_mut().push("revert".to_string());
        }
        fn commit(&self) {
            let mut msg = "commit".to_string();
            for t in &self.trailers {
                msg.push_str("\n");
                msg.push_str(t);
            }
            self.log.borrow_mut().push(msg);
        }
        fn test(&self) -> bool {
            self.log.borrow_mut().push("test".to_string());
            self.test_result
        }
    }

    #[test]
    fn green_test_and_commit() {
        let repository = FakeRepository { log: RefCell::new(vec![]), test_result: true, trailers: vec!["Issue: GDT-1234".to_string(), "Reviewed-by: Gennaro".to_string()] };
        let result = tcr(&repository);
        assert!(result);
        assert_eq!(repository.log.borrow().as_slice(), &["stage", "test", "commit\nIssue: GDT-1234\nReviewed-by: Gennaro"]);
    }

    #[test]
    fn red_test_and_revert() {
        let repository = FakeRepository { log: RefCell::new(vec![]), test_result: false, trailers: vec![] };
        let result = tcr(&repository);
        assert!(!result);
        assert_eq!(repository.log.borrow().as_slice(), &["stage", "test", "revert"]);
    }
}
