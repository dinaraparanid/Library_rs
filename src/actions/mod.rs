extern crate fltk;

use crate::Lang;

use fltk::{
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::SingleWindow,
};

pub mod book;
pub mod genres;
pub mod giveaway;
pub mod read;
pub mod tables;

/// Function that makes almost
/// all manipulations with books
/// (except adding new book)

#[inline]
pub fn help(lang: Lang) {
    let mut wind = SingleWindow::new(
        400,
        200,
        900,
        700,
        match lang {
            Lang::English => "Help",
            Lang::Russian => "Помощь",
        },
    );

    let mut text = TextDisplay::new(20, 20, 880, 680, "");

    let mut tbe = TextBuffer::default();
    tbe.set_text("    Author: dinaraparanid (Follow me on github: https://github.com/dinaraparanid)\n
    Licence: MIT or Apache-2.0\n
    Documentation:\n
        public: https://docs.rs/binartree/1.1.9/booklibrs/\n
        private (whole project): go to librs/target/doc/booklibrs/all.html (click on it)\n\n
    About project:\n
    This is a project that aims to create a system for issuing books in libraries.\n
    The entire project is written in the Rust programming language using the fltk, chrono, and yaml_rust crates.\n\n
    How to use:\n\n
    1. Start\n\n
    If you a windows user, you need to just press on run.bat, and programme'll start working.\n\n
    If you are not windows user, than you need to install\n
    Rust compiler: https://www.rust-lang.org/tools/install\n
    C/C++ MVSC compiler (NOT MinGW!!!): https://visualstudio.microsoft.com/vs/features/cplusplus/\n
    Git: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git\n
    CMake: https://cmake.org/install/\n\n
    After that you need to use your console, go to project direction, write 'cargo run --release' and programme'll start working.\n\n
    2. Tools\n\n
    First of all you will see a table. It contains all readers in an ascending order and their reading status \
    (book / start date / finish date)\n
    In main menu (on the left top corner) you will see 6 sub menus: Readers, Books, Giveaway, Restore, Language, Help.\n\n
    Reader menu:\n contains all action that you can do with readers: Add, Remove, Change 1-st name, 2-nd name, mid. name, get information.\n
    There are also 2 types of getting info: with books that reader is reading now and with all books that reader read\n  
    All these action requires your input. But you can simply click on reader you need in main table and all needed action will be there
    (except adding reader, that's requires input for new reader)\n\n
    Book menu:\n
    1) Add books (if book in library, it'll add new books, else it'll create new books in library)\n
    2) Remove book. You can remove only one book in this action. Requires number of book.\n
    3) Remove all specific books. It'll remove all books with same title, author and amount of pages\n.
    4) Change title, author, amount of pages. I think it'll be obvious.\n
    5) Change book's location. You'll change cabinet and shelf where book is located. I don't know anything about your library,
    so I can't control your input\n
    6) Get type book's information. As for reader, you'll get a window with info and all actions for it.\n
    7) Get current book's information. You'll get info window about current book. Also you can change location (cabinet and shelf) of book\n
    8) List of all books by authors. Window with all books sorted by authors. Authors sorted by ascending.\n
    9) List of all books.\n\n
    Genres menu:\n
    1) List of all books by genres. Sort all books by genres.\n
    2) Add genre / Remove genre. By default it'll be no genres, so, you need to add it. It can be anything, so you can use your imagination\
     :)\n
    3) Customize book genres. You'll get window with genres. You need to choose genre. If book has some genres, it'll be a check mark near \
     genre\n\n
    Giveaway menu is a menu for giving and returning books.\n
    1) Give book. Requires input for reader and book. At the end you need to choose return date. Also you can do it in reader info
     (or just click on reader)\n
    2) Get book from reader. Requires reader input. Also you can do it in reader info or click on reader.\n
    3) Change return date. You can also click on return date in main table and change it.\n\n
    Restore menu:\n
    1) Restore previous data. Also you can use Ctrl-Z.\n
    2) Restore next date. Also you can use Ctrl-Shift-Z.\n\n
    In language menu you can choose language. My languages are Russian and English, so I don't add others,
    so I hope your english is not worse than mine :D\n\n
    In Help menu you can read same things if you forgot something.\n\n
    Full version with all decompiled files (recommend to download from here):
    https://drive.google.com/drive/u/0/folders/1mOTWStFbS_NS7iYs0uxS6MlT9jv8qZRf\n
    Enjoy!\n");

    let mut tbr = TextBuffer::default();
    tbr.set_text("    Автор: dinaraparanid (github: https://github.com/dinaraparanid)\n
    Лицензия: MIT или Apache-2.0\n
    Документация:\n
        публичные модули: https://docs.rs/binartree/1.1.9/booklibrs/\n
        внутренние модули: перейдите в librs/target/doc/booklibrs/all.html (нажать)\n\n
    О проекте:\n
    Это проект, цель которого создать систему учёта книг в библиотеке.\n
    Весь проект Целиком и полностью написан на ЯП Rust с использованием крейтов fltk, chrono и yaml_rust.\n\n
    Как пользоваться:\n\n
    1. Начало\n\n
    Если вы используете ОС Windows, то достаточно запустить run.bat и программа начнёт работу.\n
    Если же вы юспользуете другие ОС, то придётся скачать следующее ПО:\n
    Компилятор Rust: https://www.rust-lang.org/tools/install\n
    Компилятор C/C++ от Microsoft (MVSC) (НЕ MinGW!!!): https://visualstudio.microsoft.com/vs/features/cplusplus/\n
    Git: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git\n
    CMake: https://cmake.org/install/\n\n
    После этого нужно воспоьзоваться консолью, перейти в папку с проектом и прописать 'cargo run --release'
     и программа начнёт работу\n\n
    2. Инструменты\n\n
    Во первых, вы увидете таблицу со всеми читателями, отсортированнными в лексикографической последовательности \
    (по возратанию имени) и их статус чтения (книга / дата выдачи / дата возврата).\n
    В главном меню (в левом верхнем углу) расположенно 6 подменю: Читатели, Книги, Выдача, Выдача, Откат, Язык и Помощь\
    .\n\n
    Меню Читатели:\n 
    Содержит все действия, выполнимые с читателями: Добавить, Удалить, Изменить имя, фамилию, отчество,
     получить информацию.\n
    Получить информацию о читателях можно в двух видах: с прочитанными книгами, и с читаемыми сейчас.\n
    Во всех этих действиях необходим ввод. Но вы можете кликнуть на читателя в главной таблице и все необходимые 
    действия будут там.(кроме добавления читателя - необходим ввод параметров читателя)\n\n
    Меню Книги:\n
    1) Добавить книги (если книга есть в библиотеке, добавит к кол-ву, иначе создаст новую книгу в библиотеке)\n
    2) Удалить книгу. Вы можете удалить только одну книгу по её номеру.\n
    3) Удалить все схожие книги. Удалит все книги с одинаковыми названием, автором и кол-вом страниц\n.
    4) Изменить название, автора, кол-во страниц.\n
    5) Изменить расположение книги. Изменяет номер шкафа и полки. Я не знаю ничего о вашей библиотеке,
    так что не могу контролировать ввод. Это на вашу совесть.\n
    6) Получить информацию о всех схожих книгах. Так же, как и для читателя, выведется окно с информацией\
     и доступными действиями.\n
    7) Получить информацию о конкретной книгe. Выводится окно с информацией о конкретной книге. \
     Так же вы можете изменить расположение (шкаф и полку)\n
    8) Список всех книг по авторам. Окно со всеми читателями, отсортированными по авторам. \
     Авторы отсортированы по возрастанию (лексикографически).\n
    9) Все книги. Выводит все книги.\n\n
    Меню Жанры:\n
    1) Список всех книг по жанрам. Сортирует все книги по жанрам.\n
    2) Добавить / удалить жанр. По-умолчанию жанров нет, так что нужно самому добавлять их. \
     Ограничений нет, так что используйте всю свою фантазию :)\n
    3) Изменить жанры книги. Выводится окно с жанрами. нужно отметить галочкой все нужные жанры. \
     Если жанров нет, то их не будет. Если у книги уже есть жанры, рядом будет галочка.\n\n
    Меню Выдачи книг:\n
    1) Выдать книгу. Вводятся параметры читателя и книги. В конце нужно выбрать дату возврата.\
    Так же книгу можно выдать в информации о читателе (или кликнув на нужного читателя в главной таблице)\n
    2) Вернуть книгу. Вводятся параметры читателя.\
     Так же книгу можно вернуть в информации о читателе (или кликнув на нужного читателя в главной таблице)\n
    3) Изменить дату выдачи. Можно кликнуть на дату выдачи в главной таблице и сделать тоже самое.\n\n
    Меню Отката (Восстановления):\n
    1) Откатить изменения назад. Так же можно нажать Ctrl-Z.\n
    2) Откатить изменения вперед. Так же можно нажать Ctrl-Shift-Z.\n\n
    В меню Язык можно выбрать язык (русский или английский).\n
    В меню Помощь можно прочитать всё, что вы прочли, если что-то забыли.\n\n
    Полная версия со всеми скомпилированными файлами (СКАЧИВАТЬ ОТСЮДА):
    https://drive.google.com/drive/u/0/folders/1mOTWStFbS_NS7iYs0uxS6MlT9jv8qZRf\n
    Надеюсь, вам понравится!\n");

    text.set_buffer(match lang {
        Lang::English => tbe,
        Lang::Russian => tbr,
    });

    wind.end();
    wind.show();
}
