trait Event {
    fn show(&self);
}

impl Event for String {
    fn show(&self) {
        println!("{}", *self)
    }
}

enum Process<'a> {
    Stop,
    Skip,
    Prefix { ev: Box<Event + 'a>, p: Box<Process<'a>> },
}

impl<'a> Process<'a> {
    pub fn stop() -> Process<'a> {
        Process::Stop
    }

    pub fn skip() -> Process<'a> {
        Process::Skip
    }

    pub fn prefix<'b, T>(ev: T, p: Process<'b>) -> Process<'b>
        where T: Event + 'b {
        Process::Prefix { ev: Box::new(ev), p: Box::new(p) }
    }

    pub fn run(&self) {
        use Process::*;
        match *self {
            Stop => loop {},
            Skip => (),
            Prefix { ref ev, ref p } => {
                ev.show();
                p.run()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let p = Process::prefix("e".to_string(), Process::skip());
        p.run();
    }
}
