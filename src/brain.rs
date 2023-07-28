use std::ops::Sub;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Brain {
    pub state: State,
    pub state_history: Vec<State>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct State {
    pub counterA: u8,
    pub counterB: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            counterA: 0,
            counterB: 0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Event {
    IncrementA,
    DecrementA,
    IncrementB,
    DecrementB,
    Commit,
    Rollback,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            state_history: Vec::new(),
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::IncrementA => self.state.counterA += 1,
            Event::DecrementA => self.state.counterA = self.state.counterA.saturating_sub(1),
            Event::IncrementB => self.state.counterB += 1,
            Event::DecrementB => self.state.counterB = self.state.counterB.saturating_sub(1),
            Event::Commit => self.state_history.push(self.state.clone()),
            Event::Rollback => {
                if let Some(state) = self.state_history.pop() {
                    self.state = state;
                }
            }
        }
    }
}
