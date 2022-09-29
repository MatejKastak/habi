#[macro_use]
extern crate log;

use notify_rust::Notification;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::Source;
use std::io::BufReader;
use std::{thread, time};

fn show_habit(habit: &str) {
    // play_notification_sound(handle);

    debug!("Show the notification");

    Notification::new()
        .summary(habit)
        .body("habi")
        .icon("firefox")
        .show()
        .unwrap();

    debug!("Notification returned");
}

fn play_notification_sound(handle: &rodio::OutputStreamHandle) {
    debug!("Playing notification sound");
    let file =
        std::fs::File::open("/usr/share/sounds/freedesktop/stereo/window-attention.oga").unwrap();

    debug!("Decode a new source");
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    debug!("Playing the sound...");
    handle
        .play_raw(source.amplify(1.5).convert_samples())
        .unwrap();
    debug!("Playing the sound... DONE");
}

// TODO: Specify the `sleep_time` from command line
fn main() {
    env_logger::init();

    info!("Habi started");

    let short = vec![
        "Straighten your back",
        "Head exercises",
        "Chin tucks",
        "Stretches",
        "Hand exercises",
        "Drink water",
        "Look into distance",
        "Moving the eyes",
        "Eye blinking",
    ];

    debug!("Initialized data vectors");

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    debug!("Opened `OutputStream`");

    let mut rng = thread_rng();

    debug!("Intialized rng");

    debug!("Starting main execution loop");

    // Sleep time between habits in minutes
    let sleep_time: u32 = 4;

    loop {
        debug!("Generate random habit");
        let habit = short.choose(&mut rng).unwrap();
        debug!("Got habit '{}'", habit);
        show_habit(habit);
        debug!("Displayed habit, now sleeping for '{}' minutes", sleep_time);
        thread::sleep(time::Duration::from_secs((sleep_time * 60).into()))
    }
}
