
pub fn qwertz(ch: &str) -> Option<usize> {
    Some(match ch {
        "esc" => 0,
        "^" | "°" => 1,
        "tab" => 2,
        "caps" => 3,
        "shift" | "lshift" => 4,
        "ctrl" | "lctrl" => 5,
        "1" => 7,
        "q" => 8,
        "a" => 9,
        "<" | ">" => 10,
        "win" | "super" => 11,
        "f1" => 12,
        "s" => 15,
        "e" => 20,
        "r" => 26,
        "t" => 32,
        "g" => 33,
        "u" => 44,
        "n" => 46,
        "i" => 50,
        _ => return None,
    })
}
