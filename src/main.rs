extern crate gdk;
extern crate gio;
extern crate glib;

use std::fs;
use std::thread::sleep;
use std::time::Duration;
//use std::rc::Rc;
use std::io::Write;
use std::thread;

use dirs::home_dir;
use gio::prelude::*;
use gtk::prelude::*;
#[allow(unused_imports)]
use tesla::{FullVehicleData, StateOfCharge, TeslaClient, Vehicle, VehicleClient, VehicleState, ClimateState};

use crate::config::Config;
#[macro_use]
extern crate log;
use chrono::Local;
use log::LevelFilter;

mod config;

// TODO : Use all_data.gui_settings.gui_distance_units // km/hr
const KM_PER_MILES: f64 = 1.6;


enum Message {
    SendVehicle(Vehicle),
    SendFullVehicleData(FullVehicleData),
}

/*
struct RustedThunderApp {
    builder: gtk::Builder,
    #[allow(dead_code)]
    client: TeslaClient,
    vclient: VehicleClient,
    all_data: FullVehicleData,
}

impl RustedThunderApp {
    fn new(builder: gtk::Builder, client: TeslaClient, vclient: VehicleClient, all_data: FullVehicleData) -> Rc<RustedThunderApp> {
        let instance = Rc::new(RustedThunderApp {
            builder: builder,
            client: client,
            vclient: vclient,
            all_data: all_data,
        });
        instance
    }
*/
    /*
    fn set_button_labels(&self) {
        let climate_control_button: gtk::Button = self.builder.get_object("climate_control_button").unwrap();
        if self.all_data.climate_state.is_auto_conditioning_on {
            climate_control_button.set_label("Turn climate control OFF");
        } else {
            climate_control_button.set_label("Turn climate control ON");
        }

        let lock_button: gtk::Button = self.builder.get_object("lock_button").unwrap();
        if self.all_data.vehicle_state.locked {
            lock_button.set_label("Unlock");
        } else {
            lock_button.set_label("Lock");
        }
    }
    */

    /*
    fn set_buttons(&self, cloned_self: Rc<RustedThunderApp>) {
        let climate_control_button: gtk::Button = self.builder.get_object("climate_control_button").unwrap();
        climate_control_button.connect_clicked(move |_button| {
            cloned_self.on_climate_control_button_clicked(_button);
        });
        */
        // TODO: How can I have set_buttons, set_buttons2 and set_buttons3 be the same function without having "value used here after move" problems?
        /*
        let frunk_button: gtk::Button = self.builder.get_object("frunk_button").unwrap();
        frunk_button.connect_clicked(move |_button| {
            cloned_self.on_frunk_button_clicked(_button);
        });
        let lock_button: gtk::Button = self.builder.get_object("lock_button").unwrap();
        lock_button.connect_clicked(move |_button| {
            cloned_self.on_lock_button_clicked(_button);
        });
        */
    //}

    /*
    fn set_buttons2(&self, cloned_self: Rc<RustedThunderApp>) {
        let frunk_button: gtk::Button = self.builder.get_object("frunk_button").unwrap();
        frunk_button.connect_clicked(move |_button| {
            cloned_self.on_frunk_button_clicked(_button);
        });
    }

    fn set_buttons3(&self, cloned_self: Rc<RustedThunderApp>) {
        let lock_button: gtk::Button = self.builder.get_object("lock_button").unwrap();
        lock_button.connect_clicked(move |_button| {
            cloned_self.on_lock_button_clicked(_button);
        });
    }

    fn set_doors_and_windows_state(&self) {
        let vehicle_state: &VehicleState = &self.all_data.vehicle_state;

        let rear_trunk_open_image: gtk::Image = self.builder.get_object("rear_trunk_open_image").unwrap();
        rear_trunk_open_image.set_opacity(vehicle_state.rt as f64);
        let front_trunk_open: gtk::Image = self.builder.get_object("front_trunk_open").unwrap();
        front_trunk_open.set_opacity(vehicle_state.ft as f64);
        let passenger_front_door_open: gtk::Image = self.builder.get_object("passenger_front_door_open").unwrap();
        passenger_front_door_open.set_opacity(vehicle_state.pf as f64);
        let passenger_rear_door_open: gtk::Image = self.builder.get_object("passenger_rear_door_open").unwrap();
        passenger_rear_door_open.set_opacity(vehicle_state.pr as f64);
        let driver_front_door_open: gtk::Image = self.builder.get_object("driver_front_door_open").unwrap();
        driver_front_door_open.set_opacity(vehicle_state.df as f64);
        let driver_rear_door_open: gtk::Image = self.builder.get_object("driver_rear_door_open").unwrap();
        driver_rear_door_open.set_opacity(vehicle_state.dr as f64);
        let passenger_front_window_open: gtk::Image = self.builder.get_object("passenger_front_window_open").unwrap();
        passenger_front_window_open.set_opacity(vehicle_state.fp_window as f64);
        let passenger_rear_window_open: gtk::Image = self.builder.get_object("passenger_rear_window_open").unwrap();
        passenger_rear_window_open.set_opacity(vehicle_state.rp_window as f64);
        let driver_front_window_open: gtk::Image = self.builder.get_object("driver_front_window_open").unwrap();
        driver_front_window_open.set_opacity(vehicle_state.fd_window as f64);
        let driver_rear_window_open: gtk::Image = self.builder.get_object("driver_rear_window_open").unwrap();
        driver_rear_window_open.set_opacity(vehicle_state.rd_window as f64);
    }

    fn set_battery_state(&self) {
        let charge_state: &StateOfCharge = &self.all_data.charge_state;

        let battery_indicator_bar: gtk::LevelBar = self.builder.get_object("battery_indicator_bar").unwrap();
        battery_indicator_bar.set_value(charge_state.battery_level as f64 / 100.0);
        battery_indicator_bar.add_offset_value("medium", 0.50);
        let battery_level_label: gtk::Label = self.builder.get_object("battery_level_label").unwrap();
        let charging_label: gtk::Label = self.builder.get_object("charging_label").unwrap();

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
            }
            _ => charging_label_text = charge_state.charging_state.clone()
        }
        charging_label.set_text(charging_label_text.as_str());
    }

    fn on_climate_control_button_clicked(&self, _button: &gtk::Button) {
        if self.all_data.climate_state.is_auto_conditioning_on {
            match self.vclient.auto_conditioning_stop() {
                // TODO : Should I log the _v variable if _.result != true ?
                Ok(_v) => println!("auto_conditioning has been stopped."),
                Err(e) => println!("failed to stop the auto_conditioning: {:?}", e),
            }
        } else {
            match self.vclient.auto_conditioning_start() {
                Ok(_v) => println!("auto_conditioning has been turned on."),
                Err(e) => println!("failed to start the auto_conditioning: {:?}", e),
            }
        }
    }

    fn on_frunk_button_clicked(&self, _button: &gtk::Button) {
        println!("on_frunk_button_clicked!");
        //TODO: POST /api/1/vehicles/{id}/command/actuate_trunk
    }

    fn on_lock_button_clicked(&self, _button: &gtk::Button) {
        if self.all_data.vehicle_state.locked {
            match self.vclient.door_unlock() {
                // TODO : Should I log the _v variable if _.result != true ?
                Ok(_v) => println!("doors have been unlocked."),
                Err(e) => println!("failed to unlock the doors: {:?}", e),
            }
        } else {
            match self.vclient.door_lock() {
                Ok(_v) => println!("doors have been locked."),
                Err(e) => println!("failed to lock the doors: {:?}", e),
            }
        }
    }
    */
