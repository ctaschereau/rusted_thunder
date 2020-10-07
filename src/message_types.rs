use tesla::{FullVehicleData, Vehicle};

pub enum MessagesForGUI {
    VehicleName(String),
    FullVehicleData(FullVehicleData),
}

pub enum MessagesForWorker {
    DoRefresh(),
}
