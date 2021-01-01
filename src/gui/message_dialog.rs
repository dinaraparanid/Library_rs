use fltk::frame::Frame;
use fltk::window::SingleWindow;
use fltk::GroupExt;
use fltk::WidgetBase;
use fltk::WidgetExt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MessageDialog {
    window: Rc<RefCell<SingleWindow>>,
}

impl MessageDialog {
    #[inline]
    pub fn new(message: &str, location: i32) -> Self {
        let wind: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(700, 500, 500, 150, message)));
        let label: Rc<RefCell<Frame>> = Rc::new(RefCell::new(WidgetBase::new(
            location, 50, 300, 30, message,
        )));
        WidgetExt::set_label_size(&mut *(*label).borrow_mut(), 18);
        GroupExt::end(&*(*wind).borrow());

        MessageDialog { window: wind }
    }

    #[inline]
    pub fn show(&self) {
        (*self.window).borrow_mut().show_with_env_args()
    }
}
