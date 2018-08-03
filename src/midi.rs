use std::fmt;
use std::path::Path;

const notes: [&'static str; 128] = [
    "C - 0", "C# - 0", "D - 0", "D# - 0", "E - 0", "F - 0", "F# - 0", "G - 0", "G# - 0", "A - 0", "A# - 0", "B - 0",
    "C - 1", "C# - 1", "D - 1", "D# - 1", "E - 1", "F - 1", "F# - 1", "G - 1", "G# - 1", "A - 1", "A# - 1", "B - 1",
    "C - 2", "C# - 2", "D - 2", "D# - 2", "E - 2", "F - 2", "F# - 2", "G - 2", "G# - 2", "A - 2", "A# - 2", "B - 2",
    "C - 3", "C# - 3", "D - 3", "D# - 3", "E - 3", "F - 3", "F# - 3", "G - 3", "G# - 3", "A - 3", "A# - 3", "B - 3",
    "C - 4", "C# - 4", "D - 4", "D# - 4", "E - 4", "F - 4", "F# - 4", "G - 4", "G# - 4", "A - 4", "A# - 4", "B - 4",
    "C - 5", "C# - 5", "D - 5", "D# - 5", "E - 5", "F - 5", "F# - 5", "G - 5", "G# - 5", "A - 5", "A# - 5", "B - 5",
    "C - 6", "C# - 6", "D - 6", "D# - 6", "E - 6", "F - 6", "F# - 6", "G - 6", "G# - 6", "A - 6", "A# - 6", "B - 6",
    "C - 7", "C# - 7", "D - 7", "D# - 7", "E - 7", "F - 7", "F# - 7", "G - 7", "G# - 7", "A - 7", "A# - 7", "B - 7",
    "C - 8", "C# - 8", "D - 8", "D# - 8", "E - 8", "F - 8", "F# - 8", "G - 8", "G# - 8", "A - 8", "A# - 8", "B - 8",
    "C - 9", "C# - 9", "D - 9", "D# - 9", "E - 9", "F - 9", "F# - 9", "G - 9", "G# - 9", "A - 9", "A# - 9", "B - 9",
    "C - 10", "C# - 10", "D - 10", "D# - 10", "E - 10", "F - 10", "F# - 10", "G - 10"
];

const instruments: [&'static str; 128] = [
    "Acoustic Grand",
    "Bright Acoustic",
    "Electric Grand",
    "Honky-Tonk",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clav",
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accoridan",
    "Harmonica",
    "Tango Accordian",
    "Acoustic Guitar(nylon)",
    "Acoustic Guitar(steel)",
    "Electric Guitar(jazz)",
    "Electric Guitar(clean)",
    "Electric Guitar(muted)",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    "Acoustic Bass",
    "Electric Bass(finger)",
    "Electric Bass(pick)",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Strings",
    "Timpani",
    "String Ensemble 1",
    "String Ensemble 2",
    "SynthStrings 1",
    "SynthStrings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "SynthBrass 1",
    "SynthBrass 2",
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Skakuhachi",
    "Whistle",
    "Ocarina",
    "Lead 1 (square)",
    "Lead 2 (sawtooth)",
    "Lead 3 (calliope)",
    "Lead 4 (chiff)",
    "Lead 5 (charang)",
    "Lead 6 (voice)",
    "Lead 7 (fifths)",
    "Lead 8 (bass+lead)",
    "Pad 1 (new age)",
    "Pad 2 (warm)",
    "Pad 3 (polysynth)",
    "Pad 4 (choir)",
    "Pad 5 (bowed)",
    "Pad 6 (metallic)",
    "Pad 7 (halo)",
    "Pad 8 (sweep)",
    "FX 1 (rain)",
    "FX 2 (soundtrack)",
    "FX 3 (crystal)",
    "FX 4 (atmosphere)",
    "FX 5 (brightness)",
    "FX 6 (goblins)",
    "FX 7 (echoes)",
    "FX 8 (sci-fi)",
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bagpipe",
    "Fiddle",
    "Shanai",
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot"
];

