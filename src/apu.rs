const MEM_NR10: u16 = 0xFF10;
const MEM_NR11: u16 = 0xFF11;
const MEM_NR12: u16 = 0xFF12;
const MEM_NR13: u16 = 0xFF13;
const MEM_NR14: u16 = 0xFF14;
const MEM_NR21: u16 = 0xFF16;
const MEM_NR22: u16 = 0xFF17;
const MEM_NR23: u16 = 0xFF18;
const MEM_NR24: u16 = 0xFF19;
const MEM_NR30: u16 = 0xFF1A;
const MEM_NR31: u16 = 0xFF1B;
const MEM_NR32: u16 = 0xFF1C;
const MEM_NR33: u16 = 0xFF1D;
const MEM_NR34: u16 = 0xFF1E;
const MEM_NR41: u16 = 0xFF20;
const MEM_NR42: u16 = 0xFF21;
const MEM_NR43: u16 = 0xFF22;
const MEM_NR44: u16 = 0xFF23;
const MEM_NR50: u16 = 0xFF24;
const MEM_NR51: u16 = 0xFF25;
const MEM_NR52: u16 = 0xFF26;

#[derive(Debug, Copy, Clone)]
struct ChannelSweep(u8);

impl ChannelSweep {
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
struct LengthTimerAndDutyCycle(u8);

impl LengthTimerAndDutyCycle {
    const WAVE_DUTY: u8 = 0b1100_0000;
    const INITIAL_LENGTH_TIMER: u8 = 0b0011_1111;

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
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
struct PeriodHighAndControl(u8);

impl PeriodHighAndControl {
    const TRIGGER: u8 = 0b1000_0000;
    const LENGTH_ENABLE: u8 = 0b0100_0000;
    const PERIOD: u8 = 0b0000_0111;
    const UNUSED: u8 = 0b0011_1000;

