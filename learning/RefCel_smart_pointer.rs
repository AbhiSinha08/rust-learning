pub mod library {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: u32,
        max: u32
    }
    impl<'a, T: Messenger> LimitTracker<'a, T> {
        pub fn new(messenger: &'a T, max: u32) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max
            }
        }
        pub fn set_val(&self, value: u32) {
            if value > self.max {
                self.messenger.send("error greater than max value");
            }
        }
    }
}

mod tests {
    use super::library;
    use std::cell::RefCell;

    struct Saver {
        messages: RefCell<Vec<String>>
    }
    impl library::Messenger for Saver {
        fn send(&self, msg: &str) {
            let message = String::from(msg);

            let mut mut_ref = self.messages.borrow_mut();
            mut_ref.push(message);
        }
    }

    pub fn test () {
        let messenger = Saver { messages: RefCell::new(Vec::new()) };
        let tracker = library::LimitTracker::new(&messenger, 100);
        tracker.set_val(110);

        println!("message history: {:?}", messenger.messages);
    }
}

fn main() {
    tests::test();
}