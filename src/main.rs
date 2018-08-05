mod midi;
use midi::*;

fn main() {
    println!(
        "{}",
        MidiFile::read_file("/home/ro6aff/compmus/examples/moonlight_sonata.mid".to_string())
    );
}
