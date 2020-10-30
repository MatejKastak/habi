use notify_rust::{Hint, Notification};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::{thread, time};

fn show_habit(habit: &str) -> u32 {
    let mut sleep_time = 0;
    Notification::new()
        .summary(habit)
        .body("habi")
        .icon("firefox")
        .action("default", "default")
        .hint(Hint::Resident(true))
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            // left click
            "default" => sleep_time = 10,
            // right click
            "__closed" => sleep_time = 0,
            _ => sleep_time = 0,
        });
    return sleep_time;
}

fn main() {
    let short = vec![
        "straighten your back",
        "head exercises",
        "chin tucks",
        "stretches",
        "wall stand",
        "wall exercises",
        "grease the groove",
        "push-ups",
        "squads",
        "plank",
        "l-sit",
        "wash the face",
        "hand exercises",
        "drink water",
        "eye exercises",
        "look into distance",
        "moving the eyes",
        "eye blinking",
    ];

    let long = vec![
        "breathing exercise",
        "workout",
        "meditation",
        "read the book",
        "walk",
    ];

    let mut rng = thread_rng();

    loop {
        // 1 in 5 to choose long
        let habit = if rng.gen_range(0, 5) == 0 {
            long.choose(&mut rng).unwrap()
        } else {
            short.choose(&mut rng).unwrap()
        };
        let sleep_time = show_habit(habit);
        thread::sleep(time::Duration::from_secs((sleep_time * 60).into()))
    }
}
