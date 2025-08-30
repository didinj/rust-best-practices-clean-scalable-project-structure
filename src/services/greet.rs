use crate::utils;

pub fn greet(name: &str) {
    utils::util::print_message(&format!("Hello, {}!", name));
}
