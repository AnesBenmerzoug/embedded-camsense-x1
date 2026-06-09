// States
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header<const N: u8>;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Data(pub u8);

// State Machine

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine<S> {
    state: S
}

impl StateMachine<Header<0x55>> {
    fn new() -> Self {
        Self { state: Header::<0x55> }
    }
}

impl StateMachine<Data> {
    pub fn first_byte(&self) -> u8 {
        self.state.0
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

impl From<(StateMachine::<Header::<0x08>>, u8)> for StateMachine::<Data> {
    fn from((_, first_byte): (StateMachine::<Header::<0x08>>, u8)) -> Self {
        Self {
            state: Data(first_byte)
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
            Self::Header08(state) => Self::Data((state, value).into()),
            _ => Self::Header55(StateMachine::new())
        }
    }
}