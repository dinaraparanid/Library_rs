extern crate fltk;

use crate::{
    actions::{book::*, read::*},
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, input4::Input4, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
};

use fltk::{
    app,
    app::{channel, App},
    dialog::alert,
    input::*,
    WidgetExt,
};

use std::num::ParseIntError;

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
/// If you have mistakes in input,
/// program will let you know

pub fn give_book(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Find Reader",
        "First Name",
        "Second Name",
        "Middle Name",
        "Age",
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        let (s3, r3) = fltk::app::channel();
                        let mut inp2 = Input3::<Input, Input, IntInput>::new(
                            "Find Book",
                            "Title",
                            "Author",
                            "Pages",
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        inp2.hide();

                                        if let Ok(book) = inp2.set_input() {
                                            unsafe {
                                                let bind;

                                                match check_book(book_system, &book) {
                                                    Ok(x) => bind = x,
                                                    Err(_) => return,
                                                }

                                                let (s4, r4) = fltk::app::channel();
                                                let mut inp3 =
                                                    Input3::<IntInput, IntInput, IntInput>::new(
                                                        "Set Return Date",
                                                        "Day",
                                                        "Month (number)",
                                                        "Year",
                                                    );

                                                inp3.show();
                                                (*inp3.ok).borrow_mut().emit(s4, true);

                                                while app.wait() {
                                                    if let Some(mes) = r4.recv() {
                                                        match mes {
                                                            true => {
                                                                let date_params = inp3.set_input();
                                                                inp3.hide();

                                                                if let Ok(dat) = date_params {
                                                                    match dat
                                                                        .get_unchecked(0)
                                                                        .trim()
                                                                        .parse::<u8>()
                                                                    {
                                                                        Ok(day) => {
                                                                            match dat
									                                            .get_unchecked(1)
									                                            .trim()
									                                            .parse::<u8>() {
									                                            Ok(month) => {
										                                            match dat
											                                            .get_unchecked(2)
											                                            .trim()
											                                            .parse::<u16>() {
											                                            Ok(year) => {
												                                            match Date::new(
													                                            day,
													                                            month,
													                                            year) {
													                                            Err(_) => {
														                                            alert(
															                                            500,
															                                            500,
															                                            "Incorrect Date"
														                                            );
														                                            caretaker.pop();
													                                            },

													                                            Ok(date) => {
														                                            let simple_book = (*book_system
															                                            .books
															                                            .get_unchecked(bind))
															                                            .borrow_mut()
															                                            .get_unused();

														                                            match simple_book {
															                                            None => {
																                                            alert(
																	                                            500,
																	                                            500,
																	                                            "There are no free books"
																                                            );
																                                            caretaker.pop();
															                                            },

															                                            Some(sim) => {
																                                            if (**reader_base
																	                                            .readers
																	                                            .get_unchecked(rind))
																	                                            .borrow()
																	                                            .reading
																	                                            .is_some() {
																	                                            alert(
																		                                            500,
																		                                            500,
																		                                            "This reader is already reading another book"
																	                                            );
																	                                            caretaker.pop();
																	                                            return;
																                                            }

																                                            (*reader_base
																	                                            .readers
																	                                            .get_unchecked(rind))
																	                                            .borrow_mut()
																	                                            .start_reading(
																		                                            (*book_system
																			                                            .books
																			                                            .get_unchecked(bind))
																			                                            .borrow_mut()
																			                                            .books
																			                                            .get_unchecked(sim),
																	                                            );

																                                            (*(*book_system
																	                                            .books
																	                                            .get_unchecked(bind))
																	                                            .borrow_mut()
																	                                            .books
																	                                            .get_unchecked(sim))
																	                                            .borrow_mut()
																	                                            .start_reading(
																		                                            reader_base.readers
																		                                                       .get_unchecked(rind),
																		                                            date,
																	                                            );

																                                            fltk::dialog::message(
																	                                            500,
																	                                            500,
																	                                            "Book successfully given to reader"
																                                            );

																                                            book_system.save();
																                                            reader_base.save();
															                                            }
														                                            }
													                                            }
												                                            }
											                                            }

											                                            Err(_) => {
												                                            alert(
													                                            500,
													                                            500,
													                                            "Year input error"
												                                            );

												                                            caretaker.pop();

												                                            println!("{:?}",
												                                                     dat.get_unchecked(2)
												                                                        .trim()
												                                                        .parse::<u16>()
												                                            );
											                                            }
										                                            }
									                                            }

									                                            Err(_) => {
										                                            alert(
											                                            500,
											                                            500,
											                                            "Month input error"
										                                            );

										                                            caretaker.pop();

										                                            println!("{:?}",
										                                                     dat.get_unchecked(1)
										                                                        .trim()
										                                                        .parse::<u8>()
										                                            );
									                                            }
								                                            }
                                                                        }

                                                                        Err(_) => {
                                                                            alert(
                                                                                500,
                                                                                500,
                                                                                "Day input error",
                                                                            );

                                                                            caretaker.pop();

                                                                            println!(
                                                                                "{:?}",
                                                                                dat.get_unchecked(
                                                                                    0
                                                                                )
                                                                                .trim()
                                                                                .parse::<u8>()
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            false => (),
                                                        }
                                                    } else if !inp3.shown() {
                                                        caretaker.pop();
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                            } else if !inp2.shown() {
                                caretaker.pop();
                                break;
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            caretaker.pop();
            break;
        }
    }
}

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
/// If you have mistakes in input,
/// program will let you know

pub fn get_book(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Find Reader",
        "First Name",
        "Second Name",
        "Middle Name",
        "Age",
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        let (s3, r3) = fltk::app::channel();
                        let mut inp2 = Input3::<Input, Input, IntInput>::new(
                            "Find Book",
                            "Title",
                            "Author",
                            "Pages",
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        inp2.hide();

                                        if let Ok(book) = inp2.set_input() {
                                            let bind;

                                            match check_book(book_system, &book) {
                                                Ok(x) => bind = x,
                                                Err(_) => return,
                                            }

                                            unsafe {
                                                let simple =
                                                    (*book_system.books.get_unchecked(bind))
                                                        .borrow_mut()
                                                        .find_by_reader(
                                                            reader_base.readers.get_unchecked(rind),
                                                        );

                                                match simple {
                                                    None => {
                                                        alert(
                                                            500,
                                                            500,
                                                            "This reader wasn't reading this book",
                                                        );
                                                        caretaker.pop();
                                                    }

                                                    Some(sim) => {
                                                        (*reader_base
                                                            .readers
                                                            .get_unchecked_mut(rind))
                                                        .borrow_mut()
                                                        .finish_reading();

                                                        match (*(*book_system
		                                                    .books
		                                                    .get_unchecked(bind))
		                                                    .borrow_mut()
		                                                    .books
		                                                    .get_unchecked(sim))
		                                                    .borrow_mut()
		                                                    .finish_reading()
	                                                    {
		                                                    Ok(_) => fltk::dialog::message(
			                                                    500,
			                                                    500,
			                                                    "Book is returned",
		                                                    ),

		                                                    Err(_) => fltk::dialog::message(
			                                                    500,
			                                                    500,
			                                                    "Book is returned, but reader is late",
		                                                    ),
	                                                    }

                                                        book_system.save();
                                                        reader_base.save();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                            } else if !inp2.shown() {
                                caretaker.pop();
                                break;
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            caretaker.pop();
            break;
        }
    }
}
