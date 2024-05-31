use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, peripherals::Peripherals, spi};
use esp_idf_svc::hal::prelude::*;
use sx127x_lora;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let config = spi::config::DriverConfig::new();

    let spi_driver = spi::SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        Some(peripherals.pins.gpio6),
        &config,
    ).unwrap();

    let cs = PinDriver::output(peripherals.pins.gpio7).unwrap();
    let reset = PinDriver::output(peripherals.pins.gpio8).unwrap();
    let dr = spi::SpiDeviceDriver::new(
        spi_driver,
        None,
        &spi::config::Config::new(),
    );
    sx127x_lora::LoRa::new(
        dr,
        cs,
        reset,
        915,
        Delay::new_default(),
    );

    log::info!("Hello, world!");
}
