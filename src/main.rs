mod midi;
use midi::*;

fn main() {
    println!(
        "{}",
        MidiFile::new(
            "prase.midi".to_string(),
            vec![
                Chunk::Header(HeaderChunk::new(1, 1, 12345)),
                Chunk::Track(TrackChunk::from(vec![
                    Event::Midi(MidiEvent::new(0, 0b10010000, 0, 61, 21)),
                    Event::Midi(MidiEvent::new(142, 0b10000000, 0, 61, 21)),
                    Event::Midi(MidiEvent::new(142, 0b11000000, 0, 0, 0)),
                ])),
            ]
        )
    );
}
