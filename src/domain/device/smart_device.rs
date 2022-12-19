pub struct SmartDevice{
    name: String,
}


trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}

//
// struct OwningDeviceInfoProvider {
//     socket: SmartSocket,
// }
//
// struct BorrowingDeviceInfoProvider<'a, 'b> {
//     socket: &'a SmartSocket,
//     thermo: &'b SmartThermometer,
// }