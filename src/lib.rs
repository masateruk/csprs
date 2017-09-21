#[derive(Clone, Debug)]
enum Event {
    Name(String),
}

impl Event {
    fn show(&self) {
        match *self {
            Event::Name(ref s) => println!("{}", s)
        }
    }
}

#[derive(Clone, Debug)]
enum Process {
    Stop,
    Skip,
    Prefix { ev: Event, p: Box<Process> },
}

impl Process {
    pub fn stop() -> Process {
        Process::Stop
    }

    pub fn skip() -> Process {
        Process::Skip
    }

    pub fn prefix(ev: &Event, p: Process) -> Process {
        Process::Prefix { ev: ev.clone(), p: Box::new(p) }
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
        Process::prefix(&Event::Name("e".to_string()), Process::skip()).run()
    }
}
