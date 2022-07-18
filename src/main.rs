mod gui;

use gtk::prelude::*;
use gui::build_ui;

use find_food::{fresh_start, take_step};

fn main() {
    let gui = true;
    if gui {
        let application =
            gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
        application.connect_activate(build_ui);

        application.run();
    } else {
        // just for profiling/benchmarking
        let mut state = fresh_start();
        for _ in 0..10000 {
            take_step(&mut state)
        }
    }
}
