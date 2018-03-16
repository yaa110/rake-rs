pub(crate) trait NumberChecker {
    fn is_number(&self) -> bool;
}

impl<'a> NumberChecker for &'a str {
    fn is_number(&self) -> bool {
        self.parse::<f64>().is_ok()
    }
}
