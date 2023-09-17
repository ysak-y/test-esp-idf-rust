use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() {
    info!("Hello, ESP32!");
    let dp = Peripherals::take().unwrap();
    let mut led = PinDriver::output(dp.pins.gpio5).unwrap();

    FreeRtos::delay_ms(10);
    loop {
        // put your main code here, to run repeatedly:
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
    }
}
