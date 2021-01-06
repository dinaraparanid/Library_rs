/// Tests for Date, Book and TheBook
/// ---------------------------------------------------------------

/// Date tests

mod date_tests {
    use crate::book::Date;

    #[test]
    fn date_test() {
        let mut date = Date::new(0, 0, 0);
        assert!(date.is_none());

        date = Date::new(1, 13, 2020);
        assert!(date.is_none());

        date = Date::new(29, 2, 2019);
        assert!(date.is_none());

        date = Date::new(29, 2, 2020);
        assert!(date.is_some());
    }
}

/// Book tests

mod book_tests {
    use crate::book::{Book, BookInterface, Date};
    use crate::reader::Reader;
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn book_new_changing_test() {
        let book = Rc::new(RefCell::new(Book::new(
            "Title1".to_string(),
            "Author1".to_string(),
            200,
        )));

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: false, readers.yaml: [] }",
                   format!("{:?}", (*book).borrow()));

        (*book)
            .borrow_mut()
            .change_title("Title2".to_string())
            .change_author("Author2".to_string())
            .change_pages(300);

        assert_eq!("Book { title: \"Title2\", author: \"Author2\", pages: 300, is using: false, readers.yaml: [] }",
                   format!("{:?}", (*book).borrow()));
    }

    #[test]
    fn book_readers_manipulation_test() {
        let book = Rc::new(RefCell::new(Book::new(
            "Title1".to_string(),
            "Author1".to_string(),
            200,
        )));

        let reader1 = Rc::new(RefCell::new(Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        )));

        let reader2 = Rc::new(RefCell::new(Reader::new(
            "Another Name".to_string(),
            "Another Family".to_string(),
            "Another Father".to_string(),
            60,
        )));

        assert!((*reader1).borrow_mut().start_reading(&book).is_ok());

        assert!((*book)
            .borrow_mut()
            .start_reading(&reader1, Date::new(1, 1, 1).unwrap())
            .is_ok());

        assert!((*reader2).borrow_mut().start_reading(&book).is_err());

        assert!((*book)
            .borrow_mut()
            .start_reading(&reader2, Date::new(1, 1, 1).unwrap())
            .is_err());

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: true, readers.yaml: [\"Name Family Father 50\"] }",
                   format!("{:?}", (*book).borrow()));

        assert_eq!((*book).borrow_mut().finish_reading().err().unwrap(), 1);

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: false, readers.yaml: [\"Name Family Father 50\"] }",
                   format!("{:?}", (*book).borrow()));

        assert!((*reader2).borrow_mut().start_reading(&book).is_ok());

        assert!((*book)
            .borrow_mut()
            .start_reading(&reader2, Date::new(1, 1, 1).unwrap())
            .is_ok());

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: true, readers.yaml: [\"Name Family Father 50\", \"Another Name Another Family Another Father 60\"] }",
                   format!("{:?}", (*book).borrow()));

        assert_eq!((*book).borrow().find_reader(&reader1), 0);
        assert_eq!((*book).borrow().find_reader(&reader2), 1);

        (*reader1)
            .borrow_mut()
            .change_name("Michael".to_string())
            .unwrap()
            .change_family("Jackson".to_string())
            .unwrap()
            .change_father("Joseph".to_string())
            .unwrap()
            .change_age(60);

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: true, readers.yaml: [\"Michael Jackson Joseph 60\", \"Another Name Another Family Another Father 60\"] }",
                   format!("{:?}", (*book).borrow()));

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: true, readers.yaml: [\"Michael Jackson Joseph 60\"] }",
                   format!("{:?}", (*book).borrow_mut().remove_reader(&(*(*reader2).borrow()) as *mut Reader)));

        let mut x = (*book).borrow_mut();

        assert_eq!("Book { title: \"Title1\", author: \"Author1\", pages: 200, is using: true, readers.yaml: [] }",
        format!("{:?}", x.remove_all_readers()));
    }
}

/// TheBook tests

mod the_book_tests {
    use crate::book::{BookInterface, Date, TheBook};
    use crate::reader::Reader;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_book_new_add_remove_test() {
        let mut the_book = TheBook::new("Title".to_string(), "Author".to_string(), 200);
        assert_eq!(
            "The Book { title: \"Title\", author: \"Author\", pages: 200, books.yaml: [\"Book { title: \\\"Title\\\", author: \\\"Author\\\", pages: 200, is using: false, readers.yaml: [] }\"] }",
            format!("{:?}", the_book)
        );

        let reader = Rc::new(RefCell::new(Reader::new(
            "Name".to_string(),
            "Family".to_string(),
            "Father".to_string(),
            50,
        )));

        for _ in 0..50 {
            let ind = the_book.get_unused();

            assert!((*reader)
                .borrow_mut()
                .start_reading(the_book.books.get(ind).unwrap())
                .is_ok());

            assert!((**the_book.books.get(ind).unwrap())
                .borrow_mut()
                .start_reading(&reader, Date::new(1, 1, 1).unwrap())
                .is_ok());

            the_book.add_book();
        }

        assert_eq!((*reader).borrow().books.len(), 50);
        assert_eq!(the_book.books.len(), 51);
        assert_eq!(the_book.get_unused(), 50);

        assert!(the_book.remove_book(50).is_ok());
        assert_eq!((*reader).borrow().books.len(), 50);
        assert_eq!(the_book.books.len(), 50);

        assert!(the_book.remove_book(49).is_ok());
        assert_eq!(the_book.books.len(), 49);
        assert_eq!((*reader).borrow().books.len(), 49);

        the_book.remove_all_books();

        assert_eq!(
            "The Book { title: \"Title\", author: \"Author\", pages: 200, books.yaml: [] }",
            format!("{:?}", the_book)
        );

        assert_eq!(
            "Reader { name: \"Name\", family: \"Family\", father: \"Father\", age: 50, books.yaml: [] }",
            format!("{:?}", (*reader).borrow())
        );
    }

    #[test]
    fn the_book_changing_test() {
        let mut the_book = TheBook::new("Title".to_string(), "Author".to_string(), 200);
        let reader = Rc::new(RefCell::new(Reader::new(
            "Michael".to_string(),
            "Jackson".to_string(),
            "Joseph".to_string(),
            60,
        )));

        assert!((*reader)
            .borrow_mut()
            .start_reading(the_book.books.first().unwrap())
            .is_ok());

        assert_eq!("Reader { name: \"Michael\", family: \"Jackson\", father: \"Joseph\", age: 60, books.yaml: [\"Title Author 200\"] }",
                   format!("{:?}", *(*reader).borrow()));

        the_book
            .change_title("Harry Potter".to_string())
            .change_author("Joanne Rowling".to_string())
            .change_pages(400);

        assert_eq!("Reader { name: \"Michael\", family: \"Jackson\", father: \"Joseph\", age: 60, books.yaml: [\"Harry Potter Joanne Rowling 400\"] }",
                   format!("{:?}", *(*reader).borrow()));
    }
}
