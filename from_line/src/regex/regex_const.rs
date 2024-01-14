use once_cell::sync::OnceCell;
use regex::Regex;

static ONLY_ZERO_REGEX: OnceCell<Regex> = OnceCell::new();

pub fn only_zero_regex() -> &'static Regex {
    ONLY_ZERO_REGEX.get_or_init(|| Regex::new(r"^0+$").unwrap())
}