fn midi_event_type_to_string(a: u8) -> String {
    if a >> 4 == 0b1000 {
        String::from("NOTE OFF")
    }
    else if a >> 4 == 0b1001 {
        String::from("NOTE ON")
    }
    else if a >> 4 == 0b1010 {
        String::from("POLYPHONIC KEY PRESSURE")
    }
    else if a >> 4 == 0b1011 {
        String::from("CONTROL CHANGE")
    }
    else if a >> 4 == 0b1100 {
        String::from("PROGRAM CHANGE")
    }
    else if a >> 4 == 0b1101 {
        String::from("CHANNEL PRESSURE")
    }
    else if a >> 4 == 0b1110 {
        String::from("PITCH WHEEL CHANGE")
    } else {
        panic!("UNKNOWN EVENT");
    }
}

fn var_len_enc(mut val: u32) -> Vec<u8> {
    let mut ans = vec![];
    let mut buff: u32 = val & 0x7f;
    while {val >>= 7; val > 0} {
        buff <<= 8;
        buff |= 0x80;
        buff += val & 0x7f;
    }
    loop {
        ans.push((buff & 0xff) as u8);
        if buff & 0x80 != 0 {
            buff >>= 8;
        } else {
            break;
        }
    }
    ans
}

pub struct MidiEvent {
    delta_time: u32,
    message_type: u8,
    channel: u8,
    data1: u8,
    data2: u8
}

impl MidiEvent {
    pub fn new(delta_time: u32, message_type: u8, channel: u8, data1: u8, data2: u8) -> MidiEvent {
        MidiEvent {
            delta_time: delta_time,
            message_type: message_type,
            channel: channel,
            data1: data1,
            data2: data2
        }
    }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let mut res = var_len_enc(self.delta_time);
        res.push(self.message_type + self.channel);
        res.push(self.data1);
        if !(self.message_type == 0b11000000|| self.delta_time == 0b11010000) {
            res.push(self.data2);
        }

        res
    }
}

impl fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message_type >> 4 == 0b1000 || self.message_type >> 4 == 0b1001 || self.message_type >> 4 == 0b1010 {
            write!(f, "DELTA TIME {} FOR {} AT CHANNEL {} WITH DATA1 {} AND DATA2 {}", self.delta_time, midi_event_type_to_string(self.message_type), self.channel, notes[self.data1 as usize], self.data2)
        } else if self.message_type >> 4 == 0b1100 {
            write!(f, "DELTA TIME {} FOR {} AT CHANNEL {} WITH DATA1 {}", self.delta_time, midi_event_type_to_string(self.message_type), self.channel, instruments[self.data1 as usize])
        } else {
            write!(f, "DELTA TIME {} FOR {} AT CHANNEL {} WITH DATA1 {} AND DATA2 {}", self.delta_time, midi_event_type_to_string(self.message_type), self.channel, self.data1, self.data2)
        }
    }
}

pub struct SysEvent {
    delta_time: u32,
    message: u8
}

impl SysEvent {
    pub fn new(delta_time: u32, message: u8) -> SysEvent {
        SysEvent {
            delta_time: delta_time,
            message: message
        }
    }

}

impl fmt::Display for SysEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELTA TIME {} FOR {} EVENT", self.delta_time, match self.message {
            0b11111010 => "START",
            0b11111011 => "CONTINUE",
            0b11111100 => "STOP",
            0b11111111 => "RESET",
            _ => panic!("UNKNOWN EVENT")
        })
    }
}

pub struct MetaEvent {
    delta_time: u32,
    message_type: u8,
    length: u64,
    bytes: Vec<u8>
}

