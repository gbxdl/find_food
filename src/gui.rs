use find_food::SimulationState;
use find_food::{fresh_start, take_step};

use cairo;
use glib::timeout_add_local;
use gtk::prelude::*;
use std::time::Duration;

use glib::signal::SignalHandlerId;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(application: &gtk::Application) {
    // create window
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Evolution");
    // window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 630);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let drawing_area = Rc::new(RefCell::new(gtk::DrawingArea::new()));
    let signal_ids = Rc::new(RefCell::new(vec![]));

    // create buttons
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let start_button = gtk::Button::with_label("Start");
    let take_x_steps_button = gtk::Button::with_label("Take 1000 steps");
    let take_step_button = gtk::Button::with_label("Step");

    window.add(&vbox);

    let state = Rc::new(RefCell::new(fresh_start()));
    update_drawing_area(
        &state,
        &drawing_area.borrow_mut(),
        &mut signal_ids.borrow_mut(),
    );

    // button_box.pack_start(&life_button, true, true, 0);
    button_box.pack_start(&start_button, true, true, 0);
    button_box.pack_start(&take_x_steps_button, true, true, 0);
    button_box.pack_start(&take_step_button, true, true, 0);

    vbox.pack_start(&*drawing_area.borrow(), true, true, 0);

    let state_take_step = Rc::clone(&state);
    let drawing_area_take_step = Rc::clone(&drawing_area);
    let signal_ids_take_step = Rc::clone(&signal_ids);

    take_step_button.connect_clicked(move |_| {
        take_step(&mut state_take_step.borrow_mut());
        update_drawing_area(
            &state_take_step,
            &drawing_area_take_step.borrow_mut(),
            &mut signal_ids_take_step.borrow_mut(),
        );
        drawing_area_take_step
            .borrow()
            .queue_draw_area(0, 0, 600, 600); //refresh drawing area linux
    });

    let state_x_steps = Rc::clone(&state);
    let drawing_area_x_steps = Rc::clone(&drawing_area);
    let signal_ids_x_steps = Rc::clone(&signal_ids);

    take_x_steps_button.connect_clicked(move |_| {
        for _ in 0..1000 {
            take_step(&mut state_x_steps.borrow_mut());
        }
        update_drawing_area(
            &state_x_steps,
            &drawing_area_x_steps.borrow_mut(),
            &mut signal_ids_x_steps.borrow_mut(),
        );
        drawing_area_x_steps
            .borrow()
            .queue_draw_area(0, 0, 600, 600); //refresh drawing area linux
    });

    start_button.connect_clicked(move |button| {
        let state3 = Rc::clone(&state);
        let drawing_area3 = Rc::clone(&drawing_area);
        let signal_ids3 = Rc::clone(&signal_ids);
        if button.label().unwrap().as_str() == "Start" {
            button.set_label("Stop");
            timeout_add_local(Duration::from_millis(100), move || {
                take_step(&mut state3.borrow_mut());
                update_drawing_area(
                    &state3,
                    &drawing_area3.borrow_mut(),
                    &mut signal_ids3.borrow_mut(),
                );
                drawing_area3.borrow().queue_draw_area(0, 0, 600, 600); //refresh drawing area linux
                Continue(state3.borrow().running)
            });
        } else {
            button.set_label("Start");
        }
        negate_running(&mut state.borrow_mut());
    });

    vbox.pack_start(&button_box, false, false, 0);
    window.show_all();
}

fn negate_running(state: &mut SimulationState) {
    state.running = !state.running;
}

fn update_drawing_area(
    state: &Rc<RefCell<SimulationState>>,
    drawing_area: &gtk::DrawingArea,
    signal_ids: &mut Vec<SignalHandlerId>,
) {
    // find old signal handlers and disconnect
    for _ in 0..signal_ids.len() {
        let id = signal_ids.pop();
        drawing_area.disconnect(id.unwrap());
    }

    let width = state.borrow().config.grid_width;
    let height = state.borrow().config.grid_height;

    for (pos, val) in state.borrow().grid.ternary.iter().enumerate() {
        let color = match val {
            1 => (255.0, 0.0, 0.0),
            -1 => (0.0, 255.0, 0.0),
            _ => (255.0, 255.0, 255.0),
        };
        let signal_id =
            drawing_area.connect_draw(move |_, ctx| draw_square(ctx, pos, color, width, height));
        signal_ids.push(signal_id);
    }
}

fn draw_square(
    ctx: &cairo::Context,
    position: usize,
    color: (f64, f64, f64),
    width: usize,
    height: usize,
) -> gtk::Inhibit {
    // ctx.scale(500f64, 500f64);

    ctx.set_line_width(0.5);

    let rect_width = 600.0 / width as f64;
    let rect_height = 600.0 / height as f64;

    ctx.rectangle(
        (position % width) as f64 * rect_width,
        (position / width) as f64 * rect_height,
        rect_width,
        rect_height,
    );
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke().expect("problems stroking squares.");
    ctx.rectangle(
        (position % width) as f64 * rect_width,
        (position / width) as f64 * rect_height,
        rect_width,
        rect_height,
    );
    ctx.set_source_rgb(color.0, color.1, color.2);
    ctx.fill().expect("Problems filling squares.");

    Inhibit(false)
}
