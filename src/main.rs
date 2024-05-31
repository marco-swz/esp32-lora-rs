/*
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
*/

/// Send and receive a hello world using Heltec's Lora32 (esp32 + sx127x) board
use esp_idf_svc::hal::gpio::Gpio18;
use esp_idf_svc::hal::{
    delay::{Ets, FreeRtos},
    gpio::PinDriver,
    // gpio::Pins,
    prelude::*,
    spi::config::{Config, DriverConfig},
    spi::{SpiDeviceDriver, SpiDriver},
};

const BAND: i64 = 866;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let spi = peripherals.spi2;

    let config = Config::new().baudrate(200.kHz().into());

    let sclk = peripherals.pins.gpio4;
    let miso = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio6;
    let cs = peripherals.pins.gpio7;
    let rst = peripherals.pins.gpio9;

    let driver = SpiDriver::new(spi, sclk, mosi, Some(miso), &DriverConfig::new()).unwrap();
    let device = SpiDeviceDriver::new(driver, Option::<Gpio18>::None, &config).unwrap();

    let lora = sx127x_lora::LoRa::new(
        device,
        PinDriver::output(cs).unwrap(),
        PinDriver::output(rst).unwrap(),
        BAND,
        Ets,
    );

    match lora {
        Ok(_) => println!("lora success"),
        Err(ref x) => println!("error {:?}", x),
    };

    let mut packets_sent: usize = 0;

    let mut lora = lora.unwrap();
    loop {
        let message = format!("Hello Dude {packets_sent}");
        let mut buff = [0u8; 255];
        buff[..message.len()].copy_from_slice(message.as_bytes());
        let transmit = lora.transmit_payload(buff, message.len());

        match transmit {
            Ok(_) => {
                println!("Sent packet {packets_sent}");
                packets_sent += 1;
            }
            Err(e) => println!("Error: {:?}", e),
        }
        let poll = lora.poll_irq(Some(5)); //5 Second timeout
        match poll {
            Ok(size) => {
                println!("with Payload: ");
                let buffer = lora.read_packet().unwrap(); // Received buffer. NOTE: 255 bytes are always returned
                for i in 0..size {
                    print!("{}", buffer[i] as char);
                }
                println!();
            }
            Err(_err) => (),
        }
        FreeRtos::delay_ms(1000);
    }
}
