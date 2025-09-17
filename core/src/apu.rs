use std::cmp::{max, min};

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

#[derive(Debug, Copy, Clone)]
enum SweepDirection {
    Increase = 0b0,
    Decrease = 0b1,
}

impl From<bool> for SweepDirection {
    fn from(value: bool) -> Self {
        if value {
            Self::Decrease
        } else {
            Self::Increase
        }
    }
}

#[derive(Debug)]
struct SweepTimer {
    enabled: bool,
    counter: u8,
    pace: u8,
    direction: SweepDirection,
    shift: u8,
    period: u16,
    negate_mode: bool,
}

impl SweepTimer {
    const fn new() -> Self {
        Self {
            enabled: false,
            counter: 0,
            pace: 0,
            direction: SweepDirection::Increase,
            shift: 0,
            period: 0,
            negate_mode: false,
        }
    }
    const fn tick(&mut self) -> bool {
        if !self.enabled {
            return false;
        }

        self.counter -= 1;
        if self.counter == 0 {
            self.counter = if self.pace > 0 { self.pace } else { 8 };
            return self.pace > 0;
        }
        false
    }

    const fn trigger(&mut self, period: u16) {
        self.enabled = self.pace > 0 || self.shift > 0;
        self.period = period;
        self.counter = if self.pace > 0 { self.pace } else { 8 };
        self.negate_mode = false;
    }

