extern crate ramidier;

use ramidier::enums::led_light::color::LedColor;
use ramidier::enums::led_light::mode::LedMode;
use ramidier::io::output::ChannelOutput;
use std::error::Error;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => println!("Error: {err}"),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Setup MIDI Output
    let mut midi_out = ChannelOutput::builder()
        .port(2)
        .initialize_note_led(true)
        .build()?;

    match midi_out.set_all_pads_color(LedMode::On100Percent, LedColor::Green) {
        Ok(()) => println!("Successfully set all pads color"),
        _ => println!("Error communicating with midi output"),
    }

    Ok(())
}
