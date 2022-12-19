use crate::smart_room::SmartRoom;

pub struct SmartHouse {
    name: String,
    rooms: Vec<SmartRoom>,
}

impl SmartHouse {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    fn get_rooms(&self) -> [&str; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список комнат")
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список устройств в комнате `room`")
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}