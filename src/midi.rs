use std::fmt;
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
        panic!();
    }
}

pub struct MidiEvent {
    delta_time: u32,
    message_type: u8,
    channel: u8,
    data1: u8,
    data2: u8
}

impl MidiEvent {
    pub fn new() -> MidiEvent {
        MidiEvent {
            delta_time: 0,
            message_type: 0,
            channel: 0,
            data1: 0,
            data2: 0
        }
    }
}

impl fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELTA TIME {} FOR {} AT CHANNEL {} WITH DATA1 {} AND DATA2 {}", self.delta_time, midi_event_type_to_string(self.message_type), self.channel, self.data1, self.data2)
    }
}

pub struct SysexEvent {
    delta_time: u32,
    message: u8
}

impl SysexEvent {
    pub fn new() -> SysexEvent {
        SysexEvent {
            delta_time: 0,
            message: 0
        }
    }
}

impl fmt::Display for SysexEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELTA TIME {} FOR {} EVENT", self.delta_time, match self.message {
            0b11111010 => "START",
            0b11111011 => "CONTINUE",
            0b11111100 => "STOP",
            0b11111111 => "RESET",
            _ => panic!()
        })
    }
}

pub struct MetaEvent {
    delta_time: u32,
    message_type: u8,
    length: u64,
    bytes: Box<[u8]>
}

impl MetaEvent {
    pub fn new() -> MetaEvent {
        MetaEvent {
            delta_time: 0,
            message_type: 0,
            length: 0,
            bytes: Box::new([])
        }
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