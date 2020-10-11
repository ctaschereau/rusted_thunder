//use futures::{channel::mpsc, StreamExt};

use std::fs;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc;

use dirs::home_dir;
// use tesla::{FullVehicleData, StateOfCharge, TeslaClient, Vehicle, VehicleClient, VehicleState, ClimateState};
use tesla::{TeslaClient, VehicleClient};

use crate::config::Config;
use crate::message_types::{MessagesForGUI, MessagesForWorker};

pub fn start_communication_thread(rx_on_comm: mpsc::Receiver<MessagesForWorker>, tx_to_gui: glib::Sender<MessagesForGUI>) {
    thread::spawn(move || {
        debug!("Going to init the Tesla api clients...");
        let cfg: Config = get_config();
        let client = TeslaClient::default(cfg.global.api_token.as_str());
        let car_name = cfg.global.default_vehicle.unwrap();
        debug!("Tesla api client init done. Going to fetch the vehicles...");
        let vehicle = client.get_vehicle_by_name(car_name.as_str()).unwrap().expect("Car does not exist by that name");
        let vclient = client.vehicle(vehicle.id);
        debug!("Got the vehicles.");
        tx_to_gui.send(MessagesForGUI::VehicleName(vehicle.display_name.clone())).expect("Couldn't send data to channel");
        if vehicle.state != "online" {
            wake_up(&vclient);
        }

        debug!("Going to get the initial vehicle data!");
        refresh(&vclient, tx_to_gui.clone());

        // Start a loop that blocks and waits for new messages
        let mut worker_message_receiver = rx_on_comm.iter();
        loop {
            let next_message = worker_message_receiver.next();
            if next_message.is_some() {
                match next_message.unwrap() {
                    MessagesForWorker::DoRefresh() => {
                        debug!("The comm thread received a DoRefresh request!");
                        refresh(&vclient, tx_to_gui.clone());
                    }
                }
            }
        }
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

fn refresh(vclient: &VehicleClient, tx_to_gui: glib::Sender<MessagesForGUI>) {
    let all_data = vclient.get_all_data().expect("Could not get all data");
    tx_to_gui.send(MessagesForGUI::FullVehicleData(all_data)).expect("Couldn't send data to channel");
    debug!("The comm thread sent back the refresh request!");
}
