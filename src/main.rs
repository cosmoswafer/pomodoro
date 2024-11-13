use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use libadwaita as adw;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.example.GnomePomodoro";
const WORK_TIME: u32 = 25 * 60; // 25 minutes in seconds
const BREAK_TIME: u32 = 5 * 60; // 5 minutes in seconds

struct PomodoroTimer {
    time_remaining: u32,
    is_running: bool,
    is_work_period: bool,
    timer_label: gtk4::Label,
}

impl PomodoroTimer {
    fn new(timer_label: gtk4::Label) -> Self {
        Self {
            time_remaining: WORK_TIME,
            is_running: false,
            is_work_period: true,
            timer_label,
        }
    }

    fn format_time(&self) -> String {
        let minutes = self.time_remaining / 60;
        let seconds = self.time_remaining % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn update_label(&self) {
        self.timer_label.set_text(&self.format_time());
    }

    fn toggle(&mut self) -> bool {
        self.is_running = !self.is_running;
        self.is_running
    }

    fn tick(&mut self) -> bool {
        if self.is_running && self.time_remaining > 0 {
            self.time_remaining -= 1;
            self.update_label();
            true
        } else if self.time_remaining == 0 {
            self.is_work_period = !self.is_work_period;
            self.time_remaining = if self.is_work_period {
                WORK_TIME
            } else {
                BREAK_TIME
            };
            self.is_running = false;
            self.update_label();
            false
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.time_remaining = WORK_TIME;
        self.is_running = false;
        self.is_work_period = true;
        self.update_label();
    }
}

fn build_ui(app: &Application) {
    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pomodoro Timer")
        .default_width(300)
        .default_height(200)
        .build();

    // Create a vertical box
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_all(10);

    // Create the timer label
    let timer_label = Label::new(None);
    timer_label.set_markup("<span font='38'>");

    // Create the timer instance
    let timer = Rc::new(RefCell::new(PomodoroTimer::new(timer_label.clone())));
    timer.borrow_mut().update_label();

    // Create buttons
    let start_button = Button::with_label("Start/Pause");
    let reset_button = Button::with_label("Reset");

    // Add widgets to the box
    vbox.append(&timer_label);
    vbox.append(&start_button);
    vbox.append(&reset_button);

    // Set up button click handlers
    let timer_clone = timer.clone();
    start_button.connect_clicked(move |_| {
        let mut timer = timer_clone.borrow_mut();
        let is_running = timer.toggle();
        
        if is_running {
            let timer_weak = Rc::downgrade(&timer_clone);
            glib::timeout_add_local(Duration::from_secs(1), move || {
                if let Some(timer) = timer_weak.upgrade() {
                    return glib::Continue(timer.borrow_mut().tick());
                }
                glib::Continue(false)
            });
        }
    });

    let timer_clone = timer.clone();
    reset_button.connect_clicked(move |_| {
        timer_clone.borrow_mut().reset();
    });

    // Add the box to the window
    window.set_child(Some(&vbox));
    window.present();
}

fn main() {
    // Initialize GTK
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}
