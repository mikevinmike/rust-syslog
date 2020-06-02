extern crate syslog;

use std::net::ToSocketAddrs;

fn main() {
    let formatter = syslog::Formatter5424 {
        facility: syslog::Facility::LOG_CRON,
        hostname: Some("my-host".to_string()),
        process: "my-process".to_string(),
        pid: 0,
    };

    let log_server_address = "0.0.0.0:6514".to_socket_addrs().unwrap().next().unwrap();
    let local_address = "0.0.0.0:6515".to_socket_addrs().unwrap().next().unwrap();
    let logger = syslog::udp(formatter, local_address, log_server_address).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(syslog::BasicLogger::<syslog::Formatter5424>::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .ok();

    log::info!("hello world");
}
