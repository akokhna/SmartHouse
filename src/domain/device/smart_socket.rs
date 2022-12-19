
pub struct SmartSocket {
    name: String,
    state: bool,
}

impl SmartSocket{
    pub fn new(name: String, state: bool) -> Self {
        Self { name, state }
    }

}