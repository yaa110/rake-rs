pub (crate) trait NumberChecker<T> {
    fn is_number(&self, T) -> bool;
}
