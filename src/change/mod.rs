pub mod input1;
pub mod input2;
pub mod input3;
pub mod input4;

extern crate fltk;
use fltk::{prelude::*, window::SingleWindow};
use std::{cell::RefCell, rc::Rc};

/// Trait for change messages.

pub trait Inputable {
    /// Gets menu's window

    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>>;

    /// Returns input by vector of strings if everything is ok
    /// else it returns error

    fn set_input(&mut self) -> Result<Vec<String>, ()>;

    /// Checks if window is shown

    #[inline]
    fn shown(&self) -> bool {
        (*self.get_wind()).borrow().shown()
    }

    /// Hides window

    #[inline]
    fn hide(&self) {
        (*self.get_wind()).borrow_mut().hide()
    }

    /// Shows window

    #[inline]
    fn show(&self) {
        (*self.get_wind()).borrow_mut().show_with_env_args()
    }
}