    const fn next_period(&mut self) -> u16 {
        self.negate_mode = matches!(self.direction, SweepDirection::Decrease);
        let delta = self.period >> self.shift;
        match self.direction {
            SweepDirection::Increase => self.period + delta,
            SweepDirection::Decrease => self.period - delta,
        }
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

    const fn trigger(&mut self, frame: u8) {
        // Triggering sets the counter to max value if it has expired
        if self.counter == 0 {
            self.counter = self.max_value;
            // Obscure behaviour: If triggered on even clock timer while enabled is decremented by 1
            if self.enabled && frame % 2 == 0 {
                self.counter -= 1;
            }
        }
    }

    const fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
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
enum EnvelopeDirection {
    Decrease = 0b0,
    Increase = 0b1,
}

impl From<bool> for EnvelopeDirection {
    fn from(value: bool) -> Self {
        if value {
            Self::Increase
        } else {
            Self::Decrease
        }
    }
}

#[derive(Debug, Clone)]
struct EnvelopeTimer {
    volume: u8,
    direction: EnvelopeDirection,
    pace: u8,
    initial_volume: u8,
    configured_direction: EnvelopeDirection,
    configured_pace: u8,
    counter: u8,
}

impl EnvelopeTimer {
    const fn new(volume: u8, direction: EnvelopeDirection, pace: u8) -> Self {
        Self {
            volume,
            direction,
            pace,
            initial_volume: volume,
            configured_direction: direction,
            configured_pace: pace,
            counter: pace,
        }
    }

    fn tick(&mut self) {
        if self.pace == 0 {
            return;
        }

        self.counter -= 1;
        if self.counter == 0 {
            self.counter = self.pace;
            self.volume = match self.direction {
                EnvelopeDirection::Decrease => min(self.volume.saturating_sub(1), 0),
                EnvelopeDirection::Increase => max(self.volume.saturating_add(1), 15),
            };
        }
    }

    const fn trigger(&mut self, frame: u8) {
        self.volume = self.initial_volume;
        self.direction = self.configured_direction;
        self.pace = self.configured_pace;

        self.counter = self.pace;

        if frame == 6 {
            self.counter += 1;
        }
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

#[derive(Debug, Clone)]
struct MasterVolume {
    vin_left: bool,
    left_volume: u8,
    vin_right: bool,
    right_volume: u8,
}

impl MasterVolume {
    const fn new(left_volume: u8, right_volume: u8) -> Self {
        Self {
            vin_left: false,
            left_volume,
            vin_right: false,
            right_volume,
        }
    }
}

#[derive(Debug, Clone)]
struct Panning {
    left: [bool; 4],
    right: [bool; 4],
}

impl Panning {
    const fn new(left: [bool; 4], right: [bool; 4]) -> Self {
        Self { left, right }
    }
}

struct PulseChannel {
    enabled: bool,
    sweep: SweepTimer,
    period_counter: PeriodCounter,
    duty_cycle: DutyCycle,
    length_timer: LengthTimer,
    envelope: EnvelopeTimer,
    sample_index: usize,
}

impl PulseChannel {
    const fn new(enabled: bool, envelope_timer: EnvelopeTimer, duty_cycle: DutyCycle) -> Self {
        Self {
            enabled,
            sweep: SweepTimer::new(),
            period_counter: PeriodCounter::new(),
            duty_cycle,
            length_timer: LengthTimer::new(64),
            envelope: envelope_timer,
            sample_index: 0,
        }
    }

    fn tick(&mut self, frame: u8) {
        if self.period_counter.tick() {
            self.sample_index = (self.sample_index + 1) % 8;
        }

        if frame % 4 == 2 && self.sweep.tick() {
            let period = self.sweep.next_period();
            if period > 2047 {
                self.enabled = false;
            } else if self.sweep.shift > 0 {
                self.period_counter.period = period;
                self.sweep.period = period;

                if self.sweep.next_period() > 2047 {
                    self.enabled = false;
                }
            }
        }

        if frame % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }

        if frame == 7 {
            self.envelope.tick();
        }
    }

    const fn trigger(&mut self, frame: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.period_counter.trigger();
        self.sweep.trigger(self.period_counter.period);
        if self.sweep.shift > 0 && self.sweep.next_period() > 2047 {
            self.enabled = false;
        }

        self.envelope.trigger(frame);
        self.length_timer.trigger(frame);
    }

    const fn power_on(&mut self) {
        self.duty_cycle = DutyCycle::OneEighth;
    }

    const fn power_off(&mut self) {
        self.enabled = false;
        self.sweep = SweepTimer::new();
        self.length_timer.set_enabled(false);
        self.envelope = EnvelopeTimer::new(0, EnvelopeDirection::Decrease, 0);
        self.period_counter.period = 0;
        self.period_counter.period = 0;
    }

    const fn dac_enabled(&self) -> bool {
        self.envelope.initial_volume > 0 || self.envelope.configured_direction as u8 > 0
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

    const fn tick(&mut self, frame: u8) {
        let mut i = 0;
        while i < 2 {
            if self.period_counter.tick() {
                self.sample_index = (self.sample_index + 1) % 32;
            }
            i += 1;
        }

        if frame % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }
    }

    const fn trigger(&mut self, frame: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.period_counter.trigger();
        self.length_timer.trigger(frame);
    }

    const fn power_off(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.length_timer.set_enabled(false);
        self.volume = OutputLevel::Mute;
        self.period_counter.period = 0;
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
    envelope: EnvelopeTimer,
    frequency_and_randomness: FrequencyAndRandomness,
}

impl NoiseChannel {
    const fn new() -> Self {
        Self {
            enabled: false,
            length_timer: LengthTimer::new(64),
            envelope: EnvelopeTimer::new(0, EnvelopeDirection::Decrease, 0),
            frequency_and_randomness: FrequencyAndRandomness::empty(),
        }
    }

    fn tick(&mut self, frame: u8) {
        if frame % 2 == 0 && self.length_timer.tick() {
            self.enabled = false;
        }

        if frame == 7 {
            self.envelope.tick();
        }
    }

    const fn trigger(&mut self, frame: u8) {
        if self.dac_enabled() {
            self.enabled = true;
        }
        self.envelope.trigger(frame);
        self.length_timer.trigger(frame);
    }

    const fn power_off(&mut self) {
        self.enabled = false;
        self.length_timer.set_enabled(false);
        self.envelope = EnvelopeTimer::new(0, EnvelopeDirection::Decrease, 0);
        self.frequency_and_randomness = FrequencyAndRandomness::from_bits(0);
    }

    const fn dac_enabled(&self) -> bool {
        self.envelope.initial_volume > 0 || self.envelope.configured_direction as u8 > 0
    }
}

pub struct Apu {
    enabled: bool,
    frame: u8,
    channel1: PulseChannel,
    channel2: PulseChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    master_volume: MasterVolume,
    panning: Panning,
    wave_ram: [u8; WAVE_RAM_SIZE],
}

impl Apu {
    pub const fn new() -> Self {
        let envelope_timer1 = EnvelopeTimer::new(15, EnvelopeDirection::Decrease, 3);
        let envelope_timer2 = EnvelopeTimer::new(0, EnvelopeDirection::Decrease, 0);
        let pan_left = [true; 4];
        let pan_right = [true, true, false, false];
        Self {
            enabled: true,
            frame: 0,
            channel1: PulseChannel::new(true, envelope_timer1, DutyCycle::OneHalf),
            channel2: PulseChannel::new(false, envelope_timer2, DutyCycle::OneEighth),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            master_volume: MasterVolume::new(7, 7),
            panning: Panning::new(pan_left, pan_right),
            wave_ram: [0xFF; WAVE_RAM_SIZE],
        }
    }

    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        self.frame = (self.frame + 1) % 8;

        self.channel1.tick(self.frame);
        self.channel2.tick(self.frame);
        self.channel3.tick(self.frame);
        self.channel4.tick(self.frame);
    }

    const fn power_on(&mut self) {
        self.enabled = true;
        self.frame = 7;

        self.channel1.power_on();
        self.channel2.power_on();
    }

    const fn power_off(&mut self) {
        self.enabled = false;
        self.master_volume = MasterVolume::new(0, 0);
        self.panning = Panning::new([false; 4], [false; 4]);

        self.channel1.power_off();
        self.channel2.power_off();
        self.channel3.power_off();
        self.channel4.power_off();
    }

    pub const fn read_audio(&self, addr: u16) -> u8 {
        match addr {
            MEM_AUD1SWEEP => self.read_aud1sweep(),
            MEM_AUD1LEN => self.read_aud1len(),
            MEM_AUD1ENV => self.read_aud1env(),
            MEM_AUD1HIGH => self.read_aud1high(),
            MEM_AUD2LEN => self.read_aud2len(),
            MEM_AUD2ENV => self.read_aud2env(),
            MEM_AUD2HIGH => self.read_aud2high(),
            MEM_AUD3ENA => self.read_aud3ena(),
            MEM_AUD3LEVEL => self.read_aud3level(),
            MEM_AUD3HIGH => self.read_aud3high(),
            MEM_AUD4ENV => self.read_aud4env(),
            MEM_AUD4POLY => self.channel4.frequency_and_randomness.bits(),
            MEM_AUD4GO => self.read_aud4go(),
            MEM_AUDVOL => self.read_audvol(),
            MEM_AUDTERM => self.read_audterm(),
            MEM_AUDENA => self.read_audena(),
            WAVE_RAM_START..=WAVE_RAM_END => self.wave_ram[(addr - WAVE_RAM_START) as usize],
            _ => {
                // AUD1LOW, AUD2LOW, AUD3LEN, AUD3LOW, AUD4LEN are write-only
                0xFF
            }
        }
    }

    const fn read_aud1sweep(&self) -> u8 {
        let mut bits = 0x80;
        bits |= self.channel1.sweep.pace << 4;
        bits |= (self.channel1.sweep.direction as u8) << 3;
        bits |= self.channel1.sweep.shift;
        bits
    }

    const fn read_aud1len(&self) -> u8 {
        let mut bits = 0x3F;
        bits |= (self.channel1.duty_cycle as u8) << 6;
        bits
    }

    const fn read_aud1env(&self) -> u8 {
        let mut bits = 0;
        bits |= self.channel1.envelope.initial_volume << 4;
        bits |= (self.channel1.envelope.configured_direction as u8) << 3;
        bits |= self.channel1.envelope.configured_pace;
        bits
    }

    const fn read_aud1high(&self) -> u8 {
        let mut bits = 0xBF;
        if self.channel1.length_timer.enabled {
            bits |= 0x40;
        }
        bits
    }

    const fn read_aud2len(&self) -> u8 {
        let mut bits = 0x3F;
        bits |= (self.channel2.duty_cycle as u8) << 6;
        bits
    }

    const fn read_aud2env(&self) -> u8 {
        let mut bits = 0;
        bits |= self.channel2.envelope.initial_volume << 4;
        bits |= (self.channel2.envelope.configured_direction as u8) << 3;
        bits |= self.channel2.envelope.configured_pace;
        bits
    }

    const fn read_aud2high(&self) -> u8 {
        let mut bits = 0xBF;
        if self.channel2.length_timer.enabled {
            bits |= 0x40;
        }
        bits
    }

    const fn read_aud3ena(&self) -> u8 {
        let mut bits = 0x7F;
        if self.channel3.dac_enabled {
            bits |= 0x80;
        }
        bits
    }

    const fn read_aud3level(&self) -> u8 {
        let mut bits = 0x9F;
        bits |= (self.channel3.volume as u8) << 5;
        bits
    }

    const fn read_aud3high(&self) -> u8 {
        let mut bits = 0xBF;
        if self.channel3.length_timer.enabled {
            bits |= 0x40;
        }
        bits
    }

    const fn read_aud4env(&self) -> u8 {
        let mut bits = 0;
        bits |= self.channel4.envelope.initial_volume << 4;
        bits |= (self.channel4.envelope.configured_direction as u8) << 3;
        bits |= self.channel4.envelope.configured_pace;
        bits
    }

    const fn read_aud4go(&self) -> u8 {
        let mut bits = 0xBF;
        if self.channel4.length_timer.enabled {
            bits |= 0x40;
        }
        bits
    }

    const fn read_audvol(&self) -> u8 {
        let mut bits = 0;
        if self.master_volume.vin_left {
            bits |= 0x80;
        }
        bits |= self.master_volume.left_volume << 4;
        if self.master_volume.vin_right {
            bits |= 0x08;
        }
        bits |= self.master_volume.right_volume;
        bits
    }

    const fn read_audterm(&self) -> u8 {
        let mut bits = 0;
        if self.panning.left[3] {
            bits |= 0x80;
        }
        if self.panning.left[2] {
            bits |= 0x40;
        }
        if self.panning.left[1] {
            bits |= 0x20;
        }
        if self.panning.left[0] {
            bits |= 0x10;
        }
        if self.panning.right[3] {
            bits |= 0x08;
        }
        if self.panning.right[2] {
            bits |= 0x04;
        }
        if self.panning.right[1] {
            bits |= 0x02;
        }
        if self.panning.right[0] {
            bits |= 0x01;
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

    pub fn write_audio(&mut self, addr: u16, value: u8) {
        // Cannot write to most registers while off
        // DMG specific: length counters are still writable when power is off
        if !self.enabled
            && addr != MEM_AUDENA
            && addr < WAVE_RAM_START
            && addr != MEM_AUD1LEN
            && addr != MEM_AUD2LEN
            && addr != MEM_AUD3LEN
            && addr != MEM_AUD4LEN
        {
            return;
        }

        match addr {
            MEM_AUD1SWEEP => self.write_aud1sweep(value),
            MEM_AUD1LEN => self.write_aud1len(value),
            MEM_AUD1ENV => self.write_aud1env(value),
            MEM_AUD1LOW => self.write_aud1low(value),
            MEM_AUD1HIGH => self.write_aud1high(value),
            MEM_AUD2LEN => self.write_aud2len(value),
            MEM_AUD2ENV => self.write_aud2env(value),
            MEM_AUD2LOW => self.write_aud2low(value),
            MEM_AUD2HIGH => self.write_aud2high(value),
            MEM_AUD3ENA => self.write_aud3ena(value),
            MEM_AUD3LEN => self.channel3.length_timer.load(value),
            MEM_AUD3LEVEL => self.write_aud3level(value),
            MEM_AUD3LOW => self.write_aud3low(value),
            MEM_AUD3HIGH => self.write_aud3high(value),
            MEM_AUD4LEN => self.write_aud4len(value),
            MEM_AUD4ENV => self.write_aud4env(value),
            MEM_AUD4POLY => self.write_aud4poly(value),
            MEM_AUD4GO => self.write_aud4go(value),
            MEM_AUDVOL => self.write_audvol(value),
            MEM_AUDTERM => self.write_audterm(value),
            MEM_AUDENA => self.write_audena(value),
            WAVE_RAM_START..=WAVE_RAM_END => {
                self.wave_ram[(addr - WAVE_RAM_START) as usize] = value;
            }
            _ => {}
        }
    }

    fn write_aud1sweep(&mut self, value: u8) {
        self.channel1.sweep.pace = (value & 0x70) >> 4;
        self.channel1.sweep.direction = SweepDirection::from((value & 0x08) != 0);
        self.channel1.sweep.shift = value & 0x07;

        if self.channel1.sweep.negate_mode
            && matches!(self.channel1.sweep.direction, SweepDirection::Increase)
        {
            self.channel1.enabled = false;
        }
    }

    fn write_aud1len(&mut self, value: u8) {
        self.channel1.duty_cycle = DutyCycle::from(value >> 6);
        let length = value & 0x3F;
        self.channel1.length_timer.load(length);
    }

    fn write_aud1env(&mut self, value: u8) {
        self.channel1.envelope.initial_volume = value >> 4;
        let direction = value & 0x08 != 0;
        self.channel1.envelope.configured_direction = EnvelopeDirection::from(direction);
        self.channel1.envelope.configured_pace = value & 0x07;
        if !self.channel1.dac_enabled() {
            self.channel1.enabled = false;
        }
    }

    const fn write_aud1low(&mut self, value: u8) {
        self.channel1.period_counter.period &= 0x700;
        self.channel1.period_counter.period |= value as u16;
    }

    const fn write_aud1high(&mut self, value: u8) {
        self.channel1.period_counter.period &= 0xFF;
        self.channel1.period_counter.period |= (value as u16 & 0x07) << 8;

        let prev_length_enabled = self.channel1.length_timer.enabled;
        let length_enabled = value & 0x40 != 0;
        self.channel1.length_timer.set_enabled(length_enabled);
        // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
        if !prev_length_enabled
            && length_enabled
            && self.frame % 2 == 0
            && self.channel1.length_timer.tick()
        {
            self.channel1.enabled = false;
        }

        let triggered = value & 0x80 != 0;
        if triggered {
            self.channel1.trigger(self.frame);
        }
    }

    fn write_aud2len(&mut self, value: u8) {
        self.channel2.duty_cycle = DutyCycle::from(value >> 6);
        let length = value & 0x3F;
        self.channel2.length_timer.load(length);
    }

    fn write_aud2env(&mut self, value: u8) {
        self.channel2.envelope.initial_volume = value >> 4;
        let direction = value & 0x08 != 0;
        self.channel2.envelope.configured_direction = EnvelopeDirection::from(direction);
        self.channel2.envelope.configured_pace = value & 0x07;
        if !self.channel2.dac_enabled() {
            self.channel2.enabled = false;
        }
    }

    const fn write_aud2low(&mut self, value: u8) {
        self.channel2.period_counter.period &= 0x700;
        self.channel2.period_counter.period |= value as u16;
    }

    const fn write_aud2high(&mut self, value: u8) {
        self.channel2.period_counter.period &= 0xFF;
        self.channel2.period_counter.period |= (value as u16 & 0x07) << 8;

        let prev_length_enabled = self.channel2.length_timer.enabled;
        let length_enabled = value & 0x40 != 0;
        self.channel2.length_timer.set_enabled(length_enabled);
        // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
        if !prev_length_enabled
            && length_enabled
            && self.frame % 2 == 0
            && self.channel2.length_timer.tick()
        {
            self.channel2.enabled = false;
        }

        let triggered = value & 0x80 != 0;
        if triggered {
            self.channel2.trigger(self.frame);
        }
    }

    const fn write_aud3ena(&mut self, value: u8) {
        let enabled = value & 0x80 != 0;
        self.channel3.set_dac_enabled(enabled);
    }

    fn write_aud3level(&mut self, value: u8) {
        let level = (value & 0x60) >> 5;
        self.channel3.volume = OutputLevel::from(level);
    }

    const fn write_aud3low(&mut self, value: u8) {
        self.channel3.period_counter.period &= 0x700;
        self.channel3.period_counter.period |= value as u16;
    }

    const fn write_aud3high(&mut self, value: u8) {
        self.channel3.period_counter.period &= 0xFF;
        self.channel3.period_counter.period |= (value as u16 & 0x07) << 8;

        let prev_length_enabled = self.channel3.length_timer.enabled;
        let length_enabled = value & 0x40 != 0;
        self.channel3.length_timer.set_enabled(length_enabled);
        // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
        if !prev_length_enabled
            && length_enabled
            && self.frame % 2 == 0
            && self.channel3.length_timer.tick()
        {
            self.channel3.enabled = false;
        }

        let triggered = value & 0x80 != 0;
        if triggered {
            self.channel3.trigger(self.frame);
        }
    }

    const fn write_aud4len(&mut self, value: u8) {
        let length = value & 0x3F;
        self.channel4.length_timer.load(length);
    }

    fn write_aud4env(&mut self, value: u8) {
        self.channel4.envelope.initial_volume = value >> 4;
        let direction = value & 0x08 != 0;
        self.channel4.envelope.configured_direction = EnvelopeDirection::from(direction);
        self.channel4.envelope.configured_pace = value & 0x07;
        if !self.channel4.dac_enabled() {
            self.channel4.enabled = false;
        }
    }

    const fn write_aud4poly(&mut self, value: u8) {
        self.channel4.frequency_and_randomness = FrequencyAndRandomness::from_bits(value);
    }

    const fn write_aud4go(&mut self, value: u8) {
        let prev_length_enabled = self.channel4.length_timer.enabled;
        let length_enabled = value & 0x40 != 0;
        self.channel4.length_timer.set_enabled(length_enabled);
        // Obscure behaviour: if the length timer is enabled on an even clock it gets ticked
        if !prev_length_enabled
            && length_enabled
            && self.frame % 2 == 0
            && self.channel4.length_timer.tick()
        {
            self.channel4.enabled = false;
        }

        let triggered = value & 0x80 != 0;
        if triggered {
            self.channel4.trigger(self.frame);
        }
    }

    const fn write_audvol(&mut self, value: u8) {
        self.master_volume.vin_left = value & 0x80 != 0;
        self.master_volume.left_volume = (value & 0x70) >> 4;
        self.master_volume.vin_right = value & 0x08 != 0;
        self.master_volume.right_volume = value & 0x07;
    }

    const fn write_audterm(&mut self, value: u8) {
        self.panning.left[3] = value & 0x80 != 0;
        self.panning.left[2] = value & 0x40 != 0;
        self.panning.left[1] = value & 0x20 != 0;
        self.panning.left[0] = value & 0x10 != 0;
        self.panning.right[3] = value & 0x08 != 0;
        self.panning.right[2] = value & 0x04 != 0;
        self.panning.right[1] = value & 0x02 != 0;
        self.panning.right[0] = value & 0x01 != 0;
    }

    const fn write_audena(&mut self, value: u8) {
        let prev_enabled = self.enabled;
        self.enabled = value & 0x80 != 0;
        if prev_enabled && !self.enabled {
            self.power_off();
        }

        if !prev_enabled && self.enabled {
            self.power_on();
        }
    }
}