    const fn new() -> Self {
        Self::from_bits(Self::TRIGGER | Self::PERIOD)
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
struct LengthTimer(u8);

impl LengthTimer {
    const INITIAL_LENGTH_TIMER: u8 = 0b0011_1111;
    const UNUSED: u8 = 0b1100_0000;

    const fn new() -> Self {
        Self::from_bits(Self::INITIAL_LENGTH_TIMER)
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
struct Control(u8);

impl Control {
    const TRIGGER: u8 = 0b1000_0000;
    const LENGTH_ENABLE: u8 = 0b0100_0000;
    const UNUSED: u8 = 0b0011_1111;

    const fn new() -> Self {
        Self::from_bits(Self::TRIGGER)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
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
struct AudioMasterControl(u8);

impl AudioMasterControl {
    const AUDIO_ENABLE: u8 = 0b1000_0000;
    const CHANNEL_4_ENABLE: u8 = 0b0000_1000;
    const CHANNEL_3_ENABLE: u8 = 0b0000_0100;
    const CHANNEL_2_ENABLE: u8 = 0b0000_0010;
    const CHANNEL_1_ENABLE: u8 = 0b0000_0001;
    const UNUSED: u8 = 0b0111_0000;

    const fn new() -> Self {
        Self::from_bits(Self::AUDIO_ENABLE | Self::CHANNEL_4_ENABLE)
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
    sweep: ChannelSweep,
    // NR11
    length_timer_and_duty_cycle: LengthTimerAndDutyCycle,
    // NR12
    volume_and_envelope: VolumeAndEnvelope,
    // NR13
    period_low: u8,
    // NR14
    period_high_and_control: PeriodHighAndControl,
}

impl Channel1 {
    const fn new() -> Self {
        Self {
            sweep: ChannelSweep::empty(),
            length_timer_and_duty_cycle: LengthTimerAndDutyCycle::from_bits(
                0b1000_0000 | LengthTimerAndDutyCycle::INITIAL_LENGTH_TIMER,
            ),
            volume_and_envelope: VolumeAndEnvelope::from_bits(
                VolumeAndEnvelope::INITIAL_VOLUME | 0b11,
            ),
            period_low: 0xFF,
            period_high_and_control: PeriodHighAndControl::new(),
        }
    }
}

struct Channel2 {
    // NR21
    length_timer_and_duty_cycle: LengthTimerAndDutyCycle,
    // NR22
    volume_and_envelope: VolumeAndEnvelope,
    // NR23
    period_low: u8,
    // NR24
    period_high_and_control: PeriodHighAndControl,
}

impl Channel2 {
    const fn new() -> Self {
        Self {
            length_timer_and_duty_cycle: LengthTimerAndDutyCycle::from_bits(
                LengthTimerAndDutyCycle::INITIAL_LENGTH_TIMER,
            ),
            volume_and_envelope: VolumeAndEnvelope::empty(),
            period_low: 0xFF,
            period_high_and_control: PeriodHighAndControl::new(),
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
    // NR33
    period_low: u8,
    // NR34
    period_high_and_control: PeriodHighAndControl,
}

impl Channel3 {
    const fn new() -> Self {
        Self {
            dac_enable: DacEnable::empty(),
            length_timer: 0xFF,
            output_level: OutputLevel::empty(),
            period_low: 0xFF,
            period_high_and_control: PeriodHighAndControl::new(),
        }
    }
}

struct Channel4 {
    // NR41
    length_timer: LengthTimer,
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
            length_timer: LengthTimer::new(),
            volume_and_envelope: VolumeAndEnvelope::empty(),
            frequency_and_randomness: FrequencyAndRandomness::empty(),
            control: Control::new(),
        }
    }
}

pub struct Apu {
    channel_1: Channel1,
    channel_2: Channel2,
    channel_3: Channel3,
    channel_4: Channel4,
    // NR50
    master_volume: MasterVolume,
    // NR51
    sound_panning: SoundPanning,
    // NR52
    audio_master_control: AudioMasterControl,
}

impl Apu {
    pub const fn new() -> Self {
        Self {
            channel_1: Channel1::new(),
            channel_2: Channel2::new(),
            channel_3: Channel3::new(),
            channel_4: Channel4::new(),
            master_volume: MasterVolume::new(),
            sound_panning: SoundPanning::new(),
            audio_master_control: AudioMasterControl::new(),
        }
    }

    pub fn read_audio(&self, addr: u16) -> u8 {
        match addr {
            MEM_NR10 => self.channel_1.sweep.bits(),
            MEM_NR11 => self.channel_1.length_timer_and_duty_cycle.bits(),
            MEM_NR12 => self.channel_1.volume_and_envelope.bits(),
            MEM_NR13 => self.channel_1.period_low,
            MEM_NR14 => self.channel_1.period_high_and_control.bits(),
            MEM_NR21 => self.channel_2.length_timer_and_duty_cycle.bits(),
            MEM_NR22 => self.channel_2.volume_and_envelope.bits(),
            MEM_NR23 => self.channel_2.period_low,
            MEM_NR24 => self.channel_2.period_high_and_control.bits(),
            MEM_NR30 => self.channel_3.dac_enable.bits(),
            MEM_NR31 => self.channel_3.length_timer,
            MEM_NR32 => self.channel_3.output_level.bits(),
            MEM_NR33 => self.channel_3.period_low,
            MEM_NR34 => self.channel_3.period_high_and_control.bits(),
            MEM_NR41 => self.channel_4.length_timer.bits(),
            MEM_NR42 => self.channel_4.volume_and_envelope.bits(),
            MEM_NR43 => self.channel_4.frequency_and_randomness.bits(),
            MEM_NR44 => self.channel_4.control.bits(),
            MEM_NR50 => self.master_volume.bits(),
            MEM_NR51 => self.sound_panning.bits(),
            MEM_NR52 => self.audio_master_control.bits(),
            _ => {
                println!("Warning: Address {addr:#X} is not mapped to an I/O register.");
                0xFF
            }
        }
    }

    pub fn write_audio(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_NR10 => self.channel_1.sweep = ChannelSweep::from_bits(value),
            MEM_NR11 => {
                self.channel_1.length_timer_and_duty_cycle =
                    LengthTimerAndDutyCycle::from_bits(value);
            }
            MEM_NR12 => self.channel_1.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_NR13 => self.channel_1.period_low = value,
            MEM_NR14 => {
                self.channel_1.period_high_and_control = PeriodHighAndControl::from_bits(value);
            }
            MEM_NR21 => {
                self.channel_2.length_timer_and_duty_cycle =
                    LengthTimerAndDutyCycle::from_bits(value);
            }
            MEM_NR22 => self.channel_2.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_NR23 => self.channel_2.period_low = value,
            MEM_NR24 => {
                self.channel_2.period_high_and_control = PeriodHighAndControl::from_bits(value);
            }
            MEM_NR30 => self.channel_3.dac_enable = DacEnable::from_bits(value),
            MEM_NR31 => self.channel_3.length_timer = value,
            MEM_NR32 => self.channel_3.output_level = OutputLevel::from_bits(value),
            MEM_NR33 => self.channel_3.period_low = value,
            MEM_NR34 => {
                self.channel_3.period_high_and_control = PeriodHighAndControl::from_bits(value);
            }
            MEM_NR41 => self.channel_4.length_timer = LengthTimer::from_bits(value),
            MEM_NR42 => self.channel_4.volume_and_envelope = VolumeAndEnvelope::from_bits(value),
            MEM_NR43 => {
                self.channel_4.frequency_and_randomness = FrequencyAndRandomness::from_bits(value);
            }
            MEM_NR44 => self.channel_4.control = Control::from_bits(value),
            MEM_NR50 => self.master_volume = MasterVolume::from_bits(value),
            MEM_NR51 => self.sound_panning = SoundPanning::from_bits(value),
            MEM_NR52 => self.audio_master_control = AudioMasterControl::from_bits(value),
            _ => println!("Warning: Address {addr:#X} is not mapped to an I/O register."),
        }
    }
}
