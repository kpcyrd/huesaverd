extern crate homeassistant;
extern crate env_logger;
#[macro_use] extern crate log;

use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

struct Screensaver {
    display: String,
}

impl Screensaver {
    fn new(display: String) -> Screensaver {
        Screensaver {
            display,
        }
    }

    fn run(&self, bin: &str, args: Vec<&str>) {
        Command::new(bin)
             .args(args)
             .env("DISPLAY", &self.display)
             .status()
             .expect("failed to execute process");
    }

    fn turn_on(&self) {
        self.run("xset", vec!("dpms", "force", "on"));
        self.run("xset", vec!("-dpms"));
    }

    fn turn_off(&self) {
        self.run("xset", vec!("dpms", "force", "off"));
        self.run("xset", vec!("+dpms"));
    }
}

fn main() {
    env_logger::init().unwrap();

    let mut args = env::args().skip(1);
    let display = args.next().unwrap();
    let url = args.next().unwrap();
    let entity = args.next().unwrap();

    let client = homeassistant::Client::new(url, None);
    let saver = Screensaver::new(display);

    loop {
        match client.get_state(&entity) {
            Ok(response) => {
                info!("{:?} is {:?}", entity, response.state);
                if response.state == "on" {
                    saver.turn_on();
                } else {
                    saver.turn_off();
                }
            },
            Err(err) => println!("ERROR: {:?}", err),
        };

        thread::sleep(Duration::from_secs(1));
    }
}
