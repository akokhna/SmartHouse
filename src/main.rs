mod domain;
use domain::house::smart_house;
use domain::device::smart_device;
use domain::room::smart_room;
use domain::device::smart_thermometer::SmartThermometer;
use domain::device::smart_socket::SmartSocket;

fn main() {
    println!("Start....")
    // Инициализация устройств
     let socket1 = SmartSocket::new(String::from("socke_1"), true);
     let socket2 = SmartSocket::new(String::from("socke_2"), true);
     let thermo = SmartThermometer::new(String::from("thermo_1"),false);

    // Инициализация Квартиры
    let 

    // Инициализация дома

    // let house = SmartHouse::new();
    //
    //
    // // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    // let info_provider_1 = OwningDeviceInfoProvider {
    //     socket: socket1,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report1 = house.create_report(/* &info_provider_1 */);
    //
    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermo: &thermo,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(/* &info_provider_2 */);
    //
    // // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    // println!("Report #2: {report2}");
}