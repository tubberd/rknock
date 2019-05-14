extern crate clap;
extern crate dns_lookup;

use clap::*;

use std::process::Command;

fn main() {
	let app_matches = App::new("rknock")
		.about("Port knocking client written in Rust")
		.author("Daniel Wanner <daniel.wanner@pm.me>")
		.version("0.1.0")
		.arg(
			Arg::with_name("ipv4")
				.short("4")
				.long("ipv4")
				.help("Force usage of IPv4")
				.conflicts_with("ipv6"),
		)
		.arg(
			Arg::with_name("ipv6")
				.short("6")
				.long("ipv6")
				.help("Force usage of IPv6")
				.conflicts_with("ipv4"),
		)
		.arg(
			Arg::with_name("interval")
				.short("i")
				.long("interval")
				.help("Sets the interval between knocks")
				.takes_value(true)
				.default_value("1000"),
		)
		.arg(
			Arg::with_name("host")
				.index(1)
				.required(true)
				.help("The host to knock at"),
		)
		.arg(
			Arg::with_name("ports")
				.index(2)
				.required(true)
				.help("The ports to knock, in order"),
		)
		.get_matches();

	let interval_raw = app_matches.value_of("interval").unwrap();
	let interval = interval_raw
		.parse::<u64>()
		.expect("Interval needs to be an integer!");

	let host = app_matches.value_of("host").unwrap().to_string();
	let ports_raw = app_matches.value_of("ports").unwrap();
	let mut ports: Vec<u16> = Vec::new();
	let ports_arr: Vec<&str> = ports_raw.split(",").collect();

	for port in ports_arr {
		let port = String::from(port)
			.parse::<u16>()
			.expect("Ports need to be an unsigned 16-bit integer!");
		ports.push(port);
	}

	if app_matches.occurrences_of("ipv6") > 0 {
		println!("Knocking v6...");
		knock_v6(host, ports, interval);
	} else {
		println!("Knocking v4...");
		knock_v4(host, ports, interval)
	}
}

fn knock_v4(host: String, ports: Vec<u16>, interval: u64) {
	let lookup = dns_lookup::lookup_host(&host).unwrap();
	let mut ip_addr_opt = None;
	for ip in lookup {
		if ip.is_ipv4() {
			ip_addr_opt = Some(ip);
		}
	}
	if ip_addr_opt.is_none() {
		println!("Could not resolve host!");
	}
	let ip_addr = ip_addr_opt.unwrap();
	println!("Knocking. Host: {}", ip_addr);

	let mut index = 1;

	for port in ports {
		println!("...{}...", index);
		nmap_knock(false, &host, port, &interval);
		index += 1;
	}
}

fn knock_v6(host: String, ports: Vec<u16>, interval: u64) {
	let lookup = dns_lookup::lookup_host(&host).unwrap();
	let mut ip_addr_opt = None;
	for ip in lookup {
		if ip.is_ipv4() {
			ip_addr_opt = Some(ip);
		}
	}
	if ip_addr_opt.is_none() {
		println!("Could not resolve host!");
	}
	let ip_addr = ip_addr_opt.unwrap();
	println!("Knocking. Host: {}", ip_addr);

	let mut index = 1;
	for port in ports {
		println!("...{}...", index);
		nmap_knock(true, &host, port, &interval);
		index += 1;
	}
}

fn nmap_knock(ipv6: bool, host: &String, port: u16, duration: &u64) {
	let ip_arg;
	if ipv6 {
		ip_arg = "-6";
	} else {
		ip_arg = "-4";
	}

	let cmd = Command::new("nmap")
		.arg(ip_arg)
		.arg("-Pn")
		.arg("--host-timeout")
		.arg(&duration.to_string())
		.arg("--max-retries")
		.arg("0")
		.arg("-p")
		.arg(&port.to_string())
		.arg(host)
		.output();

	if cmd.is_err() {
		println!("Error!");
	}
}
