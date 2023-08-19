use enigo::*;
use evdev_rs::{Device, enums::EventCode};
use evdev_rs::ReadFlag;

fn main() {
    let d = Device::new_from_path("/dev/input/by-id/usb-UGTABLET_10_inch_PenTablet-event-if02").unwrap();

    let mut enigo = Enigo::new();

    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);
        match ev{
            Ok(ev) => 
                if ev.event_code == EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_1) {
                    println!("hella!");
                    enigo.key_down(Key::Control);
                    enigo.key_click(Key::Layout('z'));
                    enigo.key_up(Key::Control);
                } else {
                    println!("incorrect");
                }
            Err(_) => (),
        }
    }
}
