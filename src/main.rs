use cbor::{Decoder, Encoder};
use daemonize::Daemonize;
use std::fs::{self, File};
use std::io::{prelude::*, Error, ErrorKind};
use std::process::Command;
use std::thread;

struct Service {
    id: i32,
    path: String,
    manual: bool,
    started: bool,
    consider_started_on_text: String,
    timeout: i32,
    consider_started_on_time: i32,
}

struct StartupChain {
    child_chains: Vec<Box<StartupChain>>,
    child_services: Vec<Box<Service>>,
}

fn mk_chain(vec: Vec<(i32, String, bool, String, i32, i32)>) -> Result<StartupChain, ()> {
    let root_chain = StartupChain {
        child_chains: Vec::new(),
        child_services: Vec::new(),
    };
    for (id, task, has_manual, on_text_started, timeout_ms, on_time_started) in vec {}
    Ok(chain)
}

fn read_cbor(path: &str) -> Result<StartupChain, Error> {
    let mut conf = File::open(path)?;
    let mut bin_data = Vec::<u8>::new();
    conf.read_to_end(&mut bin_data)?;
    let mut d = Decoder::from_bytes(bin_data);
    let startup_chain: Vec<(i32, String, bool, String, i32, i32)> = d
        .decode()
        .collect::<Result<_, _>>()
        .unwrap_or(vec![(0, String::new(), false, String::new(), 0, 0)]);
    match mk_chain(startup_chain) {
        Ok(c) => Ok(c),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "")),
    }
}

fn startup(path: &str) -> Result<(), Error> {
    let mut startup_chain = read_cbor(path)?;
    for service in startup_chain.child_services {
        Command::new(&service.path).spawn()?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    startup("/etc/laosp/startup")
}
