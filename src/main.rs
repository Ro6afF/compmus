extern crate rand;
mod analytics;
mod midi;
use midi::*;
use rand::prelude::*;

fn main() {
    let inp =
        MidiFile::read_file("/home/ro6aff/compmus/examples/chopin_nocturne_9_2.mid".to_string());
    println!("{}\n---------------------------", inp);
    let plok = analytics::construct_probalility_table(&inp);
    let mut currn = (52, 55);
    println!("{:?}", plok);
    for ((a, b), v) in &plok {
        println!("{} ; {} ->", NOTES[*a as usize], NOTES[*b as usize]);
        for (x, y) in v {
            println!("\t{} -> {}", NOTES[*x as usize], y)
        }
    }
    let mut rng = thread_rng();
    println!("----------------------------");
    let mut chunks = vec![];
    chunks.push(Chunk::Header(HeaderChunk::new(1, 1, 16)));
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
            (a, _) => {
                events.push(Event::Midi(MidiEvent::new(0, 0b10010000, 0, a, 127)));
                events.push(Event::Midi(MidiEvent::new(8, 0b10000000, 0, a, 0)));
            }
        }
        let mut blqh: f32 = rng.gen();
        let mut currc = 0.0;
        println!("{:?}", currn);
        if (&plok).contains_key(&currn) {
            for i in &plok[&currn] {
                match i {
                    (a, b) => {
                        currc += b;
                        if blqh <= currc {
                            let ln;
                            match currn {
                                (_, w) => ln = w,
                            }
                            currn = (ln, *a);
                            break;
                        }
                    }
                }
            }
        } else {
            let mut currn1 = (0, 0);
            for ((a, _), _) in &plok {
                match &currn {
                    (x, y) => if a == x {
                        currn1 = (*x, *y);
                    },
                    _ => panic!(),
                }
            }
            currn = currn1;
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
