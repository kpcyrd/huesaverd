extern crate homeassistant;
extern crate env_logger;
#[macro_use] extern crate log;

use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn run(bin: &str, args: Vec<&str>) {
    Command::new(bin)
         .args(args)
         .status()
         .expect("failed to execute process");
}

fn turn_on() {
    run("xset", vec!("dpms", "force", "on"));
    run("xset", vec!("-dpms"));
}

fn turn_off() {
    run("xset", vec!("dpms", "force", "off"));
    run("xset", vec!("+dpms"));
}

fn main() {
    env_logger::init().unwrap();

    let mut args = env::args().skip(1);
    let url = args.next().unwrap();
    let entity = args.next().unwrap();
    let client = homeassistant::Client::new(url, None);

    loop {
        match client.get_state(&entity) {
            Ok(response) => {
                info!("{:?} is {:?}", entity, response.state);
                if response.state == "on" {
                    turn_on();
                } else {
                    turn_off();
                }
            },
            Err(err) => println!("ERROR: {:?}", err),
        };

        thread::sleep(Duration::from_secs(1));
    }
}
