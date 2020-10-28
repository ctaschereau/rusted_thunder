#![allow(unused_variables)]
#![allow(unused_imports)]
#![warn(dead_code)]
extern crate gdk;
extern crate gio;
extern crate glib;

use std::io::Write;

use gio::prelude::*;
use gtk::prelude::*;


// https://docs.rs/secret-service/1.1.0/secret_service/index.html

#[macro_use]
extern crate log;
use chrono::Local;
use crate::config::Config;
use log::LevelFilter;
// use crate::message_types::{MessagesForGUI, MessagesForWorker};


mod config;
mod app;

fn main() {
    init_logger();
    info!("rusted_thunder starting up...");

    let app = gtk::Application::new(
        Some("com.github.ctaschereau.rusted_thunder"),
        Default::default()
    ).expect("Initialization failed...");
    app.connect_activate(|app| build_ui(app));
    app.run(&std::env::args().collect::<Vec<_>>());
}

fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(Some("rusted_thunder"), LevelFilter::Debug)
        .init();
}

fn init_css_provider() {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_path("../style/main.css")
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("app_layout.glade");
    let builder: gtk::Builder = gtk::Builder::from_string(glade_src);

    let app_ui_elements = app::UIElements {
        window: builder.get_object("main_window").unwrap(),
        controls_window: builder.get_object("controls_window").unwrap(),
        loading_banner: builder.get_object("loading_banner").unwrap(),

        car_name_label: builder.get_object("car_name_label").unwrap(),
        vin: builder.get_object("vin").unwrap(),
        car_version: builder.get_object("car_version").unwrap(),
        odometer: builder.get_object("odometer").unwrap(),

        rear_trunk_open_image: builder.get_object("rear_trunk_open_image").unwrap(),
        front_trunk_open: builder.get_object("front_trunk_open").unwrap(),
        passenger_front_door_open: builder.get_object("passenger_front_door_open").unwrap(),
        passenger_rear_door_open: builder.get_object("passenger_rear_door_open").unwrap(),
        driver_front_door_open: builder.get_object("driver_front_door_open").unwrap(),
        driver_rear_door_open: builder.get_object("driver_rear_door_open").unwrap(),
        passenger_front_window_open: builder.get_object("passenger_front_window_open").unwrap(),
        passenger_rear_window_open: builder.get_object("passenger_rear_window_open").unwrap(),
        driver_front_window_open: builder.get_object("driver_front_window_open").unwrap(),
        driver_rear_window_open: builder.get_object("driver_rear_window_open").unwrap(),

        battery_indicator_bar: builder.get_object("battery_indicator_bar").unwrap(),
        battery_level_label: builder.get_object("battery_level_label").unwrap(),
        car_status_label: builder.get_object("car_status_label").unwrap(),
        climate_strikethrough_image: builder.get_object("climate_strikethrough_image").unwrap(),
        lock_button_image: builder.get_object("lock_button_image").unwrap(),
        refresh_button: builder.get_object("refresh_button").unwrap(),
        climate_control_button: builder.get_object("climate_control_button").unwrap(),
        frunk_button: builder.get_object("frunk_button").unwrap(),
        lock_button: builder.get_object("lock_button").unwrap(),
        controls_button: builder.get_object("controls_button").unwrap(),
    };

    init_css_provider();

    setup_level_bar_custom_colors(&app_ui_elements.battery_indicator_bar);

    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.set_application(Some(app));
    /*
    window.connect_focus_in_event(|_, _| {
        println!("got the focus");
        Inhibit(true)
    });
    window.connect_focus_out_event(|_, _| {
        println!("lost the focus");
        Inhibit(true)
    });
    */
    // gdk::EventMask::FocusChangeMask
    window.show();
    info!("GTK app init done!");

    app_ui_elements.loading_banner.show_all();
    let mut my_app = app::MyApp::new(app_ui_elements);
    my_app.set_basic_vehicle_info();
    my_app.wake_up_if_needed();
    my_app.refresh();
}

fn setup_level_bar_custom_colors(battery_indicator_bar: &gtk::LevelBar) {
    battery_indicator_bar.remove_offset_value(Some("low"));
    battery_indicator_bar.remove_offset_value(Some("high"));
    battery_indicator_bar.remove_offset_value(Some("full"));
    battery_indicator_bar.add_offset_value("rt_low", 0.10);
    battery_indicator_bar.add_offset_value("rt_medium", 0.30);
    battery_indicator_bar.add_offset_value("rt_full", 1.0);
}
