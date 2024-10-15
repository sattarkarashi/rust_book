// Mock tests

pub trait Messenger {
    fn send(&self, msg:&str);
}

pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value:usize,
    max:usize
}

impl <'a, T> LimitTracker<'a,T> where T:messenger, {
    pub fn new(
        messenger: &'a T,
        max:usize,
    ) -> LimitTracker<'a, T> {
        LimitTracker{
            messenger,
            value:0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;
        let percentage_of_max = self.value as f64/self.max as f64;

        if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning:You're at 75% of your quota!");
        }
    }
}