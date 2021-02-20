use std::{
    fs::File,
    io::{Read, Write},
};

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod books;

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod reading;

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod change;

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod actions;

#[allow(dead_code)]
#[allow(unused_imports)]
pub mod restore;

#[allow(dead_code)]
#[allow(unused_imports)]
mod tests;

/// Language.
/// Helps to choose preferred language

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    English,
    Russian,
}

impl Default for Lang {
    /// Creates default localization.
    /// By default it's english

    #[inline]
    fn default() -> Self {
        Lang::English
    }
}

impl Lang {
    /// Creates new localization.
    /// Reads from file

    #[inline]
    pub fn new() -> Self {
        let mut buf = String::new();

        File::open("src/utils/lang.bin")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        match buf.trim().to_lowercase().as_str() {
            "english" => Lang::English,
            "russian" => Lang::Russian,
            _ => Lang::default(),
        }
    }

    /// Changes language in file

    #[inline]
    pub fn change(new_lang: Lang) {
        File::create("src/utils/lang.bin")
            .unwrap()
            .write(
                match new_lang {
                    Lang::English => "english",
                    Lang::Russian => "russian",
                }
                .as_bytes(),
            )
            .unwrap();
    }
}