//}

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
        .filter(Some("rusted_thunder"), LevelFilter::Info)
        .init();
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("app_layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let provider = gtk::CssProvider::new();
    provider
        .load_from_path("./style/main.css")
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let spinner_screen: gtk::EventBox = builder.get_object("spinner_screen").unwrap();
    spinner_screen.show_all();

    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.set_application(Some(app));
    window.show();
    info!("GTK app init done!");

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        debug!("Going to init the Tesla api clients...");
        let cfg: Config = get_config();
        let client = TeslaClient::default(cfg.global.api_token.as_str());
        let car_name = cfg.global.default_vehicle.unwrap();
        debug!("Tesla api client init done. Going to fetch the vehicles...");
        let vehicle = client.get_vehicle_by_name(car_name.as_str()).unwrap().expect("Car does not exist by that name");
        let vclient = client.vehicle(vehicle.id);
        debug!("Got the vehicles.");
        let vehicle_state = vehicle.state.clone();
        tx.send(Message::SendVehicle(vehicle)).expect("Couldn't send data to channel");

        if vehicle_state != "online" {
            wake_up(&vclient);
        }

        debug!("Going to get the vehicle data!");
        let all_data = vclient.get_all_data().expect("Could not get all data");

        tx.send(Message::SendFullVehicleData(all_data)).expect("Couldn't send data to channel");
    });

    rx.attach(None, move |msg| {
        match msg {
            Message::SendVehicle(vehicle) => {
                let car_name_label: gtk::Label = builder.get_object("car_name_label").unwrap();
                car_name_label.set_text(vehicle.display_name.as_str());
            }
            Message::SendFullVehicleData(all_data) => {
                // println!("Al data : {:#?}", all_data);
                spinner_screen.set_visible(false);
                set_battery_state(&builder, &all_data.charge_state);
                set_doors_and_windows_state(&builder, &all_data.vehicle_state);

                /*
                let rt_app = RustedThunderApp::new(builder, client, vclient, all_data);

                let car_name_label: gtk::Label = rt_app.builder.get_object("car_name_label").unwrap();
                car_name_label.set_text(car_name.as_str());

                rt_app.set_button_labels();

                // TODO: how to get a different ref counter for my app from within the set_buttons method???
                let cloned_rt_app = Rc::clone(&rt_app);
                rt_app.set_buttons(cloned_rt_app);
                let cloned_rt_app2 = Rc::clone(&rt_app);
                rt_app.set_buttons2(cloned_rt_app2);
                let cloned_rt_app3 = Rc::clone(&rt_app);
                rt_app.set_buttons3(cloned_rt_app3);

                rt_app.set_doors_and_windows_state();

                rt_app.set_battery_state();
                */
            },
        }

        glib::Continue(true)
    });
}

