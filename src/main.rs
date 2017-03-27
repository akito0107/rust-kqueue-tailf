extern crate nix;

use nix::sys::event::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;

fn main() {
    let filename = "/tmp/test";

    let mut file = OpenOptions::new().read(true).open(filename).unwrap();
    // let fd = File::open(filename).unwrap().as_raw_fd();
    let fd = file.as_raw_fd();
    let kq = match kqueue() {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    };
    println!("{:?}", fd);
    println!("{:?}", kq);
    let kev = nix::sys::event::KEvent::new(fd as usize,
                                           EventFilter::EVFILT_READ,
                                           EV_ADD,
                                           FilterFlag::all(),
                                           0,
                                           0);

    let target = vec![kev];
    match kevent(kq, &target, &mut Vec::new(), 0) {
        Ok(_) => println!("OK"),
        Err(e) => panic!("init event failed: {:?}", e),
    }
    let kev2 =
        nix::sys::event::KEvent::new(0, EventFilter::EVFILT_READ, EV_ADD, FilterFlag::all(), 0, 0);

    loop {
        let mut source = vec![kev2];

        match kevent_ts(kq, &Vec::new(), &mut source, None) {
            Ok(_) => println!("loop success"),
            Err(e) => panic!("kv loop panic {:?}", e),
        }

        let mut buf = Vec::new();

        match file.read_to_end(&mut buf) {
            Ok(size) => println!("read {:?}", size),
            Err(e) => panic!("read to end erro {:?}", e),
        }

        println!("{:?}", String::from_utf8(buf).unwrap());
    }
}
