use rppal::gpio::Level;

pub enum ButtonStatus {
    Release,
    Push,
}

impl ButtonStatus {
    pub fn id(&self) -> i8{
        match self {
            Self::Release => 1,
            Self::Push => 2,
        }
    }

    pub fn from_level(level: Level) -> ButtonStatus {
        match level {
            Level::Low => ButtonStatus::Push,
            Level::High => ButtonStatus::Release,
        }
    }

}

