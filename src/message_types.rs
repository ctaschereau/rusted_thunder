use tesla::{FullVehicleData};

pub enum MessagesForGUI {
    VehicleName(String),
    FullVehicleData(FullVehicleData),
}

pub enum MessagesForWorker {
    DoRefresh(),
}
