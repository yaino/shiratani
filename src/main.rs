mod led;
mod button;

use button::Button;
use led::Led;
use rppal::gpio::Level;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_LED: u8 = 23;
const GPIO_BUTTON: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let mut light = Led::new(GPIO_LED)?;
    let mut light_sw = Button::new(GPIO_BUTTON)?;

    loop {
        if light_sw.observe() {
            match light_sw.status() {
                Level::High => light.light_off(),
                Level::Low => light.light_on(),
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
