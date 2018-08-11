extern crate rand;
extern crate rimd;

use rand::*;
use rimd::*;
use std::path;
mod analytics;

fn main() {
    let inp = rimd::SMF::from_file(path::Path::new(
        "/home/ro6aff/compmus_1/examples/moonlight_sonata.mid",
    )).unwrap();
    let plok = analytics::construct_probalility_table(&inp);
    let mut result_b = SMFBuilder::new();
    result_b.add_track();
    result_b.add_event(
        0,
        TrackEvent {
            vtime: 0,
            event: Event::Meta(MetaEvent::tempo_setting(1000000)),
        },
    );
    result_b.add_event(
        0,
        TrackEvent {
            vtime: 0,
            event: Event::Meta(MetaEvent::key_signature(0, 0)),
        },
    );
    result_b.add_event(
        0,
        TrackEvent {
            vtime: 0,
            event: Event::Meta(MetaEvent::time_signature(4, 2, 0x30, 8)),
        },
    );

    let mut rng = thread_rng();
    let mut currn = (62, 86);
    println!("{:?}", plok);
    for _ in 0..100 {
        match currn {
            (a, b) => {
                println!("({}, {})", a, b);
                result_b.add_event(
                    0,
                    TrackEvent {
                        vtime: 0,
                        event: Event::Midi(MidiMessage::note_on(a, 127, 0)),
                    },
                );
                result_b.add_event(
                    0,
                    TrackEvent {
                        vtime: b,
                        event: Event::Midi(MidiMessage::note_off(a, 0, 0)),
                    },
                );
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
//    println!("{:#?}", &result_b.result());
    SMFWriter::from_smf(result_b.result()).write_to_file(path::Path::new("/home/ro6aff/compmus_1/examples/moonlight_sonata_result.mid")).expect("ops");
}
