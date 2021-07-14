#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[cfg(feature = "k64")]
use cortex_m_semihosting::hprintln;
use embedded_hal::{
  serial,
  blocking::i2c,
};
use ssd1306::{prelude::*, I2CDIBuilder, Builder};
use core::
{
    fmt::Write,
    sync::atomic::{
        AtomicBool,
        AtomicU16,
        Ordering
    }
};
use modbus_nostd::ModbusClient;

#[cfg(feature = "k64")]
mod k64;

#[cfg(feature = "stm32")]
mod stm32;

static VALUE_84: AtomicU16 = AtomicU16::new(0);
static VALUE_85: AtomicU16 = AtomicU16::new(0);
static VALUE_86: AtomicU16 = AtomicU16::new(50);
static WIRE_90: AtomicBool = AtomicBool::new(false);
static WIRE_91: AtomicBool = AtomicBool::new(false);
static WIRE_92: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> !{
    let ((mut uart_tx, mut uart_rx), i2c) = get_peripherals();

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.clear().unwrap();

    let mut buffer = [0u8; 256];
    loop {
        let modbus_client = ModbusClient::new(&mut buffer, 0x05.into());
        let mut read_a = modbus_client.read_register_from(0x84.into()).with_quantity(2);
        let result_a = read_a.send(&mut uart_tx, &mut uart_rx);

        if let Ok((size, data)) = result_a {
            if size != 0 {
                let value_84: u16 = (data[3] as u16) << 8 | data[4] as u16;
                let value_85: u16 = (data[5] as u16) << 8 | data[6] as u16;
                writeln!(disp, "value 84 {}", value_84);
                writeln!(disp, "value 85 {}", value_85);
            }
        }
        /*
        let modbus_client = ModbusClient::new(&mut buffer, 0x52.into());
        let mut read_b = modbus_client.read_coil_from(0x90.into()).with_quantity(3);
        let result_b = read_b.send(&mut uart_tx, &mut uart_rx);

        if let Ok((size, data)) = result_b {
            if size == 0 {
            } else {
                let value = data[2];
                WIRE_90.store(value & 0b1 == 1, Ordering::Relaxed);
                WIRE_91.store(value & 0b10 == 1, Ordering::Relaxed);
                WIRE_92.store(value & 0b100 == 1, Ordering::Relaxed);
            }
        }

        let modbus_client = ModbusClient::new(&mut buffer, 0x52.into());
        let value = VALUE_86.load(Ordering::Relaxed);
        if value == u16::MAX {
            VALUE_86.store(0, Ordering::Relaxed);
        } else {
            let _ = VALUE_86.fetch_add(1, Ordering::Relaxed);
        }
        let data = [ value ];
        let mut write_a = modbus_client.write_registers_from(0x86.into());
        let result_c = write_a.send(&data, &mut uart_tx, &mut uart_rx);

        let v1 = VALUE_84.load(Ordering::Relaxed);
        let v2 = VALUE_85.load(Ordering::Relaxed);
        let v3 = VALUE_86.load(Ordering::Relaxed);
        let w1 = if WIRE_90.load(Ordering::Relaxed) { '1' } else { '0' };
        let w2 = if WIRE_91.load(Ordering::Relaxed) { '1' } else { '0' };
        let w3 = if WIRE_92.load(Ordering::Relaxed) { '1' } else { '0' };
        //write!(disp, "R84: {}, R85: {}, R86: {}\nW90: {} , W91: {} , W92 {}", v1, v2, v3, w1, w2, w3);
        */
        delay(10_000);
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
