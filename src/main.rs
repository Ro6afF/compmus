extern crate rand;
mod analytics;
mod midi;
use midi::*;
use rand::prelude::*;

fn main() {
    let inp = MidiFile::read_file("/home/ro6aff/compmus/examples/moonlight_sonata.mid".to_string());
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
    for _ in 0..10 {
        println!("{}", NOTES[currn as usize]);
        let mut blqh: f32 = rng.gen();
        let mut currc = 0.0;
        for i in &plok[&currn] {
            match i {
                (a, b) => {
                    currc += b;
                    if currc <= blqh {
                        currn = *a;
                        break;
                    }
                }
            }
        }
    }
}
