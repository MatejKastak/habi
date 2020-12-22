 #[macro_use]
extern crate log;

use notify_rust::Notification;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rodio::Source;
use std::io::BufReader;
use std::{thread, time};

fn show_habit(handle: &rodio::OutputStreamHandle, habit: &str) -> u32 {
    let mut sleep_time = 0;

    play_notification_sound(handle);

    debug!("Show the notification");

    Notification::new()
        .summary(habit)
        .body("habi")
        .icon("firefox")
        .action("default", "default")
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            // left click
            "default" => sleep_time = 10,
            // right click
            "__closed" => sleep_time = 0,
            _ => sleep_time = 0,
        });

    debug!("Notification returned");

    return sleep_time;
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

fn main() {
    env_logger::init();

    info!("Habi started");

    let short = vec![
        "Straighten your back",
        "Head exercises",
        "Chin tucks",
        "Stretches",
        "Wall stand",
        "Wall exercises",
        "Grease the groove",
        "Push-ups",
        "Squads",
        "Plank",
        "L-sit",
        "Wash the face",
        "Hand exercises",
        "Drink water",
        "Eye exercises",
        "Look into distance",
        "Moving the eyes",
        "Eye blinking",
        "Typing practice",
    ];

    let long = vec![
        "Breathing exercise",
        "Workout",
        "Meditation",
        "Read the book",
        "Walk",
    ];

    debug!("Initialized data vectors");

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    debug!("Opened `OutputStream`");

    let mut rng = thread_rng();

    debug!("Intialized rng");

    debug!("Starting main execution loop");
    loop {
        debug!("Generate random habit");
        // 1 in 5 to choose long
        let habit = if rng.gen_range(0, 5) == 0 {
            long.choose(&mut rng).unwrap()
        } else {
            short.choose(&mut rng).unwrap()
        };
        debug!("Got habit '{}'", habit);
        let sleep_time = show_habit(&handle, habit);
        debug!("Displayed habit, now sleeping for '{}' minutes", sleep_time);
        thread::sleep(time::Duration::from_secs((sleep_time * 60).into()))
    }
}
