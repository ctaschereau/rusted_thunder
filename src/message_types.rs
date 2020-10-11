use tesla::{Vehicle, FullVehicleData};

pub enum MessagesForGUI {
    VehicleInfo(Vehicle),
    FullVehicleData(FullVehicleData),
}

pub enum MessagesForWorker {
    DoRefresh(),
}
