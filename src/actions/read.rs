extern crate fltk;

use crate::{
    actions::{book::book_info_simple, tables::*},
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, input4::Input4, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
};

use fltk::{
    app,
    app::App,
    dialog::alert,
    draw,
    frame::Frame,
    group::VGrid,
    input::*,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::Table,
    window::SingleWindow,
};

use std::{cell::RefCell, cmp::max, num::ParseIntError, rc::Rc};

/// Messages for info menu

#[derive(Clone, Copy)]
enum MessageReader {
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    RemoveThis,
}

/// Function that checks if input was empty

#[inline]
pub(crate) fn empty_inp_reader(inp: &Vec<String>) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(500, 500, "Name is empty");
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(500, 500, "Family is empty");
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(500, 500, "Father is empty");
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(500, 500, "Age is empty");
            true
        } else {
            false
        };
    }
}

/// Function that checks if input is correct.
/// Returns index of book, if it exists.
/// or calls alert and returns error

#[inline]
pub(crate) fn check_reader(reader_base: &ReaderBase, reader: &Vec<String>) -> Result<usize, ()> {
    let age;
    let ind;

    unsafe {
        if empty_inp_reader(reader) {
            return Err(());
        }

        match reader.get_unchecked(3).trim().parse::<u8>() {
            Ok(x) => age = x,
            Err(_) => {
                alert(500, 500, "Age input error");
                return Err(());
            }
        }

        ind = reader_base.find_reader(
            reader.get_unchecked(0),
            reader.get_unchecked(1),
            reader.get_unchecked(2),
            age,
        );
    }

    return match ind {
        None => {
            alert(500, 500, "Reader isn't found");
            Err(())
        }

        Some(i) => Ok(i),
    };
}

/// Function that returns index of simple book.
/// Panics if book is not in vec of books.

#[inline]
pub(crate) fn get_book_ind(book_system: &BookSystem, book: *mut Book) -> usize {
    if book.is_null() {
        panic!("nullptr in actions/read get_book_ind");
    }

    unsafe {
        match book_system.find_book(&(*book).title, &(*book).author, (*book).pages) {
            None => panic!("Index out of range"),
            Some(ind) => {
                (*(**book_system.books.get_unchecked(ind)).borrow().books)
                    .iter()
                    .position(|x| &*(**x).borrow() as *const Book == book)
                    .unwrap()
                    + 1
            }
        }
    }
}

/// Removes already known reader

#[inline]
fn remove_reader_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    match reader_base.remove_reader(ind) {
        Ok(_) => {
            fltk::dialog::message(500, 500, "Successfully removed");
            reader_base.save();
            book_system.save();
        }

        Err(_) => {
            alert(500, 500, "Reader not found");
            caretaker.pop();
        }
    }
}

/// Change name of already known reader

