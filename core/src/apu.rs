// NR10
const MEM_AUD1SWEEP: u16 = 0xFF10;
// NR11
const MEM_AUD1LEN: u16 = 0xFF11;
// NR12
const MEM_AUD1ENV: u16 = 0xFF12;
// NR13
const MEM_AUD1LOW: u16 = 0xFF13;
// NR14
const MEM_AUD1HIGH: u16 = 0xFF14;
// NR21
const MEM_AUD2LEN: u16 = 0xFF16;
// NR22
const MEM_AUD2ENV: u16 = 0xFF17;
// NR23
const MEM_AUD2LOW: u16 = 0xFF18;
// NR24
const MEM_AUD2HIGH: u16 = 0xFF19;
// NR30
const MEM_AUD3ENA: u16 = 0xFF1A;
// NR31
const MEM_AUD3LEN: u16 = 0xFF1B;
// NR32
const MEM_AUD3LEVEL: u16 = 0xFF1C;
// NR33
const MEM_AUD3LOW: u16 = 0xFF1D;
// NR34
const MEM_AUD3HIGH: u16 = 0xFF1E;
// NR41
const MEM_AUD4LEN: u16 = 0xFF20;
// NR42
const MEM_AUD4ENV: u16 = 0xFF21;
// NR43
const MEM_AUD4POLY: u16 = 0xFF22;
// NR44
const MEM_AUD4GO: u16 = 0xFF23;
// NR50
const MEM_AUDVOL: u16 = 0xFF24;
// NR51
const MEM_AUDTERM: u16 = 0xFF25;
// NR52
const MEM_AUDENA: u16 = 0xFF26;
const WAVE_RAM_START: u16 = 0xFF30;
const WAVE_RAM_END: u16 = 0xFF3F;

const WAVE_RAM_SIZE: usize = 0x10;

struct LengthTimer {
    enabled: bool,
    max_value: usize,
    divider: usize,
    counter: usize,
}

impl LengthTimer {
    const fn new(max_value: usize) -> Self {
        Self {enabled: false, max_value, divider: 0, counter: max_value }
    }

    const fn load(&mut self, length: u8) {
        self.counter = self.max_value - length as usize;
    }

    const fn tick(&mut self) -> bool {
        if !self.enabled || self.counter == 0 {
            return false;
        }

        self.divider = self.divider.wrapping_add(1);
        if self.divider % 2 == 0 {
            self.counter -= 1;
        }

        if self.counter == 0 {
            return true;
        }

        false
    }

    const fn trigger(&mut self) {
        // Triggering sets the counter to max value if it has expired
        if self.counter == 0 {
            self.counter = self.max_value;
        }
    }

    const fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Debug, Copy, Clone)]
struct Sweep(u8);

impl Sweep {
    const PACE: u8 = 0b0111_0000;
    const DIRECTION: u8 = 0b0000_1000;
    const INDIVIDUAL_STEP: u8 = 0b0000_0111;
    const UNUSED: u8 = 0b1000_0000;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct DutyCycle(u8);

impl DutyCycle {
    const WAVE_DUTY: u8 = 0b1100_0000;
    const UNUSED: u8 = 0b0011_1111;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct Envelope(u8);

impl Envelope {
    const INITIAL_VOLUME: u8 = 0b1111_0000;
    const ENVELOPE_DIRECTION: u8 = 0b0000_1000;
    const SWEEP_PACE: u8 = 0b0000_0111;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct Period(u16);

impl Period {
    const UNUSED: u16 = 0b1111_1000_0000_0000;

    const fn empty() -> Self {
        Self(Self::UNUSED)
    }

    const fn replace_low(self, low: u8) -> Self {
        let [_low, high] = self.0.to_le_bytes();
        let bits = u16::from_le_bytes([low, high]);
        Self(bits | Self::UNUSED)
    }

