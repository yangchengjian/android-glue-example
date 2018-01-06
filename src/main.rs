extern crate android_glue;

use std::sync::mpsc::channel;

fn main() {
    android_glue::write_log("main() has been called");
    let (sender, receiver) = channel::<android_glue::Event>();
    android_glue::add_sender(sender);
    loop {
        android_glue::write_log(&format!("_______EVENT_________{:?}", receiver.recv()));
    }
}
