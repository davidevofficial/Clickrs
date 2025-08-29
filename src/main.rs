use eframe::egui;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([750.0, 500.0]),
        ..Default::default()
    };

    // Our application state:
    // Interval
    let mut interval_ms = 50;
    let mut interval_s = 0;
    let mut interval_m = 0;
    let mut interval_hold_ms = 0; //ms to hold a button
    let mut interval_random_delta = 0; // +- interval in ms
    let mut times_to_repeat = 0;
    let mut times_repeated = 0;
    // Whether the clicker is toggable or heldable
    let mut hold = true;
    //Keys
    let mut activation_key = EventCode::EV_KEY(EV_KEY::KEY_F8);
    let mut spam_key = EventCode::EV_KEY(EV_KEY::BTN_LEFT);
    let mut recording_act = false;
    let mut recording_spam = false;
    // let mut name = "Arthur".to_owned();
    // let mut age = 42;
    let mut frame_n = 0;
    let mut start = false;
    let clicker = create_device();
    let mut devices = get_devices();
    let mut key_pressed: Vec<evdev_rs::InputEvent> = Vec::new();
    // End of application state

    eframe::run_simple_native("Clickrs", options, move |ctx, _frame| {

        egui::CentralPanel::default().show(ctx, |ui| {
            key_pressed = get_key_pressed(&devices);
            ctx.style_mut(|style|{
                style.override_font_id = Some(egui::FontId::proportional(24.0)); // 24 pt size globally
            });

            ui.heading("Clickrs @ DavidevOfficial @ 2025");
            ui.add_space(20.0);
            ui.label( frame_n.to_string());

            ui.label("Click Interval");
            ui.separator();
            ui.columns(3, |columns|{

                let mut s_m = String::from(interval_m.to_string());
                columns[0].text_edit_singleline(&mut s_m);
                if s_m == String::new(){ interval_m = 0; }
                else { interval_m = s_m.parse().expect("Invalid Interval Time"); }
                columns[0].label("Minutes");

                let mut s_s = String::from(interval_s.to_string());
                columns[1].text_edit_singleline(&mut s_s);
                if s_s == String::new(){ interval_s = 0; }
                else { interval_s = s_s.parse().expect("Invalid Interval Time"); }
                columns[1].label("Seconds");

                let mut s_ms = String::from(interval_ms.to_string());
                columns[2].text_edit_singleline(&mut s_ms);
                if s_ms == String::new(){ interval_ms = 0; }
                else { interval_ms = s_ms.parse().expect("Invalid Interval Time"); }
                columns[2].label("Milliseconds");

            });

            ui.add_space(20.0);
            ui.columns(2, |columns|{

                // Left Column
                columns[0].label("Options");
                columns[0].separator();
                columns[0].horizontal(|ui|{ //act btn
                    ui.label("Activation Button: ");
                    if recording_act == false{
                        if ui.button(activation_key.to_string()).clicked(){
                            recording_act = true;
                        };
                    }
                    if recording_act == true{
                        //Insert get key logic
                        if key_pressed.len() > 0{
                            for x in &key_pressed{
                                if x.is_type(&EventType::EV_KEY){
                                    activation_key = x.event_code;
                                    recording_act = false;
                                }
                            }
                        }
                        if ui.button("<Input...>").clicked(){
                            recording_act = false;
                        };
                    }
                });
                columns[0].horizontal(|ui|{ //spam btn
                    ui.label("Spam Button: ");
                    if recording_spam == false{
                        if ui.button(spam_key.to_string()).clicked(){
                            recording_spam = true;
                        };
                    }
                    if recording_spam == true{
                        //Insert get key logic
                        if key_pressed.len() > 0{
                            for x in &key_pressed{
                                if x.is_type(&EventType::EV_KEY){
                                    spam_key = x.event_code;
                                    recording_spam = false;
                                }
                            }
                        }
                        if ui.button("<Input...>").clicked(){
                            recording_spam = false;
                        };
                    }
                });
                columns[0].checkbox(&mut hold, "Hold the key to spam?");
                // Right Column
                columns[1].label("More Options");
                columns[1].separator();
                columns[1].horizontal(|ui|{
                    ui.label("Release Key delay (ms):");
                    let mut str = String::from(interval_hold_ms.to_string());
                    ui.text_edit_singleline(&mut str);
                    if str == String::new(){ interval_hold_ms = 0; }
                    else { interval_hold_ms = str.parse().expect("Invalid Interval Time"); }
                });
                columns[1].horizontal(|ui|{
                    ui.label("Release Interval (+/- ms):");
                    let mut str = String::from(interval_random_delta.to_string());
                    ui.text_edit_singleline(&mut str);
                    if str == String::new(){ interval_random_delta = 0; }
                    else { interval_random_delta = str.parse().expect("Invalid Interval Time"); }
                });
                columns[1].horizontal(|ui|{
                    ui.label("Times to repeat (0 = infinite):");
                    let mut str = String::from(times_to_repeat.to_string());
                    ui.text_edit_singleline(&mut str);
                    if str == String::new(){ times_to_repeat = 0; }
                    else { times_to_repeat = str.parse().expect("Invalid Interval Time"); }
                });

            });
            ui.horizontal_centered(|ui|{
                if ui.button("Start").clicked(){
                    start = true;
                }
                if ui.button("Stop").clicked() {
                    start = false;
                }
            });
            if start == false{times_to_repeat = 0;}
        });
        ctx.request_repaint();
        if frame_n % 600 == 0{
            //Refresh list of devices every 600 frames
            devices = get_devices();
        }
        frame_n += 1;
    })
}

