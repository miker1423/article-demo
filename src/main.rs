#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use embedded_hal::{
  serial,
  blocking::i2c,
};
use ssd1306::{prelude::*, I2CDIBuilder, Builder};
use heapless::String;
use core::fmt::Write;
use modbus_nostd::ModbusClient;

#[cfg(feature = "k64")]
mod k64;

#[cfg(feature = "stm32")]
mod stm32;

#[entry]
fn main() -> !{
    let (mut uart, i2c) = get_peripherals();
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.set_brightness(Brightness::BRIGHTEST);
    disp.flush().unwrap();
    disp.clear().unwrap();
    disp.write_str("Hello");

    let mut buffer = [0u8; 256];
    loop {
        let modbus_client = ModbusClient::new(&mut buffer, 0x52.into());
        let result =
            modbus_client
                .read_coil_from(0x01.into())
                .with_quantity(5)
                .send(&mut uart.0,&mut uart.1);
        delay(10_000);
        disp.clear();
        disp.write_str("Hello!");
    }
}

fn delay(cycles: u32) {
    for _ in 0..cycles {
        cortex_m::asm::nop();
    }
}

fn get_peripherals() -> (
    (impl serial::Write<u8>, impl serial::Read<u8>),
     impl i2c::Write) {
    #[cfg(feature = "k64")]
    return k64::config_board();
    #[cfg(feature = "stm32")]
    return stm32::config_board();
}
