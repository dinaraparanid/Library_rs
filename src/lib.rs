//#![feature(option_result_unwrap_unchecked)]

use std::{
    fs::File,
    io::{Read, Write},
};

pub mod actions;
pub mod books;
pub mod change;
pub mod reading;
pub mod restore;

#[allow(dead_code)]
#[allow(unused_imports)]
#[deprecated(note = "It has been a lot of changes since I need it,\
     since everything works correct, I have no need for tests")]
mod tests;

/// All messages, which used to call functions

#[derive(Clone, Copy)]
pub enum Message {
    AddReader,
    RemoveReader,
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    InfoReaderReading,
    InfoReaderAllBooks,
    AddBooks,
    RemoveBook,
    RemoveTheBook,
    ChangeTitle,
    ChangeAuthor,
    ChangePages,
    ChangeLocation,
    InfoTheBook,
    InfoBook,
    GiveBook,
    GetBook,
    ChangeReturnDate,
    ShowAllBooks,
    ShowGenres,
    ShowAuthor,
    AddGenre,
    RemoveGenre,
    CustomizeBookGenre,
    PrevData,
    NextData,
    English,
    Russian,
    Help,
}

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
