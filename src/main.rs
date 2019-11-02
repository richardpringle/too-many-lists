extern crate lib;

pub fn main() {
    let mut list = lib::fourth::List::new();

    list.push_front(1);
    list.pop_front();
}
