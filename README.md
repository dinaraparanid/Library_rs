**Author**: 
-----------------------
**dinaraparanid (Follow me on github: https://github.com/dinaraparanid)**

**Licence**: 
-----------------------
MIT or Apache-2.0

**Documentation**: 
----------------------

    public: https://docs.rs/binartree/1.1.2/booklibrs/
		
    private (whole project): go to librs/target/doc/booklibrs/all.html (click on it)
**About project**:
---------------------------
This is a project that aims to create a system for issuing books in libraries.
The entire project is written in the *Rust programming language* using the *fltk, chrono,* and *yaml_rust* crates.

**How to use**:
--------------------------

**1. Start**
----------------------------

If you a *windows user*, you need to just press on run.bat, and programme'll start working.

If you are *not windows user*, than you need to install 
*Rust compiler*: https://www.rust-lang.org/tools/install
*C/C++ MVSC compiler* (**NOT MinGW!!!**): https://visualstudio.microsoft.com/vs/features/cplusplus/
*Git*: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
*CMake*: https://cmake.org/install/

After that you need to use your console, go to project direction, write "*cargo run --release*" and programme'll start working.

**2. Tools**
--------------------------
First of all you will see a table. It contains all readers in an ascending order and their reading status (book / start date / finish date)
In main menu (on the left top corner) you will see 6 sub menus: "Readers", "Books", "Giveaway", "Restore", "Language", "Help".

**Reader menu**: contains all action that you can do with readers: Add, Remove, Change 1-st name, 2-nd name, mid. name, get information.
There are also 2 types of getting info: with books that reader is reading now and with all books that reader read 
(all existing books in library, if you delete book, it will not be shown.)  
All these action requires your input. But you can simply click on reader you need in main table and all needed action will be there 
(except adding reader, that's can't handle without input)

**Book menu**:
1) Add books (if book in library, it'll add new books, else it'll create new books in library)
2) Remove book. You can remove only one book in this action. Requires number of book.
3) Remove all specific books. It'll remove all books with same title, author and amount of pages.
4) Change title, author, amount of pages. I think it'll be obvious.
5) Change book's location. You'll change cabinet and shelf where book is located. I don't know anything about your library, so I can't
 control your input

6) Get type book's information. As for reader, you'll get a window with info and all actions for it.
7) Get current book's information. You'll get info window about current book. Also you can change location (cabinet and shelf) of book.
8) List of all books by authors. Window with all books sorted by authors. Authors sorted by ascending.
9) List of all books.

**Genres menu**:
1) List of all books by genres. Sort all books by genres.
2) Add genre / Remove genre. By default it'll be no genres, so, you need to add it. It can be anything, so you can use your imagination :)
3) Customize book genres. You'll get window with genres. You need to choose genre. If book has some genres, it'll be a check mark near genre

**Giveaway menu** is a menu for giving and returning books.
1) Give book. Requires input for reader and book. At the end you need to choose return date. Also you can do it in reader info 
(or just click on reader)
2) Get book from reader. Requires reader input. Also you can do it in reader info or click on reader.
3) Change return date. You can also click on return date in main table and change it.

**Restore menu**:
1) Restore previous data. Also you can use Ctrl-Z.
2) Restore next date. Also you can use Ctrl-Shift-Z.

In **language menu** you can choose language. My languages are Russian and English, so I don't add others, 
so I hope your english is not worse than mine :D

In **Help** menu you can read same things if you forgot something.

**Enjoy!**   

*Full version with all decompiled files (recommend to download from here):
 https://drive.google.com/drive/u/0/folders/1mOTWStFbS_NS7iYs0uxS6MlT9jv8qZRf*


