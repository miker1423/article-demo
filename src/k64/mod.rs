use k64f_hal::{
    pac,
    i2c,
    uart,
    gpio::GpioExt,
};
use embedded_hal::{
    serial,
    blocking::i2c as hal_i2c,
};

pub fn config_board() -> (
    (impl serial::Write<u8>, impl serial::Read<u8>),
     impl hal_i2c::Write) {
    let p = pac::Peripherals::take().unwrap();
    let port_b = p.PORTB.split();
    let uart_pins = cortex_m::interrupt::free(move |cs|
        (
            port_b.pb17.into_alternate_af3(cs),
            port_b.pb16.into_alternate_af3(cs),
        )
    );
    let config = uart::Config::new(
        9600.into(),
        uart::Parity::None,
        uart::WordLength::DataBits8,
        uart::StopBits::Stop1
    );
    let serial = uart::Serial::uart0(p.UART0, uart_pins, &config, &p.SIM);
    let port_e = p.PORTE.split();
    let i2c_pins = cortex_m::interrupt::free(move |cs|
        (
            port_e.pe24.into_af5_outputdrain(cs),
            port_e.pe25.into_af5_outputdrain(cs)
        )
    );
    let i2c = i2c::I2c::new(p.I2C0, i2c_pins, 0, &p.SIM);
    (serial.split(), i2c)
}