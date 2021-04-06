extern crate fltk;

use crate::{
    actions::{
        book::{
            add_rem::simple::{remove_book_simple, remove_the_book_simple},
            change::{full::change_location, simple::*},
            info::simple::{book_info_simple2, the_book_info_simple},
            utils::check_book,
        },
        genres::full::all_genres,
    },
    books::book_sys::BookSystem,
    books::genres::Genres,
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang, Message,
};

use fltk::{app::App, dialog::alert, table::Table};
use std::{cell::RefCell, rc::Rc};

pub mod add_rem;
pub mod change;
pub mod info;
pub(crate) mod utils;

#[inline]
pub fn action_books(
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
    main_table: &mut Table,
    action: Message,
) {
    if (*book_system).borrow().books.is_empty() {
        alert(
            500,
            500,
            match lang {
                Lang::English => "Book System is empty",
                Lang::Russian => "Книг нет",
            },
        );

        return;
    }

    match {
        let check = all_genres(genres, &*(*book_system).borrow(), app, lang);
        check
    } {
        Some(book) => {
            let label = book.label().unwrap();
            let book = label
                .trim()
                .split('_')
                .take(3)
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();

            if book.len() != 3 {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "Book isn't selected",
                        Lang::Russian => "Книга не выбрана",
                    },
                );
                return;
            }

            if let Ok(index) = {
                let check = check_book(&*(*book_system).borrow(), &book, lang);
                check
            } {
                match action {
                    Message::RemoveBook => {
                        remove_book_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::RemoveTheBook => {
                        remove_the_book_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            lang,
                        );
                    }

                    Message::ChangeTitle => {
                        change_title_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::ChangeAuthor => {
                        change_author_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::ChangePages => {
                        change_pages_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::ChangeLocation => {
                        change_location(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::InfoTheBook => {
                        the_book_info_simple(
                            index,
                            book_system.clone(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                            main_table,
                        );
                    }

                    Message::InfoBook => {
                        book_info_simple2(
                            index,
                            book_system.clone(),
                            reader_base,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }

                    Message::GetBook => {}

                    _ => unreachable!(),
                }
            }
        }

        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Book isn't selected",
                    Lang::Russian => "Книга не выбрана",
                },
            );
            return;
        }
    }
}
