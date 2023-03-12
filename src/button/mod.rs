mod button_status;

use std::error::Error;

use rppal::gpio::{InputPin, Gpio, Level};

pub struct Button{
    pin: InputPin,
    count: u8,
    status: Level,
}

impl Button {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: Gpio::new()?.get(pin)?.into_input(),
            count: 0,
            status: Level::High,
        })
    }

    pub fn observe(&mut self) -> bool {
        let mut is_change = false;
        match self.pin.read()
         {
            Level::Low => {
                if self.status == Level::High {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::Low;
                        self.count = 0;
                        is_change = true;
                    }
                } else {
                    self.count = 0;
                }
            }
            Level::High => {
                if self.status == Level::Low {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::High;
                        self.count = 0;
                        is_change = true;
                    }
                } else {
                    self.count = 0;
                }
            }
        }
        is_change
    }

    pub fn status(&self) -> Level {
        self.status
    }
}

