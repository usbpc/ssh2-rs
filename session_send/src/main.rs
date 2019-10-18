use lazy_static::lazy_static;
use ssh2::Session;
use std::sync::Mutex;

lazy_static! {
    static ref TEST: Mutex<Session> = Mutex::new(Session::new());
}

fn main() {
}