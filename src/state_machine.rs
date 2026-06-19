use crate::constants::PAYLOAD_SIZE_IN_BYTES;

// States
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header<const N: u8>;

/// Data collection state. Accumulates incoming bytes into a fixed-size buffer.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Data {
    /// Packet buffer, pre-filled with the 4 header bytes at construction.
    buf: [u8; 36],
    /// Write index into `buf`. Starts at 4 (after the pre-filled header).
    idx: u8,
}

/// Terminal state. Holds a fully received, unvalidated 36-byte packet.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Complete {
    pub buf: [u8; 36],
}

/// Generic state machine wrapper that carries typed state.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StateMachine<S> {
    state: S,
}

impl StateMachine<Header<0x55>> {
    fn new() -> Self {
        Self {
            state: Header::<0x55>,
        }
    }
}

impl From<StateMachine<Header<0x55>>> for StateMachine<Header<0xAA>> {
    fn from(_a: StateMachine<Header<0x55>>) -> Self {
        Self {
            state: Header::<0xAA>,
        }
    }
}

impl From<StateMachine<Header<0xAA>>> for StateMachine<Header<0x03>> {
    fn from(_a: StateMachine<Header<0xAA>>) -> Self {
        Self {
            state: Header::<0x03>,
        }
    }
}

impl From<StateMachine<Header<0x03>>> for StateMachine<Header<0x08>> {
    fn from(_a: StateMachine<Header<0x03>>) -> Self {
        Self {
            state: Header::<0x08>,
        }
    }
}

impl From<StateMachine<Header<0x08>>> for StateMachine<Data> {
    /// Transitions into `Data` state, pre-filling the packet header bytes
    /// `[0x55, 0xAA, 0x03, 0x08]` into the buffer so the checksum function
    /// always receives a complete, correctly framed 36-byte packet.
    fn from(_: StateMachine<Header<0x08>>) -> Self {
        let mut buf = [0u8; PAYLOAD_SIZE_IN_BYTES];
        // Hardcode first 4 bytes
        buf[0..4].copy_from_slice(&[0x55, 0xAA, 0x03, 0x08]);
        Self {
            state: Data { buf, idx: 4 },
        }
    }
}

impl StateMachine<Data> {
    /// Appends `byte` to the packet buffer.
    ///
    /// Returns:
    /// - `Ok(Self)` while the buffer is still being filled.
    /// - `Err(StateMachine<Complete>)` once all 36 bytes have been received,
    ///   using the type system to signal completion without a separate flag.
    pub fn push(mut self, byte: u8) -> Result<Self, StateMachine<Complete>> {
        self.state.buf[self.state.idx as usize] = byte;
        self.state.idx += 1;
        if self.state.idx == PAYLOAD_SIZE_IN_BYTES as u8 {
            Err(StateMachine {
                state: Complete {
                    buf: self.state.buf,
                },
            })
        } else {
            Ok(self)
        }
    }
}

impl StateMachine<Complete> {
    /// Returns a reference to the completed 36-byte packet buffer.
    pub fn buf(&self) -> &[u8; PAYLOAD_SIZE_IN_BYTES] {
        &self.state.buf
    }
}

/// Camsense X1 UART Packet State Machine
///
/// Parses incoming LiDAR data frames byte by byte. The protocol uses a fixed
/// 4-byte header followed by a 32-byte payload:
/// ```text
/// [0x55] [0xAA] [0x03] [0x08] [2 speed] [2 start angle] [8×3 data] [2 end angle] [2 checksum]
/// ```
///
/// ## Transition Sequence
/// 1. `Header55`: initial state; waits for `0xAA` (the byte after the implicit `0x55`)
/// 2. `HeaderAA`: expects `0x03`
/// 3. `Header03`: expects `0x08`
/// 4. `Header08`: expects the first payload byte; transitions into `Data`
/// 5. `Data`: accumulates payload bytes one at a time into a 36-byte buffer
/// 6. `Complete`: terminal state; holds the fully received packet, ready for checksum validation
///
/// ## Resynchronization
/// Any unexpected byte in a header state resets the machine to `Header55`, providing
/// automatic recovery from stream drift or framing errors. A checksum failure in the
/// caller should also trigger [`reset`] so the next sync sequence is found cleanly.
///
/// ## Design Notes
/// - Const-generic header states (`Header<N>`) enforce valid transitions at compile time.
/// - Payload bytes are collected directly into the final buffer; no extra copies are needed.
/// - The 4 header bytes are pre-filled at `Data` construction so the checksum function
///   always receives a fully framed 36-byte packet.
/// - Fully `no_std` compatible with zero heap allocations.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateMachineWrapper {
    /// Initial state. Waiting for the first synchronization byte (`0x55`).
    Header55(StateMachine<Header<0x55>>),
    /// Expects the second synchronization byte (`0xAA`) immediately after `0x55`.
    HeaderAA(StateMachine<Header<0xAA>>),
    // Expects the data format info byte `0x03`.
    Header03(StateMachine<Header<0x03>>),
    /// Expects the fixed packet length indicator byte (`0x08`) specific to the Camsense X1.
    Header08(StateMachine<Header<0x08>>),
    /// Accumulating payload bytes. Transitions to `Complete` after 32 bytes.
    Data(StateMachine<Data>),
    /// Packet fully received. Call [`take_buffer`] to retrieve the buffer,
    /// then [`reset`] before processing the next frame.
    Complete(StateMachine<Complete>),
}

impl StateMachineWrapper {
    /// Creates a new state machine in the initial `Header55` state.
    pub fn new() -> Self {
        Self::Header55(StateMachine::new())
    }

    /// Resets the state machine to `Header55`, discarding any in-progress packet.
    ///
    /// Should be called after successfully consuming a complete packet, or after
    /// a checksum failure to resynchronize with the byte stream.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Advances the state machine by one byte.
    ///
    /// Header states enforce exact byte values and fall back to `Header55` on
    /// mismatch, providing automatic resynchronization. The `Data` state accepts
    /// any byte and advances the buffer index unconditionally.
    pub fn step(self, value: u8) -> Self {
        match self {
            Self::Header55(state) if value == 0xAA => Self::HeaderAA(state.into()),
            Self::HeaderAA(state) if value == 0x03 => Self::Header03(state.into()),
            Self::Header03(state) if value == 0x08 => Self::Header08(state.into()),
            Self::Header08(s) => {
                let data_sm: StateMachine<Data> = s.into();
                // push the first payload byte immediately
                match data_sm.push(value) {
                    Ok(d) => Self::Data(d),
                    Err(c) => Self::Complete(c),
                }
            }
            Self::Data(s) => match s.push(value) {
                Ok(d) => Self::Data(d),
                Err(c) => Self::Complete(c),
            },
            _ => Self::Header55(StateMachine::new()),
        }
    }

    /// Returns a reference to the completed packet buffer, or `None` if the
    /// packet is not yet complete.
    pub fn take_buffer(&self) -> Option<&[u8; PAYLOAD_SIZE_IN_BYTES]> {
        if let Self::Complete(s) = self {
            Some(s.buf())
        } else {
            None
        }
    }
}
