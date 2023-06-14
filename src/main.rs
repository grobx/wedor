mod layer_shell;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4_layer_shell::Layer;
use gtk::glib::{ExitCode};
use gtk::gio::ApplicationFlags;
use sourceview5::Buffer;
use layer_shell::LayerShell;

fn main() -> ExitCode {
    // this is needed if you load a glade file that contains a GtkSourceView
    // sourceview5::View::static_type();

    // Create the app
    let app = Application::builder()
        .flags(ApplicationFlags::default())
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // Ensure layer shell is supported
        if !gtk4_layer_shell::is_supported() {
            panic!("layer shell protocol is not supported. Are you using a wlroot compositor?");
        }

        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1920)
            .default_height(1080)
            .title("Hello, World!")
            .build();

        // Layer shell settings
        window.init_layer_shell();
        window.set_layer(Layer::Background);
        // window.set_anchor(Edge::Left, true);
        // window.set_margin(Edge::Left, 42);
        // window.set_keyboard_mode(KeyboardMode::Exclusive);

        // Add the GtkSourceView
        let code = include_str!("main.rs");
        let gsv = sourceview5::View::builder()
            .buffer(&Buffer::builder().text(code).build())
            .can_focus(true)
            .hexpand_set(true)
            .vexpand_set(true)
            .show_line_numbers(true)
            .build();
        window.set_child(Some(&gsv));

        // Show the window.
        window.show();
    });

    // Run the app, then return the exit code
    app.run()
}
