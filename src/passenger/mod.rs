use crate::station::types::StationKind;

pub struct Passenger {
    kind: StationKind
}

impl Passenger {
    pub fn new(kind: StationKind) -> Self {
        Passenger { kind }
    }

    pub fn kind(&self) -> StationKind {
        self.kind
    }
}