pub trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Debug)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 7.5 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
pub struct Person<'a> {
    first_name: &'a str,
    middle_name: &'a str,
    last_name: &'a str,
}

impl<'a> Messenger for Person<'a> {
    fn send(&self, msg: &str) {
        println!(
            "{}, {} {} {}",
            msg, self.first_name, self.middle_name, self.last_name,
        )
    }
}

fn main() {
    let sampel = LimitTracker::new(
        &Person {
            first_name: "John",
            middle_name: "Jingleheimer",
            last_name: "Smith",
        },
        1000,
    );

    sampel.messenger.send("Hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
