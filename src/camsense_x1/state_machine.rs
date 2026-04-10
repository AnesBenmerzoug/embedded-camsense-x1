// States
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header<const N: u8>;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Data;

// State Machine

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine<S> {
    state: S
}

impl StateMachine<Header<0x55>> {
    fn new() -> Self {
        Self { state: Header::<0x55> }
    }
}

impl From<StateMachine::<Header::<0x55>>> for StateMachine::<Header::<0xAA>> {
    fn from(_a: StateMachine::<Header::<0x55>>) -> Self {
        Self {
            state: Header::<0xAA>
        }
    }
}

impl From<StateMachine::<Header::<0xAA>>> for StateMachine::<Header::<0x03>> {
    fn from(_a: StateMachine::<Header::<0xAA>>) -> Self {
        Self {
            state: Header::<0x03>
        }
    }
}

impl From<StateMachine::<Header::<0x03>>> for StateMachine::<Header::<0x08>> {
    fn from(_a: StateMachine::<Header::<0x03>>) -> Self {
        Self {
            state: Header::<0x08>
        }
    }
}

impl From<StateMachine::<Header::<0x08>>> for StateMachine::<Data> {
    fn from(_a: StateMachine::<Header::<0x08>>) -> Self {
        Self {
            state: Data
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateMachineWrapper {
    Header55(StateMachine::<Header::<0x55>>),
    HeaderAA(StateMachine::<Header::<0xAA>>),
    Header03(StateMachine::<Header::<0x03>>),
    Header08(StateMachine::<Header::<0x08>>),
    Data(StateMachine::<Data>),
}

impl StateMachineWrapper {
    pub fn new() -> Self {
        Self::Header55(StateMachine::new())
    }

    pub fn step(self, value: u8) -> Self {
        match self {
            Self::Header55(state) if value == 0xAA => Self::HeaderAA(state.into()),
            Self::HeaderAA(state) if value == 0x03 => Self::Header03(state.into()),
            Self::Header03(state) if value == 0x08 => Self::Header08(state.into()),
            Self::Header08(state) => Self::Data(state.into()),
            _ => Self::Header55(StateMachine::new())
        }
    }
}