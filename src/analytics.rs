use midi::*;
use std::collections::HashMap;

pub fn construct_probalility_table(a: &MidiFile) -> HashMap<(u8, u8), Vec<(u8, f32)>> {
    let mut res1 = HashMap::<(u8, u8), Vec<u8>>::new();
    for i in &a.chunks {
        match i {
            Chunk::Track(x) => {
                for j in 0..x.events.len() {
                    match &x.events[j] {
                        Event::Midi(y) => {
                            if y.message_type >> 4 == 0b1001 {
                                'second: for k in (j + 1)..x.events.len() {
                                    match &x.events[k] {
                                        Event::Midi(z) => if z.message_type >> 4 == 0b1001 {
                                            for l in (k + 1)..x.events.len() {
                                                match &x.events[l] {
                                                    Event::Midi(q) => {
                                                        if q.message_type >> 4 == 0b1001 {
                                                            (*res1
                                                                .entry((y.data1, z.data1))
                                                                .or_insert(Vec::<u8>::new()))
                                                                .push(q.data1);
                                                            break 'second;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    let mut res = HashMap::<(u8, u8), Vec<(u8, f32)>>::new();
    for (k, v) in res1 {
        let mut tmp = HashMap::<u8, u8>::new();
        let mut cnt: f32 = 0.0;
        for i in v {
            *tmp.entry(i).or_insert(0) += 1;
            cnt += 1.0;
        }
        let mut temp = vec![];
        for (k1, v1) in tmp {
            temp.push((k1, v1 as f32 / cnt));
        }
        res.insert(k, temp);
    }

    res
}
