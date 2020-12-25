pub(crate) trait NumberChecker<T> {
    fn is_number(&self, t: T) -> bool;
}
