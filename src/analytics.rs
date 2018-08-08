use midi::*;
use std::collections::HashMap;
use std::thread;

pub fn construct_probalility_table(a: &MidiFile) -> HashMap<(u8, u32), Vec<((u8, u32), f32)>> {
    let mut res1 = HashMap::<(u8, u32), Vec<(u8, u32)>>::new();
    for i in &a.chunks {
        match i {
            Chunk::Track(x) => {
                for j in 0..x.events.len() {
                    match &x.events[j] {
                        Event::Midi(y) => {
                            if y.message_type >> 4 == 0b1001 {
                                let mut delta_time = 0;
                                for k in (j + 1)..x.events.len() {
                                    match &x.events[k] {
                                        Event::Midi(z) => {
                                            delta_time += z.delta_time;
                                            if z.message_type >> 4 == 0b1000 && z.data1 == y.data1 {
                                                break;
                                            }
                                        }
                                        Event::Meta(z) => delta_time += z.delta_time,
                                        Event::Sys(z) => delta_time += z.delta_time,
                                    }
                                }

                                'breakdiz: for k in (j + 1)..x.events.len() {
                                    match &x.events[k] {
                                        Event::Midi(z) => if z.message_type >> 4 == 0b1001 {
                                            let mut delta_time_1 = 0;
                                            for l in (k + 1)..x.events.len() {
                                                match &x.events[l] {
                                                    Event::Midi(q) => {
                                                        delta_time_1 += q.delta_time;
                                                        if q.message_type >> 4 == 0b1000
                                                            && q.data1 == z.data1
                                                        {
                                                            (*res1
                                                                .entry((y.data1, delta_time))
                                                                .or_insert(Vec::<(u8, u32)>::new()))
                                                                .push((q.data1, delta_time_1));
                                                            break 'breakdiz;
                                                        }
                                                    }
                                                    Event::Meta(q) => delta_time_1 += q.delta_time,
                                                    Event::Sys(q) => delta_time_1 += q.delta_time,
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
    let mut res = HashMap::<(u8, u32), Vec<((u8, u32), f32)>>::new();
    for (k, v) in res1 {
        let mut tmp = HashMap::<(u8, u32), u32>::new();
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
