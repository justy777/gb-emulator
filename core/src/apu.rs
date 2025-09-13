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

#[derive(Debug)]
struct PeriodCounter {
    period: u16,
    counter: u16,
}

impl PeriodCounter {
    const fn new() -> Self {
        Self {
            period: 0,
            counter: 2048,
        }
    }

    const fn tick(&mut self) -> bool {
        self.counter -= 1;
        if self.counter == 0 {
            self.counter = 2048 - self.period;
            return true;
        }
        false
    }

    const fn trigger(&mut self) {
        self.counter = 2048 - self.period;
    }
}

#[derive(Debug)]
struct LengthTimer {
    enabled: bool,
    max_value: u16,
    counter: u16,
}

impl LengthTimer {
    const fn new(max_value: u16) -> Self {
        Self {
            enabled: false,
            max_value,
            counter: max_value,
        }
    }

    const fn load(&mut self, length: u8) {
        self.counter = self.max_value - length as u16;
    }

    const fn tick(&mut self) -> bool {
        if !self.enabled || self.counter == 0 {
            return false;
        }

        self.counter -= 1;
        if self.counter == 0 {
            return true;
        }

        false
    }

    const fn trigger(&mut self, divider: u8) {
        // Triggering sets the counter to max value if it has expired
        if self.counter == 0 {
            self.counter = self.max_value;
            // Obscure behaviour: If triggered on even clock timer while enabled is decremented by 1
            if self.enabled && divider % 2 == 0 {
                self.counter -= 1;
            }
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
enum DutyCycle {
    // 12.5%
    OneEighth = 0b00,
    // 25%
    OneFourth = 0b01,
    // 50%
    OneHalf = 0b10,
    // 75%
    ThreeFourths = 0b11,
}

impl From<u8> for DutyCycle {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::OneEighth,
            0b01 => Self::OneFourth,
            0b10 => Self::OneHalf,
            0b11 => Self::ThreeFourths,
            _ => unreachable!(),
        }
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
enum OutputLevel {
    // No sound
    Mute = 0b00,
    // 100%
    Full = 0b01,
    // 50%
    Half = 0b10,
    // 25 %
    Quarter = 0b11,
}

impl From<u8> for OutputLevel {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Mute,
            0b01 => Self::Full,
            0b10 => Self::Half,
            0b11 => Self::Quarter,
            _ => unreachable!(),
        }
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

struct PulseChannel {
    enabled: bool,
    sweep: Sweep,
    period_counter: PeriodCounter,
    duty_cycle: DutyCycle,
    length_timer: LengthTimer,
    envelope: Envelope,
    sample_index: usize,
}

impl PulseChannel {
    const fn new(enabled: bool, envelope_value: u8, duty_cycle: DutyCycle) -> Self {
        Self {
            enabled,
            sweep: Sweep::empty(),
            period_counter: PeriodCounter::new(),
            duty_cycle,
            length_timer: LengthTimer::new(64),
            envelope: Envelope::from_bits(envelope_value),
            sample_index: 0,
        }
    }

    const fn tick(&mut self, divider: u8) {
        if self.period_counter.tick() {
            self.sample_index = (self.sample_index + 1) % 8;
        }

        if divider % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self, divider: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.period_counter.trigger();
        self.length_timer.trigger(divider);
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.sweep = Sweep::from_bits(0);
        self.duty_cycle = DutyCycle::OneEighth;
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
    period_counter: PeriodCounter,
    length_timer: LengthTimer,
    volume: OutputLevel,
    sample_index: usize,
}

impl WaveChannel {
    const fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            period_counter: PeriodCounter::new(),
            length_timer: LengthTimer::new(256),
            volume: OutputLevel::Mute,
            sample_index: 0,
        }
    }

    const fn tick(&mut self, divider: u8) {
        let mut i = 0;
        while i < 2 {
            if self.period_counter.tick() {
                self.sample_index = (self.sample_index + 1) % 32;
            }
            i += 1;
        }

        if divider % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self, divider: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.period_counter.trigger();
        self.length_timer.trigger(divider);
    }

    const fn disable(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.length_timer.set_enabled(false);
        self.volume = OutputLevel::Mute;
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

    const fn tick(&mut self, divider: u8) {
        if divider % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self, divider: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.length_timer.trigger(divider);
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
    divider: u8,
    channel1: PulseChannel,
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
            divider: 0,
            channel1: PulseChannel::new(true, Envelope::INITIAL_VOLUME | 0b11, DutyCycle::OneHalf),
            channel2: PulseChannel::new(false, 0, DutyCycle::OneEighth),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            master_volume: MasterVolume::new(),
            sound_panning: SoundPanning::new(),
            wave_ram: [0xFF; WAVE_RAM_SIZE],
        }
    }

    pub const fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        self.divider = self.divider.wrapping_add(1);

        self.channel1.tick(self.divider);
        self.channel2.tick(self.divider);
        self.channel3.tick(self.divider);
        self.channel4.tick(self.divider);
    }

    pub const fn read_audio(&self, addr: u16) -> u8 {
        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep.bits(),
            MEM_AUD1LEN => {
                let mut bits = 0x3F;
                bits |= (self.channel1.duty_cycle as u8) << 6;
                bits
            }
            MEM_AUD1ENV => self.channel1.envelope.bits(),
            MEM_AUD1HIGH => read_audhigh(self.channel1.length_timer.enabled),
            MEM_AUD2LEN => {
                let mut bits = 0x3F;
                bits |= (self.channel2.duty_cycle as u8) << 6;
                bits
            }
            MEM_AUD2ENV => self.channel2.envelope.bits(),
            MEM_AUD2HIGH => read_audhigh(self.channel2.length_timer.enabled),
            MEM_AUD3ENA => self.read_aud3ena(),
            MEM_AUD3LEVEL => {
                let mut bits = 0x9F;
                bits |= (self.channel3.volume as u8) << 5;
                bits
            }
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

    pub fn write_audio(&mut self, addr: u16, value: u8) {
        // Cannot write to most registers while off
        if !self.enabled && addr < MEM_AUDENA {
            return;
        }

        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep = Sweep::from_bits(value),
            MEM_AUD1LEN => {
                self.channel1.duty_cycle = DutyCycle::from(value >> 6);
                let length = value & 0x3F;
                self.channel1.length_timer.load(length);
            }
            MEM_AUD1ENV => {
                self.channel1.envelope = Envelope::from_bits(value);
                if !self.channel1.dac_enabled() {
                    self.channel1.enabled = false;
                }
            }
            MEM_AUD1LOW => {
                let [_low, high] = self.channel1.period_counter.period.to_le_bytes();
                self.channel1.period_counter.period = u16::from_le_bytes([value, high]);
            }
            MEM_AUD1HIGH => {
                let [low, _high] = self.channel1.period_counter.period.to_le_bytes();
                self.channel1.period_counter.period = u16::from_le_bytes([low, value & 0x07]);

                let prev_length_enabled = self.channel1.length_timer.enabled;
                let length_enabled = value & 0x40 != 0;
                self.channel1.length_timer.set_enabled(length_enabled);
                // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
                if !prev_length_enabled
                    && length_enabled
                    && self.divider % 2 == 0
                    && self.channel1.length_timer.tick()
                {
                    self.channel1.enabled = false;
                }

                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel1.trigger(self.divider);
                }
            }
            MEM_AUD2LEN => {
                self.channel2.duty_cycle = DutyCycle::from(value >> 6);
                let length = value & 0x3F;
                self.channel2.length_timer.load(length);
            }
            MEM_AUD2ENV => {
                self.channel2.envelope = Envelope::from_bits(value);
                if !self.channel2.dac_enabled() {
                    self.channel2.enabled = false;
                }
            }
            MEM_AUD2LOW => {
                let [_low, high] = self.channel2.period_counter.period.to_le_bytes();
                self.channel2.period_counter.period = u16::from_le_bytes([value, high]);
            }
            MEM_AUD2HIGH => {
                let [low, _high] = self.channel2.period_counter.period.to_le_bytes();
                self.channel2.period_counter.period = u16::from_le_bytes([low, value & 0x07]);

                let prev_length_enabled = self.channel2.length_timer.enabled;
                let length_enabled = value & 0x40 != 0;
                self.channel2.length_timer.set_enabled(length_enabled);
                // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
                if !prev_length_enabled
                    && length_enabled
                    && self.divider % 2 == 0
                    && self.channel2.length_timer.tick()
                {
                    self.channel2.enabled = false;
                }

                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel2.trigger(self.divider);
                }
            }
            MEM_AUD3ENA => {
                let enabled = value & 0x80 != 0;
                self.channel3.set_dac_enabled(enabled);
            }
            MEM_AUD3LEN => self.channel3.length_timer.load(value),
            MEM_AUD3LEVEL => {
                let level = (value & 0x60) >> 5;
                self.channel3.volume = OutputLevel::from(level);
            }
            MEM_AUD3LOW => {
                let [_low, high] = self.channel3.period_counter.period.to_le_bytes();
                self.channel3.period_counter.period = u16::from_le_bytes([value, high]);
            }
            MEM_AUD3HIGH => {
                let [low, _high] = self.channel3.period_counter.period.to_le_bytes();
                self.channel3.period_counter.period = u16::from_le_bytes([low, value & 0x07]);

                let prev_length_enabled = self.channel3.length_timer.enabled;
                let length_enabled = value & 0x40 != 0;
                self.channel3.length_timer.set_enabled(length_enabled);
                // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
                if !prev_length_enabled
                    && length_enabled
                    && self.divider % 2 == 0
                    && self.channel3.length_timer.tick()
                {
                    self.channel3.enabled = false;
                }

                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel3.trigger(self.divider);
                }
            }
            MEM_AUD4LEN => {
                let length = value & 0x3F;
                self.channel4.length_timer.load(length);
            }
            MEM_AUD4ENV => {
                self.channel4.envelope = Envelope::from_bits(value);
                if !self.channel4.dac_enabled() {
                    self.channel4.enabled = false;
                }
            }
            MEM_AUD4POLY => {
                self.channel4.frequency_and_randomness = FrequencyAndRandomness::from_bits(value);
            }
            MEM_AUD4GO => {
                let prev_length_enabled = self.channel4.length_timer.enabled;
                let length_enabled = value & 0x40 != 0;
                self.channel4.length_timer.set_enabled(length_enabled);
                // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
                if !prev_length_enabled
                    && length_enabled
                    && self.divider % 2 == 0
                    && self.channel4.length_timer.tick()
                {
                    self.channel4.enabled = false;
                }

                let triggered = value & 0x80 != 0;
                if triggered {
                    self.channel4.trigger(self.divider);
                }
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
        self.divider = 0;
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
