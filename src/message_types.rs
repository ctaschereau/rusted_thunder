use tesla::{FullVehicleData, Vehicle};

// TODO : Rename these

// messages from worker thread to gui thread
pub enum Message {
    SendVehicle(Vehicle),
    SendFullVehicleData(FullVehicleData),
}

// messages from gui thread to worker thread
pub enum Message2 {
    DoRefresh(),
}
