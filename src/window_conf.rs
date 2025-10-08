use macroquad::prelude::*;

pub fn conf() -> Conf {
    Conf {
        window_title: "Snake".into(),
        window_width: 1080,
        window_height: 720,
        window_resizable: false, // <- this is the key line
        ..Default::default()
    }
}