impl MetaEvent {
    pub fn new(delta_time: u32, message_type: u8, length: u64, bytes: Vec<u8>) -> MetaEvent {
        MetaEvent {
            delta_time: delta_time,
            message_type: message_type,
            length: length,
            bytes: bytes
        }
    }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let mut res = var_len_enc(self.delta_time);
        res.push(self.message_type);
        for i in self.bytes.iter() {
            res.push(*i);
        }

        res
    }
}

impl fmt::Display for MetaEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.message_type {
            0x01 => write!(f, "DELTA TIME {} FOR TEXT META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x02 => write!(f, "DELTA TIME {} FOR COPYRIGHT META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x03 => write!(f, "DELTA TIME {} FOR SEQUENCE/TRACK NAME META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x04 => write!(f, "DELTA TIME {} FOR INSTRUMENT NAME META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x05 => write!(f, "DELTA TIME {} FOR LYRICS META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x06 => write!(f, "DELTA TIME {} FOR MARKER META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x07 => write!(f, "DELTA TIME {} FOR CUE POINT META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes.iter().map(|x| *x as char).collect::<String>()),
            0x20 => write!(f, "DELTA TIME {} FOR MIDI CHANNEL PREFIX META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes[0]),
            0x2f => write!(f, "DELTA TIME {} FOR END OF TRACK META EVENT", self.delta_time),
            0x51 => write!(f, "DELTA TIME {} FOR TEMPO META EVENT WITH LENGTH {} AND DATA {}", self.delta_time, self.length, self.bytes[0] as u32 * 256 * 256 + self.bytes[1] as u32 * 256 + self.bytes[2] as u32),
            0x59 => write!(f, "DELTA TIME {} FOR KEY SIGNATURE META EVENT WITH LENGTH {} AND IT IS {} {}", self.delta_time, self.length, if (self.bytes[0] as i8) < 0 {format!("{} FLAT(S)", -(self.bytes[0] as i8))} else {format!("{} SHARP(S)", self.bytes[0])}, if self.bytes[1] == 0 {"MAJOR"} else {"MINOR"} ),
            _ => panic!()
        }
    }
}

pub enum Event {
    Midi(MidiEvent),
    Meta(MetaEvent)
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Midi(x) => x.fmt(f),
            Event::Meta(x) => x.fmt(f)
        }
    }
}

pub struct HeaderChunk {
    file_type: u16,
    ntrks: u16,
    division: u16
}

impl HeaderChunk {
    pub fn new(file_type: u16, ntrks: u16, division: u16) -> HeaderChunk {
        HeaderChunk {
            file_type: file_type,
            ntrks: ntrks,
            division: division
        }
    }
}

impl fmt::Display for HeaderChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEADER CHUNK:\n\t{} {} {}", self.file_type, self.ntrks, self.division)
    }
}

pub struct TrackChunk {
    events: Vec<Event>
}

impl TrackChunk {
    pub fn new() -> TrackChunk {
        TrackChunk {
            events: vec![]
        }
    }

    pub fn from(events: Vec<Event>) -> TrackChunk {
        TrackChunk {
            events: events
        }
    }
}

impl fmt::Display for TrackChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TACK CHUNK:");
        for i in &self.events {
            write!(f, "\n\t{}", i);
        }
        write!(f, "\n")
    }
}

pub enum Chunk {
    Header(HeaderChunk),
    Track(TrackChunk)
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chunk::Header(x) => x.fmt(f),
            Chunk::Track(x) => x.fmt(f)
        }
    }
}

pub struct File {
    file_name: String,
    chunks: Vec<Chunk>
}

impl File {
    pub fn new(file_name: String, chunks: Vec<Chunk>) -> File {
        if Path::new(&file_name).exists() {
            panic!("This file exists!");
        }
        File {
            file_name: file_name,
            chunks: chunks
        }
    }

    pub fn read_file(file_name: String) -> File {
        unimplemented!();
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FILE {}:", self.file_name);
        for i in &self.chunks {
            write!(f, "\n\t{}", i);
        }
        write!(f, "\n")
    }
}