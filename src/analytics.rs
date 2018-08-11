use rimd::*;
use std::collections::HashMap;

pub fn construct_probalility_table(a: &SMF) -> HashMap<(u8, u64), Vec<((u8, u64), f32)>>
{
    let mut notes_after_note = HashMap::<(u8, u64), Vec<(u8, u64)>>::new();
    for i in &a.tracks {
        for j in 0..i.events.len() {
            let mut delta_time = 0;
            match &i.events[j].event {
                Event::Midi(first_noteon) => if first_noteon.status() == Status::NoteOn {
                    'berikdiz: for k in (j + 1)..i.events.len() {
                        delta_time += i.events[k].vtime;
                        match &i.events[k].event {
                            Event::Midi(first_noteoff) => if first_noteoff.status()
                                == Status::NoteOff
                                && first_noteon.data(1) == first_noteoff.data(1)
                            {
                                for l in (j + 1)..i.events.len() {
                                    match &i.events[l].event {
                                        Event::Midi(second_noteon) => {
                                            if second_noteon.status() == Status::NoteOn {
                                                let mut delta_time_1 = 0;
                                                for q in (l + 1)..i.events.len() {
                                                    delta_time_1 += i.events[q].vtime;
                                                    match &i.events[q].event {
                                                        Event::Midi(second_noteoff) => {
                                                            if second_noteoff.status()
                                                                == Status::NoteOff
                                                                && second_noteon.data(1)
                                                                    == second_noteoff.data(1)
                                                            {
                                                                notes_after_note
                                                                    .entry((
                                                                        first_noteon.data(1),
                                                                        delta_time,
                                                                    ))
                                                                    .or_insert(vec![])
                                                                    .push((
                                                                        second_noteon.data(1),
                                                                        delta_time_1,
                                                                    ));
                                                                break 'berikdiz;
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
    }
    let mut res = HashMap::<(u8, u64), Vec<((u8, u64), f32)>>::new();
    for (k, v) in notes_after_note {
        let mut tmp = HashMap::<(u8, u64), u64>::new();
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
