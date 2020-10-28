use std::fs;
//use std::thread;
use std::thread::sleep;
use std::time::Duration;

//use std::rc::Rc;
//use std::sync::Arc;
///use std::cell::RefCell;
///use std::rc::Rc;

use dirs::home_dir;

use gio::prelude::*;
use gtk::prelude::*;

use tesla::{FullVehicleData, StateOfCharge, TeslaClient, Vehicle, VehicleClient, VehicleState, ClimateState, DriveState};

use crate::Config;

// TODO : Use all_data.gui_settings.gui_distance_units // km/hr
const KM_PER_MILES: f64 = 1.6;

pub struct UIElements {
    pub window: gtk::ApplicationWindow,
    pub controls_window: gtk::Window,
    pub loading_banner: gtk::Box,

    pub car_name_label: gtk::Label,
    pub vin: gtk::Label,
    pub car_version: gtk::Label,
    pub odometer: gtk::Label,

    pub rear_trunk_open_image: gtk::Image,
    pub front_trunk_open: gtk::Image,
    pub passenger_front_door_open: gtk::Image,
    pub passenger_rear_door_open: gtk::Image,
    pub driver_front_door_open: gtk::Image,
    pub driver_rear_door_open: gtk::Image,
    pub passenger_front_window_open: gtk::Image,
    pub passenger_rear_window_open: gtk::Image,
    pub driver_front_window_open: gtk::Image,
    pub driver_rear_window_open: gtk::Image,

    pub battery_indicator_bar: gtk::LevelBar,
    pub battery_level_label: gtk::Label,
    pub car_status_label: gtk::Label,
    pub climate_strikethrough_image: gtk::Image,
    pub lock_button_image: gtk::Image,
    pub refresh_button: gtk::Button,
    pub climate_control_button: gtk::Button,
    pub frunk_button: gtk::Button,
    pub lock_button: gtk::Button,
    pub controls_button: gtk::Button,
}

pub struct MyApp {
    ui_elements: UIElements,
    client: TeslaClient,
    vehicle: Vehicle,
    vclient: VehicleClient,
}

impl MyApp {
    // pub fn new(ui_elements: UIElements) -> Rc<RefCell<MyApp>> {
    pub fn new(ui_elements: UIElements) -> MyApp {
        let cfg: Config = MyApp::get_config();
        let client = TeslaClient::default(cfg.global.api_token.as_str());
        debug!("Tesla api client init done. Going to fetch the vehicles...");
        let car_name = cfg.global.default_vehicle.unwrap();
        let vehicle = MyApp::get_vehicle_summary(&client, car_name.as_str());
        debug!("Got the vehicle.");
        let vclient = client.vehicle(vehicle.id);

        let my_app = MyApp{
            ui_elements: ui_elements,
            client: client,
            vehicle: vehicle,
            vclient: vclient,
        };

        my_app.set_buttons();
        return my_app;
        //let my_app_rc: Rc<RefCell<MyApp>> = Rc::new(RefCell::new(my_app));
        //return my_app_rc.clone();
    }

    fn get_config() -> Config {
        // TODO : Allow a different path, different filename and use a different default name.
        let config_path = home_dir().unwrap().join(".teslac");
        let config_data = fs::read_to_string(config_path).expect("Cannot read config");
        toml::from_str(config_data.as_str()).expect("Cannot parse config")
    }

    fn get_vehicle_summary(client: &TeslaClient, car_name: &str) -> Vehicle {
        /*
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let vehicle = client.get_vehicle_by_name(car_name).unwrap().expect("Car does not exist by that name");
            tx.send(vehicle).expect("Couldn't send data to channel");
        });

        rx.attach(None, |vehicle| {
            glib::Continue(true)
        });
        */
        client.get_vehicle_by_name(car_name).unwrap().expect("Car does not exist by that name")
    }

    // Instance methods

    pub fn set_basic_vehicle_info(&self) {
        self.ui_elements.car_name_label.set_text(self.vehicle.display_name.as_str());
        self.ui_elements.vin.set_text(self.vehicle.vin.as_str());
    }

    pub fn wake_up_if_needed(&mut self) {
        if self.vehicle.state != "online" {
            debug!("Waking up");
            match self.vclient.wake_up() {
                Ok(_) => debug!("Sent wakeup command"),
                Err(e) => error!("Wake up failed {:?}", e)
            }

            debug!("Waiting for car to wake up.");
            loop {
                if let Some(_updated_vehicle_info) = self.vclient.get().ok() {
                    self.vehicle = _updated_vehicle_info;
                    if self.vehicle.state == "online" {
                        break;
                    } else {
                        debug!("Car is not yet online (current state is {}), waiting.", self.vehicle.state);
                    }
                }

                sleep(Duration::from_secs(1));
            }
        }
    }

    pub fn refresh(&self) {
        let all_data = self.vclient.get_all_data().expect("Could not get all data");
        self.ui_elements.loading_banner.set_visible(false);
        self.ui_elements.car_version.set_text(all_data.vehicle_state.car_version.as_str());
        // TODO : Read setting to see if we print in normal units or in freedom units.
        let _tmp1 = (all_data.vehicle_state.odometer * KM_PER_MILES) as i32;
        let _tmp = format!("{}", _tmp1);
        self.ui_elements.odometer.set_text(_tmp.as_str());

        self.set_battery_state(&all_data.charge_state);
        self.set_drive_state(&all_data.drive_state);
        self.set_doors_and_windows_state(&all_data.vehicle_state);
        self.set_button_labels(&all_data);
    }

