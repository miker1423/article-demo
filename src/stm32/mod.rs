use stm32f4xx_hal::{
    pac,
    i2c,
    serial,
    prelude::*,
    gpio::GpioExt
};
use embedded_hal::{
    serial as hal_serial,
    blocking::i2c as hal_i2c,
};
use stm32f4xx_hal::rcc::RccExt;

pub fn config_board() -> (
    (impl hal_serial::Write<u8>, impl hal_serial::Read<u8>),
    impl hal_i2c::Write) {
    let p = pac::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
    let gpio_b = p.GPIOB.split();
    let i2c_pins = (
        gpio_b.pb8.into_alternate_af4().set_open_drain(),
        gpio_b.pb9.into_alternate_af4().set_open_drain(),
    );
    let i2c = i2c::I2c::new(p.I2C1, i2c_pins, 400.khz(), clocks);

    let gpio_a = p.GPIOA.split();
    let uart_pins = (
      gpio_a.pa0.into_alternate_af8(),
      gpio_a.pa1.into_alternate_af8()
    );
    let config = serial::config::Config::default().baudrate(9600.bps());
    let serial = serial::Serial::uart4(
        p.UART4,
        uart_pins,
        config,
        clocks
    ).unwrap();
    (serial.split(), i2c)
}