use rand::Rng;
use evdev_rs::*;
use evdev_rs::enums::*;
pub fn get_random_number(n: i32)->i32{
    let mut rng = rand::rng();
    rng.random_range(-n..=n)
}
pub fn get_key_pressed(devices: &Vec<evdev_rs::Device>)->Vec<evdev_rs::InputEvent>{
    let mut vector = Vec::new();
    for d in devices.iter(){
        while d.has_event_pending(){
            let f = ReadFlag::NORMAL;
            let ev = d.next_event(f).map(|val| val.1);
            match ev {
                Ok(ev) => {vector.push(ev);
                },
                Err(_e) => (),
            }
        }
    }
    vector
}
use std::path::Path;
use std::fs;
use std::fs::File;
pub fn get_devices() -> Vec<evdev_rs::Device>{
    let mut devices = Vec::new();
    let input_path = Path::new("/dev/input");

    for entry in fs::read_dir(input_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // We only want files starting with "event"
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if !name.starts_with("event") {
                continue;
            }
        } else {
            continue;
        }

        // Open the device file
        match File::open(&path) {
            Ok(file) => {
                match Device::new_from_file(file) {
                    Ok(device) => {
                        devices.push(device);
                    }
                    Err(e) => eprintln!("Failed to create device from {}: {}", path.display(), e),
                }
            }
            Err(e) => eprintln!("Failed to open {}: {}", path.display(), e),
        }
    }
    devices //return
}

pub fn create_device() -> UInputDevice {
    // Create virtual device
    let u = UninitDevice::new().unwrap();

    // Setup device
    // per: https://01.org/linuxgraphics/gfx-docs/drm/input/uinput.html#mouse-movements

    u.set_name("Clickrs");
    u.set_bustype(BusType::BUS_USB as u16);
    u.set_vendor_id(0xabcd);
    u.set_product_id(0xefef);

    // Note mouse keys have to be enabled for this to be detected
    // as a usable device, see: https://stackoverflow.com/a/64559658/6074942
    u.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();
    u.enable(EventCode::EV_KEY(EV_KEY::BTN_RIGHT)).unwrap();

    u.enable(EventCode::EV_REL(EV_REL::REL_X)).unwrap();
    u.enable(EventCode::EV_REL(EV_REL::REL_Y)).unwrap();

    u.enable(EventCode::EV_SYN(EV_SYN::SYN_REPORT)).unwrap();

    // Attempt to create UInputDevice from UninitDevice
    let v = UInputDevice::create_from_device(&u).unwrap();
    v //return ;
}

pub fn send_event(clicker: &UInputDevice, event: EventCode, value: i32){
    let t: TimeVal = std::time::SystemTime::now().try_into().unwrap();
    let _ = clicker.write_event(&InputEvent { time: t, event_code: event, value: value });
}

pub fn click(clicker: &UInputDevice, event: EventCode, hold_delay: i32){
    if hold_delay == 0{
        send_event(clicker, event, 1);
        send_event(clicker, EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
        send_event(clicker, event, 0);
        send_event(clicker, EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
    }else{
        send_event(clicker, event, 1);
        send_event(clicker, EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
        std::thread::sleep(std::time::Duration::from_millis(hold_delay as u64));
        send_event(clicker, event, 0);
        send_event(clicker, EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
    }
}