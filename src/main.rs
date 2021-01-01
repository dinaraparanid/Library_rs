use fltk::{app, button::*, frame::*, window::*};
use librs::gui::change_menu::*;

fn main() {
	let app = app::App::default();

	let mut str1 = 0;
	let mut str2 = 0;
	let mut str3 = 0;

	let test = Input3::new(
		"TITLE", "WHAT1", "WHAT2", "WHAT3", &mut str1, &mut str2, &mut str3,
	);
	test.show();

	app.run().unwrap();
}
