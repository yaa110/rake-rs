use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"-?\p{N}+[./٫,']?\p{N}*").unwrap();
}

pub(crate) trait NumberChecker {
    fn is_number(&self) -> bool;
}

impl<'a> NumberChecker for &'a str {
    fn is_number(&self) -> bool {
        NUM_RE.is_match(self)
    }
}
