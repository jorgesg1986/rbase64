extern crate gtk;
extern crate gio;
extern crate glib;
extern crate base64;

use gtk::prelude::*;
use glib::GString;
use gio::prelude::*;
use std::env::args;

fn build_ui(application: &gtk::Application) {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    // First we get the file content.
    let glade_src = include_str!("rbase64.glade");
    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").unwrap();
    window.set_application(Some(application));

    let encode_button: gtk::Button = builder.get_object("Encode").unwrap();
    let decode_button: gtk::Button = builder.get_object("Decode").unwrap();
    let input_field: gtk::Entry = builder.get_object("Input").unwrap();
    let output_field: gtk::TextView = builder.get_object("Output").unwrap();

    let input_field_clone = input_field.clone();
    let output_field_clone = output_field.clone();
    encode_button.connect_clicked(move |_| {
        let input: GString = input_field_clone
                        .get_text()
                        .expect("Please, input some text to encode.");

        let output = base64::encode(input.as_str());

        output_field_clone
            .get_buffer()
            .expect("Couldn't get text buffer.")
            .set_text(output.as_str());

    });

    decode_button.connect_clicked(move |_| {
        let input: GString = input_field
                        .get_text()
                        .expect("Please, input some text to encode.");

        let output_vec = base64::decode(input.as_str()).expect("Error while decoding string.");

        let output_str = std::str::from_utf8(&output_vec).expect("Error converting to str");

        output_field
            .get_buffer()
            .expect("Couldn't get text buffer.")
            .set_text(output_str);

    });

    window.show_all();

}

fn main() {
    let application = gtk::Application::new(
        Some("com.jorgesg1986.rbase64"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
