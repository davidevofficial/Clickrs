use eframe::egui;
use evdev;
use uinput;

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
    let mut activation_key = evdev::KeyCode::KEY_F8;
    let mut spam_key = evdev::KeyCode::BTN_LEFT;
    let mut recording_act = false;
    let mut recording_spam = false;
    // let mut name = "Arthur".to_owned();
    // let mut age = 42;
    let mut frame_n = 0;
    let mut start = false;
    // End of application state

    eframe::run_simple_native("Clickrs", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

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
                        if ui.button(activation_key.code().to_string()).clicked(){
                            recording_act = true;
                        };
                    }
                    if recording_act == true{
                        //Insert get key logic
                        if ui.button("<Input...>").clicked(){
                            recording_act = false;
                        };
                    }
                });
                columns[0].horizontal(|ui|{ //spam btn
                    ui.label("Spam Button: ");
                    if recording_spam == false{
                        if ui.button(spam_key.code().to_string()).clicked(){
                            recording_spam = true;
                        };
                    }
                    if recording_spam == true{
                        //Insert get key logic
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
        frame_n += 1;
    })
}