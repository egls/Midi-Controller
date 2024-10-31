use eframe::egui;
use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;

struct MidiController {
    slider_value: u8,
    connection: Option<MidiOutputConnection>,
}

impl Default for MidiController {
    fn default() -> Self {
        let midi_out = MidiOutput::new("My Midi Output").ok();
        let ports = midi_out.as_ref().map(|out| out.ports());

        let connection = if let Some((out, ports)) = midi_out.zip(ports) {
            if !ports.is_empty() {
                out.connect(&ports[0], "midir connection").ok()
            } else {
                None
            }
        } else {
            None
        };

        MidiController {
            slider_value: 64,
            connection,
        }
    }
}

impl eframe::App for MidiController {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl MidiController {
    fn send_midi_message(&mut self) {
        if let Some(connection) = &mut self.connection {
            let midi_message = [0xB0, 0x07, self.slider_value]; // Example CC message
            if let Err(err) = connection.send(&midi_message) {
                eprintln!("Failed to send MIDI message: {}", err);
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Value: {}", self.slider_value));
        if ui.add(egui::Slider::new(&mut self.slider_value, 0..=127).text("Slider")).changed() {
            self.send_midi_message();
        }
        if ui.button("Send MIDI").clicked() {
            self.send_midi_message();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = MidiController::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Rust MIDI Controller", native_options, Box::new(|_| Ok(Box::new(app))))?;
    Ok(())
}
