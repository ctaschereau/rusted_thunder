use std::fs;
use std::thread::sleep;
use std::time::Duration;
use dirs::home_dir;
use crate::config::Config;

mod config;

use tesla::{TeslaClient, Vehicle, VehicleState, VehicleClient, StateOfCharge};

use gtk::prelude::*;
use gio::prelude::*;


// TODO : Use all_data.gui_settings.gui_distance_units // km/hr
const KM_PER_MILES: f64 = 1.6;


fn main() {
    let app = gtk::Application::new(Some("com.github.ctaschereau.rusted_thunder"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| build_ui(app));
    app.run(&std::env::args().collect::<Vec<_>>());
}

fn build_ui(app: &gtk::Application) {
    let cfg: Config = get_config();
    let client = TeslaClient::default(cfg.global.api_token.as_str());
    let car_name = cfg.global.default_vehicle.unwrap();
    let vehicle = client.get_vehicle_by_name(car_name.as_str()).unwrap().expect("Car does not exist by that name");
    let vclient = client.vehicle(vehicle.id);

    wake_up_if_needed(&vclient, vehicle);

    let all_data = vclient.get_all_data().expect("Could not get all data");
    // println!("Al data : {:#?}", all_data);

    let glade_src = include_str!("app_layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let car_name_label: gtk::Label = builder.get_object("car_name_label").unwrap();
    car_name_label.set_text(car_name.as_str());

    set_buttons(&builder);

    set_doors_and_windows_state(&builder, &all_data.vehicle_state);

    set_battery_state(&builder, &all_data.charge_state);

    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.set_application(Some(app));
    window.show_all();
}

fn get_config() -> Config {
    let config_path = home_dir().unwrap().join(".teslac");
    let config_data = fs::read_to_string(config_path).expect("Cannot read config");
    return toml::from_str(config_data.as_str()).expect("Cannot parse config");
}

fn set_buttons(builder: &gtk::Builder) {
    let climate_control_button: gtk::Button = builder.get_object("climate_control_button").unwrap();
    climate_control_button.connect_clicked(on_climate_control_button_clicked);
    let frunk_button: gtk::Button = builder.get_object("frunk_button").unwrap();
    frunk_button.connect_clicked(on_frunk_button_clicked);
    let lock_button: gtk::Button = builder.get_object("lock_button").unwrap();
    lock_button.connect_clicked(on_lock_button_clicked);
}

fn set_doors_and_windows_state(builder: &gtk::Builder, vehicle_state: &VehicleState) {
    let rear_trunk_open_image: gtk::Image = builder.get_object("rear_trunk_open_image").unwrap();
    rear_trunk_open_image.set_opacity(vehicle_state.rt as f64);
    let front_trunk_open: gtk::Image = builder.get_object("front_trunk_open").unwrap();
    front_trunk_open.set_opacity(vehicle_state.ft as f64);
    let passenger_front_door_open: gtk::Image = builder.get_object("passenger_front_door_open").unwrap();
    passenger_front_door_open.set_opacity(vehicle_state.pf as f64);
    let passenger_rear_door_open: gtk::Image = builder.get_object("passenger_rear_door_open").unwrap();
    passenger_rear_door_open.set_opacity(vehicle_state.pr as f64);
    let driver_front_door_open: gtk::Image = builder.get_object("driver_front_door_open").unwrap();
    driver_front_door_open.set_opacity(vehicle_state.df as f64);
    let driver_rear_door_open: gtk::Image = builder.get_object("driver_rear_door_open").unwrap();
    driver_rear_door_open.set_opacity(vehicle_state.dr as f64);
    let passenger_front_window_open: gtk::Image = builder.get_object("passenger_front_window_open").unwrap();
    passenger_front_window_open.set_opacity(vehicle_state.fp_window as f64);
    let passenger_rear_window_open: gtk::Image = builder.get_object("passenger_rear_window_open").unwrap();
    passenger_rear_window_open.set_opacity(vehicle_state.rp_window as f64);
    let driver_front_window_open: gtk::Image = builder.get_object("driver_front_window_open").unwrap();
    driver_front_window_open.set_opacity(vehicle_state.fd_window as f64);
    let driver_rear_window_open: gtk::Image = builder.get_object("driver_rear_window_open").unwrap();
    driver_rear_window_open.set_opacity(vehicle_state.rd_window as f64);
}

fn set_battery_state(builder: &gtk::Builder, charge_state: &StateOfCharge) {
    let battery_indicator_bar: gtk::LevelBar = builder.get_object("battery_indicator_bar").unwrap();
    battery_indicator_bar.set_value(charge_state.battery_level as f64 / 100.0);
    let battery_level_label: gtk::Label = builder.get_object("battery_level_label").unwrap();
    let charging_label: gtk::Label = builder.get_object("charging_label").unwrap();

    let nb_remaining_kms = (charge_state.battery_range * KM_PER_MILES) as i32;
    let charge_level_string = nb_remaining_kms.to_string() + "km";
    battery_level_label.set_text(charge_level_string.as_str());
    let charging_label_text: String;
    match charge_state.charging_state.as_str() {
        "Disconnected" => charging_label_text = String::from(""),
        "Complete" => charging_label_text = String::from("Charging complete"),
        "Charging" => {
            let nb_minutes_until_full = (charge_state.time_to_full_charge * 60.0) as i32;
            charging_label_text = format!("Charging... {} minutes remaining", nb_minutes_until_full);
        },
        _ => charging_label_text = charge_state.charging_state.clone()
    }
    charging_label.set_text(charging_label_text.as_str());
}

fn on_climate_control_button_clicked(_button: &gtk::Button) {
    println!("on_climate_control_button_clicked!");
}

fn on_frunk_button_clicked(_button: &gtk::Button) {
    println!("on_frunk_button_clicked!");
}

fn on_lock_button_clicked(_button: &gtk::Button) {
    println!("on_lock_button_clicked!");
}

fn wake_up_if_needed(vclient: &VehicleClient, vehicle: Vehicle) {
    if vehicle.state != "online" {
        println!("Waking up");
        match vclient.wake_up() {
            Ok(_) => println!("Sent wakeup command"),
            Err(e) => println!("Wake up failed {:?}", e)
        }

        println!("Waiting for car to wake up.");
        loop {
            if let Some(vehicle) = vclient.get().ok() {
                if vehicle.state == "online" {
                    break;
                } else {
                    println!("Car is not yet online (current state is {}), waiting.", vehicle.state);
                }
            }

            sleep(Duration::from_secs(1));
        }
    }
}
