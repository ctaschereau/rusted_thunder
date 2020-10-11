extern crate gdk;
extern crate gio;
extern crate glib;

use std::rc::Rc;
use std::io::Write;
//use futures::{channel::mpsc}; // StreamExt
use std::sync::mpsc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{StateFlags};
#[allow(unused_imports)]
use tesla::{FullVehicleData, StateOfCharge, TeslaClient, Vehicle, VehicleClient, VehicleState, ClimateState};


#[macro_use]
extern crate log;
use chrono::Local;
use log::LevelFilter;
use crate::message_types::{MessagesForGUI, MessagesForWorker};


mod communicator;
mod message_types;
mod config;

// TODO : Use all_data.gui_settings.gui_distance_units // km/hr
const KM_PER_MILES: f64 = 1.6;




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
        .filter(Some("rusted_thunder"), LevelFilter::Debug)
        .init();
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("app_layout.glade");
    let builder: Rc<gtk::Builder> = Rc::new(gtk::Builder::from_string(glade_src));

    let provider = gtk::CssProvider::new();
    provider
        .load_from_path("../style/main.css")
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let loading_banner: gtk::Revealer = builder.get_object("loading_banner").unwrap();
    loading_banner.show_all();

    // Create 2 channels (one for each direction) between the communication thread (API caller) and main event loop
    //let (tx_to_comm, rx_on_comm):(mpsc::Sender<Message2>, mpsc::Receiver<Message2>) = mpsc::channel(1000);
    let (tx_to_comm, rx_on_comm):(mpsc::Sender<MessagesForWorker>, mpsc::Receiver<MessagesForWorker>) = mpsc::channel();
    let (tx_to_gui, rx_on_gui):(glib::Sender<MessagesForGUI>, glib::Receiver<MessagesForGUI>) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    spawn_local_handler(rx_on_gui, tx_to_comm, Rc::clone(&builder));
    communicator::start_communication_thread(rx_on_comm, tx_to_gui);

    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.set_application(Some(app));
    window.show();
    info!("GTK app init done!");
}


fn spawn_local_handler(rx_on_gui: glib::Receiver<MessagesForGUI>, tx_to_comm: mpsc::Sender<MessagesForWorker>, builder: Rc<gtk::Builder>) {
    set_buttons(Rc::clone(&builder), tx_to_comm);
    rx_on_gui.attach(None, move |msg| {
        match msg {
            MessagesForGUI::VehicleName(vehicle_name) => {
                let car_name_label: gtk::Label = builder.get_object("car_name_label").unwrap();
                car_name_label.set_text(vehicle_name.as_str());
            }
            MessagesForGUI::FullVehicleData(all_data) => {
                debug!("The main thread got the data!");
                // println!("Al data : {:#?}", all_data);
                let loading_banner: gtk::Revealer = builder.get_object("loading_banner").unwrap();
                loading_banner.set_visible(false);
                set_battery_state(Rc::clone(&builder), &all_data.charge_state);
                set_doors_and_windows_state(Rc::clone(&builder), &all_data.vehicle_state);
                set_button_labels(Rc::clone(&builder), &all_data);
            },
        }

        glib::Continue(true)
    });
}

