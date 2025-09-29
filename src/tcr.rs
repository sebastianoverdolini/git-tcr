pub fn tcr(repository: &dyn Repository) {
    repository.stage();
    match repository.test() {
        true => repository.commit(),
        false => repository.revert(),
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
    }

    impl Repository for FakeRepository {
        fn stage(&self) {
            self.log.borrow_mut().push("stage".to_string());
        }
        fn revert(&self) {
            self.log.borrow_mut().push("revert".to_string());
        }
        fn commit(&self) {
            self.log.borrow_mut().push("commit".to_string());
        }
        fn test(&self) -> bool {
            self.log.borrow_mut().push("test".to_string());
            self.test_result
        }
    }

    #[test]
    fn green_scenario_stage_test_and_commit() {
        let repository = FakeRepository { log: RefCell::new(vec![]), test_result: true };
        tcr(&repository);
        assert_eq!(repository.log.borrow().as_slice(), &["stage", "test", "commit"]);
    }

    #[test]
    fn red_scenario_stage_test_and_revert() {
        let repository = FakeRepository { log: RefCell::new(vec![]), test_result: false };
        tcr(&repository);
        assert_eq!(repository.log.borrow().as_slice(), &["stage", "test", "revert"]);
    }
}
