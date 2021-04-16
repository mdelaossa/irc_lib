pub struct Negotiator {
    done: bool,
    messages: std::slice::Iter<'static, &'static str>
}

impl Negotiator {
    const MESSAGES: [&'static str; 2] = ["NICK testing_a_rusty_thing", "USERNAME rusty 0 * None"];

    pub fn new() -> Self {
        Negotiator {
            done: false,
            messages: Negotiator::MESSAGES.iter()        
        }
    }
}

impl Default for Negotiator {
    fn default() -> Self {
        Negotiator::new()
    }
}

impl Iterator for Negotiator {
    type Item = &'static str;

    fn next(&mut self) -> Option<&'static str> {
        if self.done { return None }
        
        match self.messages.next() {
            Some(n) => Some(n),
            None => {
                self.done = true;
                None
            }
        }
    }
}