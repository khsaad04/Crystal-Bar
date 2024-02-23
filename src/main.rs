mod modules;
use modules::window::WindowModule;
use modules::workspaces::WorkspacesModule;
use modules::{clock::ClockModule, Module};

use gtk::gdk::Display;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::CssProvider;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tokio::runtime::Runtime;

const APP_ID: &str = "dev.khsaad04.bar";
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to exec tokio runtime"));

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let clock = ClockModule::default().into_widget();
    let workspaces = WorkspacesModule::default().into_widget();
    let window = WindowModule::default().into_widget();

    // Box

    let start_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    start_widgets.append(&workspaces);

    let center_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    center_widgets.append(&clock);

    let end_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    start_widgets.append(&window);

    let center_box = gtk::CenterBox::builder()
        .start_widget(&start_widgets)
        .center_widget(&center_widgets)
        .end_widget(&end_widgets)
        .build();

    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("window")
        .child(&center_box)
        .build();

    bar.set_layer(Layer::Overlay);
    bar.init_layer_shell();
    bar.auto_exclusive_zone_enable();

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        bar.set_anchor(anchor, state);
    }

    bar.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}
