pub struct SmartThermometer {
    name: String,
    state: bool,
}

impl SmartThermometer {
    pub fn new(name: String, state: bool) -> Self {
        Self { name, state }
    }
}