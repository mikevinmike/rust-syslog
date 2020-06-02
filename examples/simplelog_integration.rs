extern crate log;
extern crate simplelog;
extern crate syslog;

use std::borrow::Cow;
use std::net::ToSocketAddrs;

use log::LevelFilter;

fn main() {
    let formatter = syslog::Formatter5424 {
        facility: syslog::Facility::LOG_CRON,
        hostname: Some("my-host".to_string()),
        process: "my-process".to_string(),
        pid: 0,
    };

    let log_server_address = "syslog.monitor.stransky.co.at:49021".to_socket_addrs().unwrap().next().unwrap();
    let local_address = "0.0.0.0:6515".to_socket_addrs().unwrap().next().unwrap();
    let logger = syslog::udp(formatter, local_address, log_server_address).expect("could not connect to syslog");
    let syslogger = Box::new(syslog::BasicLogger::<syslog::Formatter5424>::new(logger));

    let mut config_builder = simplelog::ConfigBuilder::new();
    config_builder.set_time_format_str("%Y-%m-%d %H:%M:%S %z");

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(simplelog::LevelFilter::Debug, config_builder.build(), simplelog::TerminalMode::Mixed),
        syslogger
    ]).expect("failed to initialize combined logger");

    log::info!("hello world");
}