    const fn replace_high(self, high: u8) -> Self {
        let [low, _high] = self.0.to_le_bytes();
        let bits = u16::from_le_bytes([low, high]);
        Self(bits | Self::UNUSED)
    }
}

#[derive(Debug, Copy, Clone)]
struct OutputLevel(u8);

impl OutputLevel {
    const OUTPUT_LEVEL: u8 = 0b0110_0000;
    const UNUSED: u8 = 0b1001_1111;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct FrequencyAndRandomness(u8);

impl FrequencyAndRandomness {
    const CLOCK_SHIFT: u8 = 0b1111_0000;
    const LFSR_WIDTH: u8 = 0b0000_1000;
    const CLOCK_DIVIDER: u8 = 0b0000_0111;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct MasterVolume(u8);

impl MasterVolume {
    const VIN_LEFT: u8 = 0b1000_0000;
    const LEFT_VOLUME: u8 = 0b0111_0000;
    const VIN_RIGHT: u8 = 0b0000_1000;
    const RIGHT_VOLUME: u8 = 0b0000_0111;

    const fn new() -> Self {
        Self::from_bits(Self::LEFT_VOLUME | Self::RIGHT_VOLUME)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
struct SoundPanning(u8);

impl SoundPanning {
    const CHANNEL_4_LEFT: u8 = 0b1000_0000;
    const CHANNEL_3_LEFT: u8 = 0b0100_0000;
    const CHANNEL_2_LEFT: u8 = 0b0010_0000;
    const CHANNEL_1_LEFT: u8 = 0b0001_0000;
    const CHANNEL_4_RIGHT: u8 = 0b0000_1000;
    const CHANNEL_3_RIGHT: u8 = 0b0000_0100;
    const CHANNEL_2_RIGHT: u8 = 0b0000_0010;
    const CHANNEL_1_RIGHT: u8 = 0b0000_0001;

    const fn new() -> Self {
        Self::from_bits(
            Self::CHANNEL_4_LEFT
                | Self::CHANNEL_3_LEFT
                | Self::CHANNEL_2_LEFT
                | Self::CHANNEL_1_LEFT
                | Self::CHANNEL_2_RIGHT
                | Self::CHANNEL_1_RIGHT,
        )
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

struct Channel1 {
    enabled: bool,
    sweep: Sweep,
    duty_cycle: DutyCycle,
    length_timer: LengthTimer,
    envelope: Envelope,
    period: Period,
}

impl Channel1 {
    const fn new() -> Self {
        Self {
            enabled: true,
            sweep: Sweep::empty(),
            duty_cycle: DutyCycle::from_bits(0b1000_0000),
            length_timer: LengthTimer::new(64),
            envelope: Envelope::from_bits(Envelope::INITIAL_VOLUME | 0b11),
            period: Period::empty(),
        }
    }

    const fn tick(&mut self) {
        if self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.length_timer.trigger();
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.sweep = Sweep::from_bits(0);
        self.duty_cycle = DutyCycle::from_bits(0);
        self.length_timer.set_enabled(false);
        self.envelope = Envelope::from_bits(0);
    }

    const fn dac_enabled(&self) -> bool {
        self.envelope.bits() & 0xF8 != 0
    }
}

struct PulseChannel {
    enabled: bool,
    duty_cycle: DutyCycle,
    length_timer: LengthTimer,
    envelope: Envelope,
    period: Period,
}

impl PulseChannel {
    const fn new() -> Self {
        Self {
            enabled: false,
            duty_cycle: DutyCycle::empty(),
            length_timer: LengthTimer::new(64),
            envelope: Envelope::empty(),
            period: Period::empty(),
        }
    }

    const fn tick(&mut self) {
        if self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.length_timer.trigger();
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.duty_cycle = DutyCycle::from_bits(0);
        self.length_timer.set_enabled(false);
        self.envelope = Envelope::from_bits(0);
    }

    const fn dac_enabled(&self) -> bool {
        self.envelope.bits() & 0xF8 != 0
    }
}

struct WaveChannel {
    enabled: bool,
    dac_enabled: bool,
    length_timer: LengthTimer,
    volume: OutputLevel,
    period: Period,
}

impl WaveChannel {
    const fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            length_timer: LengthTimer::new(256),
            volume: OutputLevel::empty(),
            period: Period::empty(),
        }
    }

    const fn tick(&mut self) {
        if self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.length_timer.trigger();
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.length_timer.set_enabled(false);
        self.volume = OutputLevel::from_bits(0);
    }

    const fn dac_enabled(&self) -> bool {
        self.dac_enabled
    }

    const fn set_dac_enabled(&mut self, enabled: bool) {
        self.dac_enabled = enabled;
        if !enabled {
            self.enabled = false;
        }
    }
}

struct NoiseChannel {
    enabled: bool,
    length_timer: LengthTimer,
    envelope: Envelope,
    frequency_and_randomness: FrequencyAndRandomness,
}

impl NoiseChannel {
    const fn new() -> Self {
        Self {
            enabled: false,
            length_timer: LengthTimer::new(64),
            envelope: Envelope::empty(),
            frequency_and_randomness: FrequencyAndRandomness::empty(),
        }
    }

    const fn tick(&mut self) {
        if self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.length_timer.trigger();
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.length_timer.set_enabled(false);
        self.envelope = Envelope::from_bits(0);
        self.frequency_and_randomness = FrequencyAndRandomness::from_bits(0);
    }

    const fn dac_enabled(&self) -> bool {
        self.envelope.bits() & 0xF8 != 0
    }
}

pub struct Apu {
    enabled: bool,
    channel1: Channel1,
    channel2: PulseChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    master_volume: MasterVolume,
    sound_panning: SoundPanning,
    wave_ram: [u8; WAVE_RAM_SIZE],
}

impl Apu {
    pub const fn new() -> Self {
        Self {
            enabled: true,
            channel1: Channel1::new(),
            channel2: PulseChannel::new(),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            master_volume: MasterVolume::new(),
            sound_panning: SoundPanning::new(),
            wave_ram: [0xFF; WAVE_RAM_SIZE],
        }
    }

    pub const fn tick(&mut self) {
        self.channel1.tick();
        self.channel2.tick();
        self.channel3.tick();
        self.channel4.tick();
    }

    pub const fn read_audio(&self, addr: u16) -> u8 {
        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep.bits(),
            MEM_AUD1LEN => self.channel1.duty_cycle.bits(),
            MEM_AUD1ENV => self.channel1.envelope.bits(),
            MEM_AUD1HIGH => read_audhigh(self.channel1.length_timer.enabled),
            MEM_AUD2LEN => self.channel2.duty_cycle.bits(),
            MEM_AUD2ENV => self.channel2.envelope.bits(),
            MEM_AUD2HIGH => read_audhigh(self.channel2.length_timer.enabled),
            MEM_AUD3ENA => self.read_aud3ena(),
            MEM_AUD3LEVEL => self.channel3.volume.bits(),
            MEM_AUD3HIGH => read_audhigh(self.channel3.length_timer.enabled),
            MEM_AUD4ENV => self.channel4.envelope.bits(),
            MEM_AUD4POLY => self.channel4.frequency_and_randomness.bits(),
            MEM_AUD4GO => read_audhigh(self.channel4.length_timer.enabled),
            MEM_AUDVOL => self.master_volume.bits(),
            MEM_AUDTERM => self.sound_panning.bits(),
            MEM_AUDENA => self.read_audena(),
            WAVE_RAM_START..=WAVE_RAM_END => self.wave_ram[(addr - WAVE_RAM_START) as usize],
            _ => {
                // AUD1LOW, AUD2LOW, AUD3LEN, AUD3LOW, AUD4LEN are write-only
                0xFF
            }
        }
    }

    pub const fn write_audio(&mut self, addr: u16, value: u8) {
        // Cannot write to most registers while off
        if !self.enabled && addr < MEM_AUDENA {
            return;
        }

        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep = Sweep::from_bits(value),
            MEM_AUD1LEN => {
                self.channel1.duty_cycle = DutyCycle::from_bits(value);
                let length = value & 0x3F;
                self.channel1.length_timer.load(length);
            }
            MEM_AUD1ENV => {
                self.channel1.envelope = Envelope::from_bits(value);
                if !self.channel1.dac_enabled() {
                    self.channel1.enabled = false;
                }
            },
            MEM_AUD1LOW => self.channel1.period = self.channel1.period.replace_low(value),
            MEM_AUD1HIGH => {
                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel1.trigger();
                }
                self.channel1.period = self.channel1.period.replace_high(value);
                let length_enabled = value & 0x40 != 0;
                self.channel1.length_timer.set_enabled(length_enabled);
            }
            MEM_AUD2LEN => {
                self.channel2.duty_cycle = DutyCycle::from_bits(value);
                let length = value & 0x3F;
                self.channel2.length_timer.load(length);
            }
            MEM_AUD2ENV => {
                self.channel2.envelope = Envelope::from_bits(value);
                if !self.channel2.dac_enabled() {
                    self.channel2.enabled = false;
                }
            },
            MEM_AUD2LOW => self.channel2.period = self.channel2.period.replace_low(value),
            MEM_AUD2HIGH => {
                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel2.trigger();
                }
                self.channel2.period = self.channel2.period.replace_high(value);
                let length_enabled = value & 0x40 != 0;
                self.channel2.length_timer.set_enabled(length_enabled);
            }
            MEM_AUD3ENA => {
                let enabled = value & 0x80 != 0;
                self.channel3.set_dac_enabled(enabled);
            },
            MEM_AUD3LEN => self.channel3.length_timer.load(value),
            MEM_AUD3LEVEL => self.channel3.volume = OutputLevel::from_bits(value),
            MEM_AUD3LOW => self.channel3.period = self.channel3.period.replace_low(value),
            MEM_AUD3HIGH => {
                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel3.trigger();
                }
                self.channel3.period = self.channel3.period.replace_high(value);
                let length_enabled = value & 0x40 != 0;
                self.channel3.length_timer.set_enabled(length_enabled);
            }
            MEM_AUD4LEN => {
                let length = value & 0x3F;
                self.channel4.length_timer.load(length);
            },
            MEM_AUD4ENV => {
                self.channel4.envelope = Envelope::from_bits(value);
                if !self.channel4.dac_enabled() {
                    self.channel4.enabled = false;
                }
            },
            MEM_AUD4POLY => {
                self.channel4.frequency_and_randomness = FrequencyAndRandomness::from_bits(value);
            }
            MEM_AUD4GO => {
                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel4.trigger();
                }
                let length_enabled = value & 0x40 != 0;
                self.channel4.length_timer.set_enabled(length_enabled);
            }
            MEM_AUDVOL => self.master_volume = MasterVolume::from_bits(value),
            MEM_AUDTERM => self.sound_panning = SoundPanning::from_bits(value),
            MEM_AUDENA => {
                self.enabled = value & 0x80 != 0;
                if !self.enabled {
                    self.disable();
                }
            }
            WAVE_RAM_START..=WAVE_RAM_END => {
                self.wave_ram[(addr - WAVE_RAM_START) as usize] = value;
            }
            _ => {}
        }
    }

    const fn read_aud3ena(&self) -> u8 {
        let mut bits = 0x7F;
        if self.channel3.dac_enabled {
            bits |= 0x80;
        }
        bits
    }

    const fn read_audena(&self) -> u8 {
        let mut bits = 0x70;
        if self.enabled {
            bits |= 0x80;
        }
        if self.channel4.enabled {
            bits |= 0x08;
        }
        if self.channel3.enabled {
            bits |= 0x04;
        }
        if self.channel2.enabled {
            bits |= 0x02;
        }
        if self.channel1.enabled {
            bits |= 0x01;
        }
        bits
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.master_volume = MasterVolume::from_bits(0);
        self.sound_panning = SoundPanning::from_bits(0);

        self.channel1.disable();
        self.channel2.disable();
        self.channel3.disable();
        self.channel4.disable();
    }
}

const fn read_audhigh(length_enabled: bool) -> u8 {
    let mut bits = 0xBF;
    if length_enabled {
        bits |= 0x40;
    }
    bits
}
