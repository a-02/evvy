use enigo::*;
use evdev_rs::{Device, enums::EventCode};
use evdev_rs::ReadFlag;

fn main() {
  let tablet = Device::new_from_path("/dev/input/by-id/usb-UGTABLET_10_inch_PenTablet-event-if02").unwrap();

  let mut enigo = Enigo::new();

  loop {
    let tabEvent = tablet.next_event(ReadFlag::NORMAL).map(|val| val.1);
      match tabEvent{
        Ok(ev) => 
          match (ev.event_code, ev.value) {
            (EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_0), 1) => { // nothin much
              println!("i live");
          },
            (EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_1), 1) => { // undo
              println!("hella!");
              enigo.key_down(Key::Control);
              enigo.key_click(Key::Layout('z'));
              enigo.key_up(Key::Control);
          },
            (EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_2), 1) => { // brush
              println!("brushie");
              enigo.key_click(Key::Layout('b'));
          },
            (EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_3), 1) => { // insert and rename layer
              println!("wow");
              enigo.key_click(Key::Insert);
              enigo.key_click(Key::F2);
          },
          _ => ()
        }
      Err(_) => (),
    }
  }
}
