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

const WAVE_RAM_SIZE: usize = (WAVE_RAM_END as usize) - (WAVE_RAM_START as usize) + 1;

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
struct VolumeAndEnvelope(u8);

impl VolumeAndEnvelope {
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
struct Control(u8);

impl Control {
    const LENGTH_ENABLE: u8 = 0b0100_0000;
    const UNUSED: u8 = 0b1011_1111;

    const fn new() -> Self {
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
struct DacEnable(u8);

impl DacEnable {
    const ENABLE: u8 = 0b1000_0000;
    const UNUSED: u8 = 0b0111_1111;

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

#[derive(Debug, Copy, Clone)]
struct MasterControl(u8);

impl MasterControl {
    const AUDIO_ENABLE: u8 = 0b1000_0000;
    const CHANNEL_4_ENABLE: u8 = 0b0000_1000;
    const CHANNEL_3_ENABLE: u8 = 0b0000_0100;
    const CHANNEL_2_ENABLE: u8 = 0b0000_0010;
    const CHANNEL_1_ENABLE: u8 = 0b0000_0001;
    const UNUSED: u8 = 0b0111_0000;

    const fn new() -> Self {
        Self::from_bits(Self::AUDIO_ENABLE | Self::CHANNEL_1_ENABLE)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

struct Channel1 {
    // NR10
    sweep: Sweep,
    duty_cycle: DutyCycle,
    length_timer: u8,
    // NR12
    volume_and_envelope: VolumeAndEnvelope,
    period: Period,
    control: Control,
}

impl Channel1 {
    const fn new() -> Self {
        Self {
            sweep: Sweep::empty(),
            duty_cycle: DutyCycle::from_bits(0b1000_0000),
            length_timer: 0,
            volume_and_envelope: VolumeAndEnvelope::from_bits(
                VolumeAndEnvelope::INITIAL_VOLUME | 0b11,
            ),
            period: Period::empty(),
            control: Control::new(),
        }
    }
}

struct Channel2 {
    duty_cycle: DutyCycle,
    length_timer: u8,
    // NR22
    volume_and_envelope: VolumeAndEnvelope,
    period: Period,
    control: Control,
}

impl Channel2 {
    const fn new() -> Self {
        Self {
            duty_cycle: DutyCycle::empty(),
            length_timer: 0,
            volume_and_envelope: VolumeAndEnvelope::empty(),
            period: Period::empty(),
            control: Control::new(),
        }
    }
}

struct Channel3 {
    // NR30
    dac_enable: DacEnable,
    // NR31
    length_timer: u8,
    // NR32
    output_level: OutputLevel,
    period: Period,
    control: Control,
}

impl Channel3 {
    const fn new() -> Self {
        Self {
            dac_enable: DacEnable::empty(),
            length_timer: 0xFF,
            output_level: OutputLevel::empty(),
            period: Period::empty(),
            control: Control::new(),
        }
    }
}

struct Channel4 {
    // NR41
    length_timer: u8,
    // NR42
    volume_and_envelope: VolumeAndEnvelope,
    // NR43
    frequency_and_randomness: FrequencyAndRandomness,
    // NR44
    control: Control,
}

impl Channel4 {
    const fn new() -> Self {
        Self {
            length_timer: 0,
            volume_and_envelope: VolumeAndEnvelope::empty(),
            frequency_and_randomness: FrequencyAndRandomness::empty(),
            control: Control::new(),
        }
    }
}

pub struct Apu {
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    // NR50
    master_volume: MasterVolume,
    // NR51
    sound_panning: SoundPanning,
    // NR52
    audio_master_control: MasterControl,
    wave_ram: [u8; WAVE_RAM_SIZE],
}

impl Apu {
    pub const fn new() -> Self {
        Self {
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            channel4: Channel4::new(),
            master_volume: MasterVolume::new(),
            sound_panning: SoundPanning::new(),
            audio_master_control: MasterControl::new(),
            wave_ram: [0xFF; WAVE_RAM_SIZE],
        }
    }

    pub const fn read_audio(&self, addr: u16) -> u8 {
        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep.bits(),
            MEM_AUD1LEN => self.channel1.duty_cycle.bits(),
            MEM_AUD1ENV => self.channel1.volume_and_envelope.bits(),
            MEM_AUD1HIGH => self.channel1.control.bits(),
            MEM_AUD2LEN => self.channel2.duty_cycle.bits(),
            MEM_AUD2ENV => self.channel2.volume_and_envelope.bits(),
            MEM_AUD2HIGH => self.channel2.control.bits(),
            MEM_AUD3ENA => self.channel3.dac_enable.bits(),
            MEM_AUD3LEVEL => self.channel3.output_level.bits(),
            MEM_AUD3HIGH => self.channel3.control.bits(),
            MEM_AUD4ENV => self.channel4.volume_and_envelope.bits(),
            MEM_AUD4POLY => self.channel4.frequency_and_randomness.bits(),
            MEM_AUD4GO => self.channel4.control.bits(),
            MEM_AUDVOL => self.master_volume.bits(),
            MEM_AUDTERM => self.sound_panning.bits(),
            MEM_AUDENA => self.audio_master_control.bits(),
            WAVE_RAM_START..=WAVE_RAM_END => self.wave_ram[(addr - WAVE_RAM_START) as usize],
            _ => {
                // NR13, NR23, NR31, NR33, NR41 are write-only
                0xFF
            }
        }
    }

    pub const fn write_audio(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_AUD1SWEEP => self.channel1.sweep = Sweep::from_bits(value),
            MEM_AUD1LEN => {
                self.channel1.duty_cycle = DutyCycle::from_bits(value);
                self.channel1.length_timer = value & 0x3F;
            }
            MEM_AUD1ENV => self.channel1.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_AUD1LOW => self.channel1.period = self.channel1.period.replace_low(value),
            MEM_AUD1HIGH => {
                let _trigger = value & 0x80 == 0x80;
                self.channel1.period = self.channel1.period.replace_high(value);
                self.channel1.control = Control::from_bits(value);
            }
            MEM_AUD2LEN => {
                self.channel2.duty_cycle = DutyCycle::from_bits(value);
                self.channel2.length_timer = value & 0x3F;
            }
            MEM_AUD2ENV => self.channel2.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_AUD2LOW => self.channel2.period = self.channel2.period.replace_low(value),
            MEM_AUD2HIGH => {
                let _trigger = value & 0x80 == 0x80;
                self.channel2.period = self.channel2.period.replace_high(value);
                self.channel2.control = Control::from_bits(value);
            }
            MEM_AUD3ENA => self.channel3.dac_enable = DacEnable::from_bits(value),
            MEM_AUD3LEN => self.channel3.length_timer = value,
            MEM_AUD3LEVEL => self.channel3.output_level = OutputLevel::from_bits(value),
            MEM_AUD3LOW => self.channel3.period = self.channel3.period.replace_low(value),
            MEM_AUD3HIGH => {
                let _trigger = value & 0x80 == 0x80;
                self.channel3.period = self.channel3.period.replace_high(value);
                self.channel3.control = Control::from_bits(value);
            }
            MEM_AUD4LEN => self.channel4.length_timer = value & 0x3F,
            MEM_AUD4ENV => self.channel4.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_AUD4POLY => {
                self.channel4.frequency_and_randomness = FrequencyAndRandomness::from_bits(value);
            }
            MEM_AUD4GO => {
                let _trigger = value & 0x80 == 0x80;
                self.channel4.control = Control::from_bits(value);
            }
            MEM_AUDVOL => self.master_volume = MasterVolume::from_bits(value),
            MEM_AUDTERM => self.sound_panning = SoundPanning::from_bits(value),
            MEM_AUDENA => self.audio_master_control = MasterControl::from_bits(value),
            WAVE_RAM_START..=WAVE_RAM_END => {
                self.wave_ram[(addr - WAVE_RAM_START) as usize] = value;
            }
            _ => {}
        }
    }
}
