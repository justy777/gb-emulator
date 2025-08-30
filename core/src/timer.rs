use crate::interrupt::{Interrupt, InterruptFlags};

const MEM_DIV: u16 = 0xFF04;
const MEM_TIMA: u16 = 0xFF05;
const MEM_TMA: u16 = 0xFF06;
const MEM_TAC: u16 = 0xFF07;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
enum ClockSelect {
    // f/2^10
    Every256Cycles = 0b00,
    // f/2^4
    Every4Cycles = 0b01,
    // f/2^6
    Every16Cycles = 0b10,
    // f/2^8
    Every64Cycles = 0b11,
}

impl ClockSelect {
    const fn frequency_mask(self) -> u16 {
        match self {
            Self::Every256Cycles => 0x80,
            Self::Every4Cycles => 0x02,
            Self::Every16Cycles => 0x08,
            Self::Every64Cycles => 0x20,
        }
    }
}

impl From<u8> for ClockSelect {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Every256Cycles,
            0b01 => Self::Every4Cycles,
            0b10 => Self::Every16Cycles,
            0b11 => Self::Every64Cycles,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    // DIV
    // Note: only uses 14 bits
    divider: u16,
    // TIMA
    counter: u8,
    // TMA
    modulo: u8,
    // TAC
    enabled: bool,
    clock_select: ClockSelect,
    // Used to check for falling edge
    tick_signal: bool,
    audio_signal: bool,
    // Used to delay overflow until the next cycle
    overflow: bool,
    after_overflow: bool,
    apu_ticked: Option<()>,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            divider: (0xAB << 6) + 0x33,
            counter: 0,
            modulo: 0,
            enabled: false,
            clock_select: ClockSelect::Every256Cycles,
            tick_signal: false,
            audio_signal: false,
            overflow: false,
            after_overflow: false,
            apu_ticked: None,
        }
    }

    pub const fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            #[allow(clippy::cast_possible_truncation)]
            MEM_DIV => (self.divider >> 6) as u8,
            MEM_TIMA => self.counter,
            MEM_TMA => self.modulo,
            MEM_TAC => self.read_tac(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_DIV => self.reset_div(),
            MEM_TIMA => self.set_tima(value),
            MEM_TMA => self.set_tma(value),
            MEM_TAC => self.write_tac(value),
            _ => unreachable!(),
        }
    }

    pub const fn increment_divider(&mut self, interrupt_flags: &mut InterruptFlags) -> bool {
        self.after_overflow = false;
        self.divider = self.divider.wrapping_add(1);

        // Checks for next cycle after overflow occurs
        if self.overflow {
            self.counter = self.modulo;
            interrupt_flags.set(Interrupt::Timer, true);
            self.overflow = false;
            self.after_overflow = true;
        }

        self.sync_signals();
        self.apu_ticked.take().is_some()
    }

    const fn sync_signals(&mut self) {
        let new_tick_signal = self.frequency_bit() && self.enabled;

        if self.tick_signal && !new_tick_signal {
            let (counter, overflow) = self.counter.overflowing_add(1);
            self.counter = counter;
            self.overflow = overflow;
        }

        self.tick_signal = new_tick_signal;

        let new_audio_signal = self.audio_bit();

        if self.audio_signal && !new_audio_signal {
            self.apu_ticked = Some(());
        }

        self.audio_signal = new_audio_signal;
    }

    const fn frequency_bit(&self) -> bool {
        (self.divider & self.clock_select.frequency_mask()) != 0
    }

    const fn audio_bit(&self) -> bool {
        self.divider & 0x0400 != 0
    }

    const fn reset_div(&mut self) {
        self.divider = 0;
        self.sync_signals();
    }

    const fn set_tima(&mut self, value: u8) {
        if !self.after_overflow {
            self.counter = value;
            self.overflow = false;
        }
    }

    const fn set_tma(&mut self, value: u8) {
        self.modulo = value;
        if self.after_overflow {
            self.counter = value;
        }
    }

    const fn read_tac(&self) -> u8 {
        let mut bits = 0xF8;
        if self.enabled {
            bits |= 0x04;
        }
        bits |= self.clock_select as u8;
        bits
    }

    fn write_tac(&mut self, value: u8) {
        self.enabled = value & 0x04 != 0;
        self.clock_select = ClockSelect::from(value & 0x03);
        self.sync_signals();
    }
}
