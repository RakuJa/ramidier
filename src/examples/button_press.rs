use ramidier::enums::button::input_group::InputGroup;
use ramidier::enums::led_light::color::LedColor;
use ramidier::enums::led_light::mode::LedMode;
use ramidier::enums::message_filter::MessageFilter;
use ramidier::io::input::InputChannel;
use ramidier::io::input_data::MidiInputData;
use ramidier::io::output::ChannelOutput;
use std::error::Error;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => println!("Error: {err}"),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Setup MIDI Input
    let midi_in = InputChannel::builder()
        .port(2)
        .msg_to_ignore(MessageFilter::None)
        .build()?;

    // Setup MIDI Output
    let mut midi_out = ChannelOutput::builder()
        .port(2)
        .initialize_note_led(true)
        .build()?;

    midi_out
        .set_all_pads_color(LedMode::On100Percent, LedColor::Green)?;

    let _conn_in = midi_in.listen(
        Some("midir-read-input"),
        move |stamp, rx_data, ()| listener_logic(&mut midi_out, stamp, &rx_data),
        (),
    )?;
    Ok(())
}

pub fn listener_logic(midi_out: &mut ChannelOutput, stamp: u64, msg: &MidiInputData) {
    println!("{stamp}: {msg:?}");
    if msg.value > 0 {
        if let InputGroup::Pads(k) = msg.input_group {
            let _ = midi_out.set_pad_led(LedMode::Blinking1over2, k, LedColor::Green);
        }
    } else if let InputGroup::Pads(k) = msg.input_group {
        let _ = midi_out.set_pad_led(LedMode::Blinking1over2, k, LedColor::Off);
    }
}
