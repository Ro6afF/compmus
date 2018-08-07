extern crate rand;
mod analytics;
mod midi;
use midi::*;
use rand::prelude::*;
use std::fs;

fn main() {
    let inp = MidiFile::read_file("/home/ro6aff/compmus/examples/bach_tocatta_fugue_d_minor.mid".to_string());
    println!("{}\n---------------------------", inp);
    let plok = analytics::construct_probalility_table(&inp);
    for (k, v) in &plok {
        println!("{}:", NOTES[*k as usize]);
        for i in v {
            match i {
                (a, b) => println!("\t{} -> {}", NOTES[*a as usize], b),
            }
        }
    }
    println!("----------------------------");
    let mut currn: u8 = 60;
    let mut rng = thread_rng();
    let mut chunks = vec![];
    chunks.push(Chunk::Header(HeaderChunk::new(1, 1, 16)));
    let mut events = vec![];
    events.push(Event::Meta(MetaEvent::new(0, 0x51, 03, vec![0xf, 0x42, 0x40])));
    events.push(Event::Meta(MetaEvent::new(0, 0x58, 04, vec![4, 2, 48, 8])));
    for _ in 0..100 {
        println!("{}", NOTES[currn as usize]);
        events.push(Event::Midi(MidiEvent::new(0, 0b10010000, 0, currn, 127)));
        events.push(Event::Midi(MidiEvent::new(8, 0b10000000, 0, currn, 0)));
        let mut blqh: f32 = rng.gen();
        let mut currc = 0.0;
        for i in &plok[&currn] {
            match i {
                (a, b) => {
                    currc += b;
                    println!("{} <= {}", currc, blqh);
                    if  blqh <= currc {
                        currn = *a;
                        break;
                    }
                }
            }
        }
    }
    events.push(Event::Meta(MetaEvent::new(0, 0x2f, 00, vec![])));
    chunks.push(Chunk::Track(TrackChunk::from(events)));
    let res = MidiFile::new("/home/ro6aff/compmus/examples/result_2.mid".to_string(), chunks);
    res.write_file();
}
