extern crate getopts;
extern crate portmidi;

use std::env;
use std::time::Duration;
use std::thread;
use getopts::Options;
use portmidi::PortMidi;

fn print_devices(pm: &PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut opts = Options::new();
    opts.optopt("", "device_id", "MIDI device ID", "N");
    let matches = opts.parse(&args[1..]).unwrap();

    let context = PortMidi::new().unwrap();

    let device_id = matches.opt_str("device_id");
    let device_id = match device_id {
        Some(v) => v.parse::<i32>().unwrap(),
        Nothing => {
            print_devices(&context);
            return;
        }
    };

    println!("Device: {}", device_id);

    let info = context.device(device_id).unwrap();
    println!("Device: {:?}", info);

    let timeout = Duration::from_millis(10);

    let in_port = context.input_port(info, 1024).unwrap();
    while let Ok(_) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read_n(1024) {
            println!("{:?}", event);
        }
        thread::sleep(timeout);
    }
}