#[inline]
fn change_name_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_name = Input1::<Input>::new("New Name", "New Name");

    caretaker.add_memento(reader_base, book_system, genres);

    get_name.show();
    (*get_name.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_name.hide();

                    if let Ok(new_name) = get_name.set_input() {
                        unsafe {
                            match reader_base.change_name(ind, new_name.get_unchecked(0).clone()) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                    caretaker.pop();
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                    caretaker.pop();
                                }

                                Err(_) => {
                                    alert(500, 500, "New name is empty");
                                    caretaker.pop();
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_name.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Change 2-nd name of already known reader

#[inline]
fn change_family_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_family = Input1::<Input>::new("New 2-nd Name", "New 2-nd Name");

    caretaker.add_memento(reader_base, book_system, genres);

    get_family.show();
    (*get_family.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_family.hide();

                    if let Ok(new_family) = get_family.set_input() {
                        unsafe {
                            match reader_base
                                .change_family(ind, new_family.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                    caretaker.pop();
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                    caretaker.pop();
                                }

                                Err(_) => {
                                    alert(500, 500, "New 2-nd name is empty");
                                    caretaker.pop();
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_family.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Change middle name of already known reader

#[inline]
fn change_father_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_father = Input1::<Input>::new("New Middle Name", "New Middle Name");

    caretaker.add_memento(reader_base, book_system, genres);

    get_father.show();
    (*get_father.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_father.hide();

                    if let Ok(new_father) = get_father.set_input() {
                        unsafe {
                            match reader_base
                                .change_father(ind, new_father.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                    caretaker.pop();
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                    caretaker.pop();
                                }

                                Err(_) => {
                                    alert(500, 500, "New mid. name is empty");
                                    caretaker.pop();
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_father.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Changes age of already known reader

#[inline]
fn change_age_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_age = Input1::<IntInput>::new("New Age", "New Age");

    caretaker.add_memento(reader_base, book_system, genres);

    get_age.show();
    (*get_age.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_age.hide();

                    if let Ok(new_age) = get_age.set_input() {
                        if new_age.first().unwrap().is_empty() {
                            alert(500, 500, "New age is empty");
                            caretaker.pop();
                            return;
                        }

                        unsafe {
                            match reader_base.change_age(ind, new_age.get_unchecked(0).clone()) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Age input error");
                                    caretaker.pop();
                                }

                                Err(_) => {
                                    alert(500, 500, "Reader already exists");
                                    caretaker.pop();
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_age.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Function that gives information
/// about already known reader

pub fn reader_info_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let mut wind;
    let mut table2;

    unsafe {
        wind = SingleWindow::new(
            800,
            100,
            570,
            600,
            format!(
                "{} {} {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().name,
                (*reader_base.readers.get_unchecked(ind)).borrow().family,
                (*reader_base.readers.get_unchecked(ind)).borrow().father,
            )
            .as_str(),
        )
        .center_screen();

        let mut table1 = VGrid::new(0, 0, 570, 100, "");
        table1.set_params(6, 1, 1);

        table1.add(&Frame::new(
            10,
            50,
            100,
            30,
            format!(
                "First Name: {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().name
            )
            .as_str(),
        ));

        table1.add(&Frame::new(
            30,
            50,
            100,
            30,
            format!(
                "Second Name: {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().family
            )
            .as_str(),
        ));

        table1.add(&Frame::new(
            50,
            50,
            100,
            30,
            format!(
                "Middle Name: {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().father
            )
            .as_str(),
        ));

        table1.add(&Frame::new(
            70,
            50,
            100,
            30,
            format!(
                "Age: {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().age
            )
            .as_str(),
        ));

        table1.add(&Frame::new(
            70,
            50,
            100,
            30,
            format!(
                "Reading now: {}",
                if (**reader_base.readers.get_unchecked(ind))
                    .borrow()
                    .reading
                    .is_some()
                {
                    (*(**reader_base.readers.get_unchecked(ind))
                        .borrow()
                        .reading
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .title
                    .clone()
                        + " "
                        + (*(**reader_base.readers.get_unchecked(ind))
                            .borrow()
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .author
                        .as_str()
                        + " "
                        + (*(**reader_base.readers.get_unchecked(ind))
                            .borrow()
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .pages
                        .to_string()
                        .as_str()
                        + "  ("
                        + get_book_ind(
                            book_system,
                            (*(**reader_base.readers.get_unchecked(ind))
                                .borrow()
                                .reading
                                .as_ref()
                                .unwrap()
                                .upgrade()
                                .unwrap())
                            .as_ptr(),
                        )
                        .to_string()
                        .as_str()
                        + ")"
                } else {
                    "None".to_string()
                }
            )
            .as_str(),
        ));

        table1.add(&Frame::new(
            90,
            50,
            100,
            30,
            format!("Books read by reader:").as_str(),
        ));

        table1.auto_layout();

        table2 = Table::new(0, 127, 570, 600, "");
        table2.set_rows(max(
            30,
            (**reader_base.readers.get_unchecked(ind))
                .borrow()
                .books
                .len() as u32,
        ));
        table2.set_row_header(true);
        table2.set_cols(4);
        table2.set_col_header(true);
        table2.set_col_width_all(130);
        table2.end();
    }

    wind.end();

    let mut menu = MenuBar::new(0, 0, 190, 30, "");
    wind.add(&menu);

    let (s, r) = app::channel();

    menu.add_emit(
        "&Change/Change name\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeName,
    );

    menu.add_emit(
        "&Change/Change family\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFamily,
    );

    menu.add_emit(
        "&Change/Change father\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFather,
    );

    menu.add_emit(
        "&Change/Change age\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeAge,
    );

    menu.add_emit(
        "Remove reader\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::RemoveThis,
    );

    wind.show();

    let base_ptr = reader_base as *mut ReaderBase;
    let sys_ptr = book_system as *mut BookSystem;

    table2.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        fltk::table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        fltk::table::TableContext::ColHeader => draw_header(
            &format!(
                "{}",
                match col {
                    0 => "Title",
                    1 => "Author",
                    2 => "Pages",
                    _ => "Number of book",
                }
            ),
            x,
            y,
            w,
            h,
        ),

        fltk::table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

        fltk::table::TableContext::Cell => draw_data(
            &format!(
                "{}",
                cell_book2(
                    col,
                    row,
                    ind,
                    unsafe { base_ptr.as_ref().unwrap() },
                    unsafe { sys_ptr.as_ref().unwrap() }
                )
            ),
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
            None,
        ),

        _ => (),
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageReader::ChangeName => {
                    change_name_simple(ind, reader_base, book_system, genres, caretaker, app);
                    table2.redraw();
                }

                MessageReader::ChangeFamily => {
                    change_family_simple(ind, reader_base, book_system, genres, caretaker, app);
                    table2.redraw();
                }

                MessageReader::ChangeFather => {
                    change_father_simple(ind, reader_base, book_system, genres, caretaker, app);
                    table2.redraw();
                }

                MessageReader::ChangeAge => {
                    change_age_simple(ind, reader_base, book_system, genres, caretaker, app);
                    table2.redraw();
                }

                MessageReader::RemoveThis => {
                    remove_reader_simple(ind, reader_base, book_system, genres, caretaker);
                    table2.redraw();
                }
            }
            return;
        }

        if !wind.shown() {
            return;
        }

        (0..reader_base.readers.len()).for_each(|i| {
            if table2.is_selected(i as i32, 0)
                || table2.is_selected(i as i32, 1)
                || table2.is_selected(i as i32, 2)
                || table2.is_selected(i as i32, 3)
            {
                unsafe {
                    book_info_simple(
                        Some(
                            (**reader_base.readers.get_unchecked(ind))
                                .borrow()
                                .books
                                .get_unchecked(i)
                                .clone(),
                        ),
                        book_system,
                        app,
                    );
                }
                table2.unset_selection();
                return;
            }
        });
    }
}

/// Function that adds reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Add Reader",
        "First Name",
        "Second Names",
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
                        if empty_inp_reader(&reader) {
                            return;
                        }

                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(a) => unsafe {
                                match reader_base.add_reader(
                                    reader.get_unchecked(0).clone(),
                                    reader.get_unchecked(1).clone(),
                                    reader.get_unchecked(2).clone(),
                                    a,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(500, 500, "Successfully added");
                                        reader_base.save();
                                    }

                                    Err(_) => {
                                        alert(500, 500, "Reader already exists");
                                    }
                                }
                            },

                            Err(_) => {
                                alert(500, 500, "Age input error");
                                println!("{:?}", reader.last().unwrap().trim().parse::<u8>())
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that removes reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Remove Reader",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = rem_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        remove_reader_simple(rind, reader_base, book_system, genres, caretaker);
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_name(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

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

                        change_name_simple(rind, reader_base, book_system, genres, caretaker, app);
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's second name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_family(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change 2-nd Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

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

                        change_family_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's middle name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_father(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Middle Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

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

                        change_father_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's age.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_age(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Age",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

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

                        change_age_simple(rind, reader_base, book_system, genres, caretaker, app);
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that gives info about reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn reader_info(
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Find Reader",
        "First Name",
        "Second Name",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(x) => unsafe {
                                let find = (*reader_base).borrow().find_reader(
                                    reader.get_unchecked(0),
                                    reader.get_unchecked(1),
                                    reader.get_unchecked(2),
                                    x,
                                );

                                match find {
                                    None => alert(500, 500, "Reader isn't found"),

                                    Some(ind) => reader_info_simple(
                                        ind,
                                        &mut *(*reader_base).borrow_mut(),
                                        &mut *(*book_system).borrow_mut(),
                                        genres,
                                        caretaker,
                                        app,
                                    ),
                                }
                            },

                            Err(_) => {
                                alert(500, 500, "Age input error");
                                println!("{:?}", reader.last().unwrap().trim().parse::<u8>())
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}
