extern crate firmata;
extern crate serial;

use firmata::*;
use serial::*;
use std::thread;

fn main() {
    let mut sp = serial::open("/dev/ttyACM0").unwrap();

    sp.reconfigure(&|settings| {
        settings.set_baud_rate(Baud57600).unwrap();
        settings.set_char_size(Bits8);
        settings.set_parity(ParityNone);
        settings.set_stop_bits(Stop1);
        settings.set_flow_control(FlowNone);
        Ok(())
    }).unwrap();

    let mut b = firmata::Board::new(Box::new(sp)).unwrap();

    let pin = 3;

    println!("firmware version {}", b.firmware_version());
    println!("firmware name {}", b.firmware_name());
    println!("protocol version {}", b.protocol_version());

    let _ = b.set_pin_mode(pin, firmata::SERVO);

    loop {
        for value in 0..180{
            let _ = b.analog_write(pin, value);
            println!("{}", value);
            thread::sleep_ms(10);  //mjh duration needed aot integer when sleep() used
        }
    }
}
