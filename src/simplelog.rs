extern crate simplelog;

use ::{BasicLogger, Formatter3164, Formatter5424};

impl simplelog::SharedLogger for BasicLogger<Formatter3164> {
    fn level(&self) -> simplelog::LevelFilter {
        simplelog::LevelFilter::Info
    }

    fn config(&self) -> Option<&simplelog::Config> {
        None
    }

    fn as_log(self: Box<Self>) -> Box<dyn log::Log> {
        Box::new(*self)
    }
}

impl simplelog::SharedLogger for BasicLogger<Formatter5424> {
    fn level(&self) -> simplelog::LevelFilter {
        simplelog::LevelFilter::Info
    }

    fn config(&self) -> Option<&simplelog::Config> {
        None
    }

    fn as_log(self: Box<Self>) -> Box<dyn log::Log> {
        Box::new(*self)
    }
}

