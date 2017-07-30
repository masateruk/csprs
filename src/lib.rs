#[derive(Clone, Debug)]
enum Event {
    Name(String)
}

#[derive(Clone, Debug)]
enum Process {
    Stop,
    Skip,
    Prefix { ev: Event, p: Box<Process> }
}

impl Process {
    pub fn stop() -> Process {
        Process::Stop
    }

    pub fn skip() -> Process {
        Process::Skip
    }

    pub fn prefix(ev: &Event, p: &Process) -> Process {
        Process::Prefix { ev: ev.clone(), p: Box::new(p.clone()) }
    }

    pub fn run(&mut self) {
        use Process::*;
        match *self {
            Stop => loop {},
            Skip => (),
            Prefix { ev: Event::Name(_), ref mut p } => p.run()
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        Process::prefix(&Event::Name("e".to_string()), &Process::skip()).run()
    }
}
