use std::collections::HashMap;

struct Charset {
    u: &'static str,
    l: &'static str,
    n: &'static str,
    s: &'static str,
    all: &'static str,
}

impl Charset {
    pub const fn new() -> Charset {
        Charset {
            u: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            l: "abcdefghijklmnopqrstuvwxyz",
            n: "0123456789",
            s: "!@#$%^&*",
            all: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|",
        }
    }
}

pub fn default_charset() -> HashMap<char, &'static str> {
    let charset = Charset::new();
    let mut char_map = HashMap::new();
    char_map.insert('u', charset.u);
    char_map.insert('l', charset.l);
    char_map.insert('n', charset.n);
    char_map.insert('s', charset.s);
    char_map.insert('_', charset.all);

    char_map
}