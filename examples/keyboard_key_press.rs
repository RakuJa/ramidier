use log::{debug, error};
use ramidier::enums::input_group::{KeyboardChannel, KeyboardInputGroup};
use ramidier::enums::message_filter::MessageFilter;
use ramidier::io::input::InputChannel;
use ramidier::io::input_data::MidiInputData;
use std::error::Error;
use std::io::stdin;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => error!("Error: {err}"),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    // Setup MIDI Input
    let midi_in = InputChannel::builder()
        .port(1)
        .msg_to_ignore(MessageFilter::None)
        .build()?;

    let _conn_in = midi_in.listen(
        Some("midir-read-input"),
        move |stamp, rx_data, ()| listener_logic(stamp, &rx_data),
        (),
        KeyboardChannel,
    )?;
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}

pub fn listener_logic(stamp: u64, msg: &MidiInputData<KeyboardInputGroup>) {
    println!("{stamp}: {msg:?}");
    if msg.value > 0 {
        let KeyboardInputGroup::Key(k) = msg.input_group;
        debug!("Key pressed: {k:?}");
    } else {
        let KeyboardInputGroup::Key(k) = msg.input_group;
        debug!("Key released: {k:?}");
    }
}
