use crate::book::{Book, ResultSelf};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::{Rc, Weak};

pub(crate) struct Reader {
    pub(crate) name: String,
    pub(crate) family: String,
    pub(crate) father: String,
    pub(crate) age: u8,
    pub(crate) books: Vec<Weak<RefCell<Book>>>,
}

pub struct ReaderBase {
    pub(crate) readers: Vec<Rc<RefCell<Reader>>>,
}

impl Drop for Reader {
    #[inline]
    fn drop(&mut self) {
        println!(
            "Readers {} {} {} is deleted",
            self.name, self.family, self.father
        );
    }
}

impl Debug for Reader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Reader")
            .field("name", &self.name)
            .field("family", &self.family)
            .field("father", &self.father)
            .field("age", &self.age)
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| {
                        (*(*x).upgrade().unwrap()).borrow().title.clone()
                            + " "
                            + (*(*x).upgrade().unwrap()).borrow().author.clone().as_str()
                            + " "
                            + format!("{}", (*(*x).upgrade().unwrap()).borrow().pages).as_str()
                    })
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl PartialEq for Reader {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.family == other.family
            && self.father == other.father
            && self.age == other.age
    }
}

impl Eq for Reader {}

impl Reader {
    #[inline]
    pub fn new(new_name: String, new_family: String, new_father: String, new_age: u8) -> Self {
        Reader {
            name: new_name,
            family: new_family,
            father: new_father,
            age: new_age,
            books: vec![],
        }
    }

    #[inline]
    pub fn find_book(&self, book: &Rc<RefCell<Book>>) -> usize {
        for i in 0..self.books.len() {
            let book_ptr;

            unsafe {
                book_ptr = self.books.get_unchecked(i).upgrade().unwrap().as_ptr();
            }

            if book_ptr.is_null() {
                panic!("nullptr in Reader find_reader");
            }

            if book_ptr == book.as_ptr() {
                return i;
            }
        }
        self.books.len()
    }

    #[inline]
    pub fn start_reading(&mut self, book: &Rc<RefCell<Book>>) -> ResultSelf<Self> {
        return if (**book).borrow().is_using {
            Err(0)
        } else {
            self.books.push(Rc::downgrade(&book));
            Ok(self)
        };
    }

    pub fn remove_book(&mut self, book: *const Book) -> &mut Self {
        self.books = self
            .books
            .clone()
            .into_iter()
            .filter(|x| (*(*x).upgrade().unwrap()).as_ptr() as *const Book != book)
            .collect();

        self
    }

    #[inline]
    pub fn remove_all_books(&mut self) -> &mut Self {
        while !self.books.is_empty() {
            (*self.books.last().unwrap().upgrade().unwrap())
                .borrow_mut()
                .remove_reader(self as *const Reader);
            self.books.pop();
        }
        self
    }

    #[inline]
    pub fn change_name(&mut self, new_name: String) -> ResultSelf<Self> {
        return if new_name.is_empty() {
            Err(0)
        } else {
            self.name = new_name;
            Ok(self)
        };
    }

    #[inline]
    pub fn change_family(&mut self, new_family: String) -> ResultSelf<Self> {
        return if new_family.is_empty() {
            Err(0)
        } else {
            self.family = new_family;
            Ok(self)
        };
    }

    #[inline]
    pub fn change_father(&mut self, new_father: String) -> ResultSelf<Self> {
        return if new_father.is_empty() {
            Err(0)
        } else {
            self.father = new_father;
            Ok(self)
        };
    }

    #[inline]
    pub fn change_age(&mut self, new_age: u8) -> &mut Self {
        self.age = new_age;
        self
    }
}

impl Debug for ReaderBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Reader Base")
            .field(
                "readers",
                &self
                    .readers
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl ReaderBase {
    #[inline]
    pub const fn new() -> Self {
        ReaderBase { readers: vec![] }
    }

    pub fn find_reader(&self, name: &String, family: &String, father: &String, age: u8) -> usize {
        for i in 0..self.readers.len() {
            unsafe {
                if (**self.readers.get_unchecked(i)).borrow().name == *name
                    && (**self.readers.get_unchecked(i)).borrow().family == *family
                    && (**self.readers.get_unchecked(i)).borrow().father == *father
                    && (**self.readers.get_unchecked(i)).borrow().age == age
                {
                    return i;
                }
            }
        }
        self.readers.len()
    }

    #[inline]
    pub fn add_reader(
        &mut self,
        name: String,
        family: String,
        father: String,
        age: u8,
    ) -> ResultSelf<Self> {
        return if !self.readers.is_empty()
            && self.find_reader(&name, &family, &father, age) < self.readers.len()
        {
            Err(0)
        } else {
            self.readers.push(Rc::new(RefCell::new(Reader::new(
                name, family, father, age,
            ))));
            Ok(self)
        };
    }

    #[inline]
    pub fn remove_reader(
        &mut self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
    ) -> ResultSelf<Self> {
        let find = self.find_reader(name, family, father, age);

        return if find == self.readers.len() {
            Err(0)
        } else {
            unsafe {
                (**self.readers.get_unchecked_mut(find))
                    .borrow_mut()
                    .remove_all_books();
                self.readers.remove(find);
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_name(
        &mut self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
        new_name: String,
    ) -> ResultSelf<Self> {
        let find = self.find_reader(name, family, father, age);

        return if find == self.readers.len() {
            Err(0) // not found
        } else {
            if self.find_reader(&new_name, family, father, age) < self.readers.len() {
                Err(1) // already exists
            } else {
                unsafe {
                    if let Err(_) = (**self.readers.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_name(new_name)
                    {
                        return Err(2); // empty name
                    }
                }
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_family(
        &mut self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
        new_family: String,
    ) -> ResultSelf<Self> {
        let find = self.find_reader(name, family, father, age);

        return if find == self.readers.len() {
            Err(0) // not found
        } else {
            if self.find_reader(name, &new_family, father, age) < self.readers.len() {
                Err(1) // already exists
            } else {
                unsafe {
                    if let Err(_) = (**self.readers.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_family(new_family)
                    {
                        return Err(2); // empty family
                    }
                }
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_father(
        &mut self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
        new_father: String,
    ) -> ResultSelf<Self> {
        let find = self.find_reader(name, family, father, age);

        return if find == self.readers.len() {
            Err(0) // not found
        } else {
            if self.find_reader(name, family, &new_father, age) < self.readers.len() {
                Err(1) // already exists
            } else {
                unsafe {
                    if let Err(_) = (**self.readers.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_father(new_father)
                    {
                        return Err(2); // empty father
                    }
                }
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_age(
        &mut self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
        new_age: String,
    ) -> ResultSelf<Self> {
        let new_age_num;

        match new_age.trim().parse::<u8>() {
            Ok(x) => new_age_num = x,
            Err(_) => return Err(0), // parse error
        }

        let find = self.find_reader(name, family, father, age);

        return if find == self.readers.len() {
            Err(1) // not found
        } else {
            if self.find_reader(name, family, father, new_age_num) < self.readers.len() {
                Err(2) // already exists
            } else {
                unsafe {
                    (**self.readers.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_age(new_age_num);
                }
                Ok(self)
            }
        };
    }
}
