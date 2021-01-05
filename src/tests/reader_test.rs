/// Test for Reader and ReaderBase
/// ------------------------------------------------------------

/// Reader tests

mod reader_tests {
    use crate::book::{Book, BookInterface, Date};
    use crate::reader::Reader;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn reader_new_eq_test() {
        let reader1 = Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        );

        let reader2 = Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        );

        let reader3 = Reader::new(
            "Another Name".to_string(),
            "Another Family".to_string(),
            "Another Father".to_string(),
            50,
        );

        assert_eq!(
            "Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", reader1)
        );

        assert_eq!(
            "Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", reader2)
        );

        assert_eq!(
            "Reader { name: \"Another Name\", family: \"Another Family\", father: \"Another Father\", age: 50, books.yaml: [] }",
            format!("{:?}", reader3)
        );

        assert_eq!(reader1, reader2);
        assert_eq!(reader2, reader1);
        assert_ne!(reader1, reader3);
        assert_ne!(reader3, reader1);
        assert_ne!(reader2, reader3);
        assert_ne!(reader3, reader2);
    }

    #[test]
    fn reader_change_test() {
        let mut reader = Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        );

        assert_eq!(reader.change_name("".to_string()), Err(0));

        assert_eq!(
            "Reader { name: \"Name1\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", reader.change_name("Name1".to_string()).unwrap())
        );

        assert_eq!(reader.change_family("".to_string()), Err(0));

        assert_eq!(
            "Reader { name: \"Name1\", family: \"Family1\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", reader.change_family("Family1".to_string()).unwrap())
        );

        assert_eq!(reader.change_father("".to_string()), Err(0));

        assert_eq!(
            "Reader { name: \"Name1\", family: \"Family1\", father: \"Father1\", age: 50, books.yaml: [] }",
            format!("{:?}", reader.change_father("Father1".to_string()).unwrap())
        );

        assert_eq!(
            "Reader { name: \"Name1\", family: \"Family1\", father: \"Father1\", age: 60, books.yaml: [] }",
            format!("{:?}", reader.change_age(60))
        );
    }

    #[test]
    fn reader_books_manipulations_test() {
        let reader = Rc::new(RefCell::new(Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        )));

        let book1 = Rc::new(RefCell::new(Book::new(
            "Title1".to_string(),
            "Author1".to_string(),
            50,
        )));

        let book2 = Rc::new(RefCell::new(Book::new(
            "Title2".to_string(),
            "Author2".to_string(),
            60,
        )));

        let book3 = Rc::new(RefCell::new(Book::new(
            "Title3".to_string(),
            "Author3".to_string(),
            70,
        )));

        assert_eq!("Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [\"Title1 Author1 50\", \"Title2 Author2 60\", \"Title3 Author3 70\"] }",
                       format!("{:?}", 
                               (*reader)
                                   .borrow_mut()
                                   .start_reading(&book1)
                                   .unwrap()
                                   .start_reading(&book2)
                                   .unwrap()
                                   .start_reading(&book3)
                                   .unwrap()));

        assert!((*book1)
            .borrow_mut()
            .start_reading(&reader, Date::new(1, 1, 1).unwrap())
            .is_ok());

        assert!((*book2)
            .borrow_mut()
            .start_reading(&reader, Date::new(1, 1, 1).unwrap())
            .is_ok());

        assert!((*book3)
            .borrow_mut()
            .start_reading(&reader, Date::new(1, 1, 1).unwrap())
            .is_ok());

        (*book1)
            .borrow_mut()
            .change_title("KEK".to_string())
            .change_author("LOL".to_string())
            .change_pages(100);

        (*book2)
            .borrow_mut()
            .change_title("LOL".to_string())
            .change_author("KEK".to_string())
            .change_pages(200);

        (*book3)
            .borrow_mut()
            .change_title("KOK".to_string())
            .change_author("LEL".to_string())
            .change_pages(1000);

        assert_eq!("Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [\"KEK LOL 100\", \"LOL KEK 200\", \"KOK LEL 1000\"] }",
                       format!("{:?}", *(*reader).borrow())
        );

        (*reader)
            .borrow_mut()
            .remove_book(&(*(*book1).borrow()) as *const Book);

        assert_eq!("Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [\"LOL KEK 200\", \"KOK LEL 1000\"] }",
                   format!("{:?}", *(*reader).borrow())
        );

        assert_eq!(
            "Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", (*reader).borrow_mut().remove_all_books())
        );
    }
}

/// ReaderBase tests

