mod midi;
use midi::*;

fn main() {
    println!(
        "{}",
        MidiFile::read_file("/home/ro6aff/compmus/examples/alla_turca.mid".to_string())
    );
}