    fn set_doors_and_windows_state(&self, vehicle_state: &VehicleState) {
        self.ui_elements.rear_trunk_open_image.set_opacity(vehicle_state.rt as f64);
        self.ui_elements.front_trunk_open.set_opacity(vehicle_state.ft as f64);
        self.ui_elements.passenger_front_door_open.set_opacity(vehicle_state.pf as f64);
        self.ui_elements.passenger_rear_door_open.set_opacity(vehicle_state.pr as f64);
        self.ui_elements.driver_front_door_open.set_opacity(vehicle_state.df as f64);
        self.ui_elements.driver_rear_door_open.set_opacity(vehicle_state.dr as f64);
        self.ui_elements.passenger_front_window_open.set_opacity(vehicle_state.fp_window as f64);
        self.ui_elements.passenger_rear_window_open.set_opacity(vehicle_state.rp_window as f64);
        self.ui_elements.driver_front_window_open.set_opacity(vehicle_state.fd_window as f64);
        self.ui_elements.driver_rear_window_open.set_opacity(vehicle_state.rd_window as f64);
    }

    fn set_battery_state(&self, charge_state: &StateOfCharge) {
        self.ui_elements.battery_indicator_bar.set_value(charge_state.battery_level as f64 / 100.0);
        // TODO : Read setting to see if we print in normal units or in freedom units.
        let nb_remaining_kms = (charge_state.battery_range * KM_PER_MILES) as i32;
        let charge_level_string = nb_remaining_kms.to_string() + "km";
        self.ui_elements.battery_level_label.set_text(charge_level_string.as_str());
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
        self.ui_elements.car_status_label.set_text(charging_label_text.as_str());
    }

    fn set_drive_state(&self, drive_state: &DriveState) {
        // TODO : Make this more rust-y
        let shift_state = drive_state.shift_state.as_ref();
        if shift_state.is_some() && (shift_state.unwrap() == "D" || shift_state.unwrap() == "R") {
            // TODO : Read setting to see if we print in normal units or in freedom units.
            let speed: i32 = (drive_state.speed.unwrap_or(0) as f64 * KM_PER_MILES) as i32;
            self.ui_elements.car_status_label.set_text(format!("Driving {}km", speed).as_str());
        }
        // TODO : if not charging, then print "Parked"
    }

    fn set_button_labels(&self, all_data: &FullVehicleData) {
        if all_data.climate_state.is_auto_conditioning_on {
            self.ui_elements.climate_strikethrough_image.set_visible(true);
        } else {
            self.ui_elements.climate_strikethrough_image.set_visible(false);
        }

        if all_data.vehicle_state.locked {
            self.ui_elements.lock_button_image.set_from_file("../images/noun_padlock_174116.png");
        } else {
            self.ui_elements.lock_button_image.set_from_file("../images/noun_padlock_174118.png");
        }
    }

    /*
struct Dialog {
   payload: Foo,
}

impl Dialog {
    fn init() -> Rc<RefCell<Dialog>> {
        let dialog = Dialog { payload: something() };
        let handle = Rc::new(RefCell::new(dialog));
        let handle1 = handle.clone();
        button.connect_clicked(move |_| {
              handle1.borrow_mut().do_button()
        };
        handle
    }

    fn do_button(&mut self) {
        // do something
    }
}
    */

    fn set_buttons(&self) {
        /*
        let self2 = self.clone();
        self.ui_elements.refresh_button.connect_clicked(|_| {
            self2.borrow_mut().on_refresh_button_clicked();
        });
        self.ui_elements.climate_control_button.connect_clicked(|_| {
            self.on_climate_control_button_clicked();
        });
        self.ui_elements.frunk_button.connect_clicked(|_| {
            self.on_frunk_button_clicked();
        });
        self.ui_elements.lock_button.connect_clicked(|_| {
            self.on_lock_button_clicked();
        });

        self.ui_elements.controls_button.connect_clicked( |_| {
            self.on_controls_button_clicked();
        });
        */
    }

    fn on_refresh_button_clicked(&self) {
        info!("on_refresh_button_clicked!");
        // TODO :
        //let loading_banner: gtk::Box = builder.get_object("loading_banner").unwrap();
        //loading_banner.show_all();

        /*
        (refresh) button click
        start loading
        spawn thread
        wire channel
        on message from channel
            remove loading
            (if refresh,)
            self.on_refresh_info()
        */

        /*
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let all_data = vclient.get_all_data().expect("Could not get all data");
            tx.send(all_data).expect("Couldn't send data to channel");
        });

        rx.attach(None, move |all_data| {
            let builder2: Rc<gtk::Builder> = Rc::clone(&builder);
            do_refresh(builder2, all_data);
            glib::Continue(true)
        });
        */
    }

    fn on_climate_control_button_clicked(&self) {
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

    fn on_frunk_button_clicked(&self) {
        info!("on_frunk_button_clicked!");
        //TODO: POST /api/1/vehicles/{id}/command/actuate_trunk
    }

    fn on_lock_button_clicked(&self) {
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

    fn on_controls_button_clicked(&self) {
        info!("on_controls_button_clicked!");
        //let controls_window: gtk::Window = builder.get_object("controls_window").unwrap();
        //controls_window.show();
    }
}