fn set_doors_and_windows_state(builder: Rc<gtk::Builder>, vehicle_state: &VehicleState) {
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

fn set_battery_state(builder: Rc<gtk::Builder>, charge_state: &StateOfCharge) {
    let battery_indicator_bar: gtk::LevelBar = builder.get_object("battery_indicator_bar").unwrap();
    battery_indicator_bar.set_value(charge_state.battery_level as f64 / 100.0);
    battery_indicator_bar.add_offset_value("medium", 0.50);
    let battery_level_label: gtk::Label = builder.get_object("battery_level_label").unwrap();
    let car_status_label: gtk::Label = builder.get_object("car_status_label").unwrap();

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
    car_status_label.set_text(charging_label_text.as_str());
}

fn set_button_labels(builder: Rc<gtk::Builder>, all_data: &FullVehicleData) {
    let climate_control_button: gtk::Button = builder.get_object("climate_control_button").unwrap();
    let climate_control_button_image: gtk::Image = builder.get_object("climate_control_button_image").unwrap();
    if all_data.climate_state.is_auto_conditioning_on {
        // TODO : Find image with a strike through or make it
        // climate_control_button_image.set_from_file("../images/noun_Fan_1112062.png");
        climate_control_button.override_background_color(StateFlags::NORMAL, Some(&gdk::RGBA::red()))
    } else {
        climate_control_button_image.set_from_file("../images/noun_Fan_1112062.png");
    }

    let lock_button_image: gtk::Image = builder.get_object("lock_button_image").unwrap();
    if all_data.vehicle_state.locked {
        lock_button_image.set_from_file("../images/noun_padlock_174116.png");
    } else {
        lock_button_image.set_from_file("../images/noun_padlock_174118.png");
    }
}

fn set_buttons(builder: Rc<gtk::Builder>, tx_to_comm: mpsc::Sender<MessagesForWorker>) {
    let tx2 = tx_to_comm.clone();
    let refresh_button: gtk::Button = builder.get_object("refresh_button").unwrap();
    refresh_button.connect_clicked(move |_button| {
        let mut tx3 = tx2.clone();
        on_refresh_button_clicked(&mut tx3);
    });

    let climate_control_button: gtk::Button = builder.get_object("climate_control_button").unwrap();
    climate_control_button.connect_clicked(|_button| {
        on_climate_control_button_clicked(_button);
    });
    let frunk_button: gtk::Button = builder.get_object("frunk_button").unwrap();
    frunk_button.connect_clicked(|_button| {
        on_frunk_button_clicked(_button);
    });
    let lock_button: gtk::Button = builder.get_object("lock_button").unwrap();
    lock_button.connect_clicked(|_button| {
        on_lock_button_clicked(_button);
    });

    //let builder2 = Rc::clone(&builder);
    let controls_button: gtk::Button = builder.get_object("controls_button").unwrap();
    controls_button.connect_clicked(move |_button| {
        on_controls_button_clicked(_button);
    });
}

fn on_refresh_button_clicked(tx_to_comm: &mut mpsc::Sender<MessagesForWorker>) {
    match tx_to_comm.send(MessagesForWorker::DoRefresh()) {
        Ok(_) => {}
        Err(err) => error!("{:?}", err)
    }
}

fn on_climate_control_button_clicked(_button: &gtk::Button) {
    info!("on_climate_control_button_clicked!");
    /*
    if all_data.climate_state.is_auto_conditioning_on {
        match vclient.auto_conditioning_stop() {
            // TODO : Should I log the _v variable if _.result != true ?
            Ok(_v) => info!("auto_conditioning has been stopped."),
            Err(e) => info!("failed to stop the auto_conditioning: {:?}", e),
        }
    } else {
        match vclient.auto_conditioning_start() {
            Ok(_v) => info!("auto_conditioning has been turned on."),
            Err(e) => info!("failed to start the auto_conditioning: {:?}", e),
        }
    }
    */
}

fn on_frunk_button_clicked(_button: &gtk::Button) {
    info!("on_frunk_button_clicked!");
    //TODO: POST /api/1/vehicles/{id}/command/actuate_trunk
}

fn on_lock_button_clicked(_button: &gtk::Button) {
    info!("on_lock_button_clicked!");
    /*
    if all_data.vehicle_state.locked {
        match vclient.door_unlock() {
            // TODO : Should I log the _v variable if _.result != true ?
            Ok(_v) => println!("doors have been unlocked."),
            Err(e) => println!("failed to unlock the doors: {:?}", e),
        }
    } else {
        match vclient.door_lock() {
            Ok(_v) => println!("doors have been locked."),
            Err(e) => println!("failed to lock the doors: {:?}", e),
        }
    }
    */
}

fn on_controls_button_clicked(_button: &gtk::Button) {
    info!("on_controls_button_clicked!");
    //let controls_window: gtk::Window = builder.get_object("controls_window").unwrap();
    //controls_window.show();
}