fn get_config() -> Config {
    // TODO : Allow a different path, different filename and use a different default name.
    let config_path = home_dir().unwrap().join(".teslac");
    let config_data = fs::read_to_string(config_path).expect("Cannot read config");
    return toml::from_str(config_data.as_str()).expect("Cannot parse config");
}

fn wake_up(vclient: &VehicleClient) {
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

fn set_doors_and_windows_state(builder: &gtk::Builder, vehicle_state: &VehicleState) {
    let rear_trunk_open_image: gtk::Image =  builder.get_object("rear_trunk_open_image").unwrap();
    rear_trunk_open_image.set_opacity(vehicle_state.rt as f64);
    let front_trunk_open: gtk::Image =  builder.get_object("front_trunk_open").unwrap();
    front_trunk_open.set_opacity(vehicle_state.ft as f64);
    let passenger_front_door_open: gtk::Image =  builder.get_object("passenger_front_door_open").unwrap();
    passenger_front_door_open.set_opacity(vehicle_state.pf as f64);
    let passenger_rear_door_open: gtk::Image =  builder.get_object("passenger_rear_door_open").unwrap();
    passenger_rear_door_open.set_opacity(vehicle_state.pr as f64);
    let driver_front_door_open: gtk::Image =  builder.get_object("driver_front_door_open").unwrap();
    driver_front_door_open.set_opacity(vehicle_state.df as f64);
    let driver_rear_door_open: gtk::Image =  builder.get_object("driver_rear_door_open").unwrap();
    driver_rear_door_open.set_opacity(vehicle_state.dr as f64);
    let passenger_front_window_open: gtk::Image =  builder.get_object("passenger_front_window_open").unwrap();
    passenger_front_window_open.set_opacity(vehicle_state.fp_window as f64);
    let passenger_rear_window_open: gtk::Image =  builder.get_object("passenger_rear_window_open").unwrap();
    passenger_rear_window_open.set_opacity(vehicle_state.rp_window as f64);
    let driver_front_window_open: gtk::Image =  builder.get_object("driver_front_window_open").unwrap();
    driver_front_window_open.set_opacity(vehicle_state.fd_window as f64);
    let driver_rear_window_open: gtk::Image = builder.get_object("driver_rear_window_open").unwrap();
    driver_rear_window_open.set_opacity(vehicle_state.rd_window as f64);
}

fn set_battery_state(builder: &gtk::Builder, charge_state: &StateOfCharge) {
    let battery_indicator_bar: gtk::LevelBar = builder.get_object("battery_indicator_bar").unwrap();
    battery_indicator_bar.set_value(charge_state.battery_level as f64 / 100.0);
    battery_indicator_bar.add_offset_value("medium", 0.50);
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
        }
        _ => charging_label_text = charge_state.charging_state.clone()
    }
    charging_label.set_text(charging_label_text.as_str());
}
