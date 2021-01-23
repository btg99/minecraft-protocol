use serde::Serialize;
use std::fmt;
use std::fmt::Display;

pub enum State {
    Handshake,
    Status,
    Login,
    Game,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            State::Handshake => "Handshake",
            State::Status => "Status",
            State::Login => "Login",
            State::Game => "Game",
        };

        write!(f, "{}", name)
    }
}

pub enum Bound {
    Server,
    Client,
}

impl Display for Bound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Bound::Server => "Server",
            Bound::Client => "Client",
        };

        write!(f, "{}", name)
    }
}

#[derive(Serialize)]
pub struct Packet {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Packet {
    pub fn new(name: impl ToString, fields: Vec<Field>) -> Packet {
        Packet {
            name: name.to_string(),
            fields,
        }
    }
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    #[serde(flatten)]
    pub data_type: DataType,
}

impl Field {
    pub fn new(name: impl ToString, data_type: DataType) -> Field {
        Field {
            name: name.to_string(),
            data_type,
        }
    }
}

#[derive(Serialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum DataType {
    #[serde(rename(serialize = "bool"))]
    Boolean,
    #[serde(rename(serialize = "i8"))]
    Byte,
    #[serde(rename(serialize = "u8"))]
    UnsignedByte,
    #[serde(rename(serialize = "i16"))]
    Short,
    #[serde(rename(serialize = "u16"))]
    UnsignedShort,
    #[serde(rename(serialize = "i32"))]
    Int {
        var_int: bool,
    },
    #[serde(rename(serialize = "i64"))]
    Long {
        var_long: bool,
    },
    #[serde(rename(serialize = "f32"))]
    Float,
    #[serde(rename(serialize = "f64"))]
    Double,
    String {
        max_length: u16,
    },
    Uuid {
        hyphenated: bool,
    },
    #[serde(rename(serialize = "Vec<u8>"))]
    ByteArray {
        rest: bool,
    },
    CompoundTag,
    RefType {
        ref_name: String,
    },
}

pub struct Protocol {
    pub state: State,
    pub server_bound_packets: Vec<Packet>,
    pub client_bound_packets: Vec<Packet>,
}

impl Protocol {
    pub fn new(
        state: State,
        server_bound_packets: Vec<Packet>,
        client_bound_packets: Vec<Packet>,
    ) -> Protocol {
        Protocol {
            state,
            server_bound_packets,
            client_bound_packets,
        }
    }

    pub fn contains_field_with_type(&self, data_type: DataType) -> bool {
        self.server_bound_packets
            .iter()
            .chain(self.client_bound_packets.iter())
            .flat_map(|p| p.fields.iter())
            .find(|f| f.data_type == data_type)
            .is_some()
    }

    pub fn contains_field_with_predicate<F: Fn(&Field) -> bool>(&self, fun: F) -> bool {
        self.server_bound_packets
            .iter()
            .chain(self.client_bound_packets.iter())
            .flat_map(|p| p.fields.iter())
            .find(|f| fun(*f))
            .is_some()
    }
}
