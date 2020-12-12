use anyhow::*;
use duckyctl::Keyboard;

mod key;

fn main() -> Result<()> {
    let hid = duckyctl::hid()
        .context("Failed to initialize hidapi")?;

    let mut kbd = Keyboard::open(&hid)?;

    let states = &[
        "r",
        "ru",
        "rus",
        "rust",
        "",
        "rust",
        "",
        "rust",
        "",
    ];

    for _ in 0..1 {
        for state in states {
            light_chars(&mut kbd, state)?;
            std::thread::sleep_ms(500);
        }
    }

    kbd.set_all_colors((0xFF, 0xFF, 0));
    kbd.set_static_colors()?;

    std::thread::sleep_ms(2000);
    kbd.enter_autonomous_mode()?;

    Ok(())
}

fn light_chars(kbd: &mut Keyboard, text: &str) -> Result<()> {
    kbd.clear_colors();

    for char in text.chars() {
        if let Some(key) = key::qwertz(&char.to_string()) {
            let color = (0x00, 0xFF, 0x00);
            kbd.set_color(key, color);
        }
    }

    kbd.set_static_colors()?;

    Ok(())
}
