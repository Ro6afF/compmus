mod midi;
use midi::*;

fn main() {
    let inp = Box::new(MidiFile::read_file("/home/ro6aff/compmus/examples/moonlight_sonata_3.mid".to_string()));
    
    let out = Box::new(MidiFile::new("/home/ro6aff/compmus/examples/moonlight_sonata_33.mid".to_string(), inp.chunks));
    out.write_file();
}
