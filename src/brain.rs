#[derive(serde::Deserialize, serde::Serialize)]
pub struct Brain {
    pub counterA: u8,
    // pub store: Vec<Event>,
}

pub enum Event {
    IncrementA,
    DecrementA,
}

impl Brain {
    pub fn new() -> Self {
        Self { counterA: 0 }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::IncrementA => self.counterA += 1,
            Event::DecrementA => self.counterA -= 1,
        }
    }
}
