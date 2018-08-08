extern crate rand;
mod analytics;
mod midi;
use midi::*;
use rand::prelude::*;

fn main() {
    let inp = MidiFile::read_file("/home/ro6aff/compmus/examples/bach_tocatta_fugue_d_minor.mid".to_string());
    println!("{}\n---------------------------", inp);
    let plok = analytics::construct_probalility_table(&inp);
    let mut currn = (61, 512);
    for (k, v) in &plok {
        match k {
            (x, z) => {
                println!("({}, {}):", NOTES[*x as usize], z);
                //currn = (*x, *z)
            }
        }
        for i in v {
            match i {
                ((a, b), c) => println!("\t({}, {}) -> {}", NOTES[*a as usize], b, c),
            }
        }
    }
    let mut rng = thread_rng();
    println!("----------------------------");
    let mut chunks = vec![];
    chunks.push(Chunk::Header(HeaderChunk::new(1, 1, 256)));
    let mut events = vec![];
    events.push(Event::Meta(MetaEvent::new(
        0,
        0x51,
        03,
        vec![0xf, 0x42, 0x40],
    )));
    events.push(Event::Meta(MetaEvent::new(0, 0x58, 04, vec![4, 2, 48, 8])));
    for _ in 0..100 {
        match currn {
            (a, b) => {
                println!("{}, {}", NOTES[a as usize], b);
                events.push(Event::Midi(MidiEvent::new(0, 0b10010000, 0, a, 127)));
                events.push(Event::Midi(MidiEvent::new(b, 0b10000000, 0, a, 0)));
            }
        }
        let mut blqh: f32 = rng.gen();
        let mut currc = 0.0;
        for i in &plok[&currn] {
            match i {
                (a, b) => {
                    currc += b;
                    if blqh <= currc {
                        currn = *a;
                        break;
                    }
                }
            }
        }
    }
    events.push(Event::Meta(MetaEvent::new(0, 0x2f, 00, vec![])));
    chunks.push(Chunk::Track(TrackChunk::from(events)));
    let res = MidiFile::new(
        "/home/ro6aff/compmus/examples/result_2.mid".to_string(),
        chunks,
    );
    res.write_file();
}