mod reader_base_tests {
    use crate::book::{Book, Date};
    use crate::reader::ReaderBase;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn reader_base_new_find_add_remove_test() {
        let mut reader_base = ReaderBase::new();
        assert_eq!(
            "Reader Base { readers.yaml: [] }",
            format!("{:?}", reader_base)
        );

        assert_eq!("Reader Base { readers.yaml: [\"Reader { name: \\\"Name1\\\", family: \\\"Family1\\\", father: \\\"Father1\\\", age: 50, books.yaml: [] }\", \"Reader { name: \\\"Name2\\\", family: \\\"Family2\\\", father: \\\"Father2\\\", age: 60, books.yaml: [] }\"] }",
        format!("{:?}", 
                reader_base
                    .add_reader(
                        "Name1".to_string(), 
                        "Family1".to_string(), 
                        "Father1".to_string(), 
                        50, )
                    .unwrap()
                    .add_reader(
                        "Name2".to_string(), 
                        "Family2".to_string(), 
                        "Father2".to_string(), 
                        60, )
                    .unwrap()));

        assert_eq!(
            reader_base.find_reader(
                &"Name1".to_string(),
                &"Family1".to_string(),
                &"Father1".to_string(),
                50
            ),
            0
        );

        assert_eq!(
            reader_base.find_reader(
                &"Name2".to_string(),
                &"Family2".to_string(),
                &"Father2".to_string(),
                60
            ),
            1
        );

        assert_eq!(
            reader_base.find_reader(
                &"Error".to_string(),
                &"Error".to_string(),
                &"Error".to_string(),
                50
            ),
            reader_base.readers.len()
        );

        let book1 = Rc::new(RefCell::new(Book::new(
            "Title1".to_string(),
            "Author1".to_string(),
            50,
        )));

        let book2 = Rc::new(RefCell::new(Book::new(
            "Title2".to_string(),
            "Author2".to_string(),
            60,
        )));

        let book3 = Rc::new(RefCell::new(Book::new(
            "Title3".to_string(),
            "Author3".to_string(),
            70,
        )));

        (*(*reader_base.readers.first_mut().unwrap()))
            .borrow_mut()
            .start_reading(&book1)
            .unwrap()
            .start_reading(&book2)
            .unwrap()
            .start_reading(&book3)
            .unwrap();

        assert!((*book1)
            .borrow_mut()
            .start_reading(
                &(*reader_base.readers.first_mut().unwrap()),
                Date::new(1, 1, 1).unwrap(),
            )
            .is_ok());

        assert!((*book2)
            .borrow_mut()
            .start_reading(
                &(*reader_base.readers.first_mut().unwrap()),
                Date::new(1, 1, 1).unwrap(),
            )
            .is_ok());

        assert!((*book3)
            .borrow_mut()
            .start_reading(
                &(*reader_base.readers.first_mut().unwrap()),
                Date::new(1, 1, 1).unwrap(),
            )
            .is_ok());

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 50, is using: true, readers.yaml: [\"Name1 Family1 Father1 50\"] }",
                   format!("{:?}", *(*book1).borrow()));

        assert_eq!("Book { title: \"Title2\", author: \"Author2\", pages: 60, is using: true, readers.yaml: [\"Name1 Family1 Father1 50\"] }",
                   format!("{:?}", *(*book2).borrow()));

        assert_eq!("Book { title: \"Title3\", author: \"Author3\", pages: 70, is using: true, readers.yaml: [\"Name1 Family1 Father1 50\"] }",
                   format!("{:?}", *(*book3).borrow()));

        assert_eq!("Reader Base { readers.yaml: [\"Reader { name: \\\"Name2\\\", family: \\\"Family2\\\", father: \\\"Father2\\\", age: 60, books.yaml: [] }\"] }",
                   format!("{:?}",
                           reader_base
                               .remove_reader(
                                   &"Name1".to_string(),
                                   &"Family1".to_string(),
                                   &"Father1".to_string(),
                                   50, )
                               .unwrap()));

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 50, is using: true, readers.yaml: [] }",
                   format!("{:?}", *(*book1).borrow()));

        assert_eq!("Book { title: \"Title2\", author: \"Author2\", pages: 60, is using: true, readers.yaml: [] }",
                   format!("{:?}", *(*book2).borrow()));

        assert_eq!("Book { title: \"Title3\", author: \"Author3\", pages: 70, is using: true, readers.yaml: [] }",
                   format!("{:?}", *(*book3).borrow()));
    }

    #[test]
    fn reader_base_changing_test() {
        let mut reader_base = ReaderBase::new();

        assert_eq!("Reader Base { readers.yaml: [\"Reader { name: \\\"Name1\\\", family: \\\"Family1\\\", father: \\\"Father1\\\", age: 10, books.yaml: [] }\"] }",
            format!("{:?}", 
                    reader_base
                        .add_reader(
                            "Name1".to_string(),
                            "Family1".to_string(),
                            "Father1".to_string(),
                            10,
                        )
                        .unwrap()));

        assert_eq!("Reader Base { readers.yaml: [\"Reader { name: \\\"Michael\\\", family: \\\"Jackson\\\", father: \\\"Joseph\\\", age: 60, books.yaml: [] }\"] }",
                   format!("{:?}",
                           reader_base
                               .change_name(
                                   &"Name1".to_string(),
                                   &"Family1".to_string(),
                                   &"Father1".to_string(),
                                   10,
                                   "Michael".to_string()
                               )
                               .unwrap()
                               .change_family(
                                   &"Michael".to_string(),
                                   &"Family1".to_string(),
                                   &"Father1".to_string(),
                                   10,
                                   "Jackson".to_string()
                               )
                               .unwrap()
                               .change_father(
                                   &"Michael".to_string(),
                                   &"Jackson".to_string(),
                                   &"Father1".to_string(),
                                   10,
                                   "Joseph".to_string()
                               )
                               .unwrap()
                               .change_age(
                                   &"Michael".to_string(),
                                   &"Jackson".to_string(),
                                   &"Joseph".to_string(),
                                   10,
                                   "60".to_string()
                               )
                               .unwrap()));

        assert_eq!(
            reader_base
                .change_age(
                    &"Michael".to_string(),
                    &"Jackson".to_string(),
                    &"Joseph".to_string(),
                    60,
                    "aba".to_string()
                )
                .err()
                .unwrap(),
            0
        );
    }
}
