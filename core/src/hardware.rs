//! Emulated Game Boy hardware.

use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::cpu::{Cpu, RegisterU8, RegisterU16};
use crate::interrupt::{Interrupt, InterruptEnable, InterruptFlags};
use crate::joypad::Joypad;
use crate::ppu::Ppu;
use crate::serial::SerialPort;
use crate::timer::Timer;

const MEM_DMA: u16 = 0xFF46;
const WORK_RAM_SIZE: usize = 8 * 1024;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;

/// Emulated Game Boy hardware.
pub struct GameboyHardware {
    cpu: Cpu,
    // ROM and External RAM
    cartridge: Cartridge,
    dma: DmaTransfer,
    // Picture Processing Unit
    ppu: Ppu,
    // WRAM
    work_ram: [u8; WORK_RAM_SIZE],
    // P1/JOYP
    joypad: Joypad,
    // Link Cable
    serial_port: SerialPort,
    timer: Timer,
    // IF
    interrupt_flags: InterruptFlags,
    // Audio Processing Unit
    apu: Apu,
    // HRAM
    high_ram: [u8; HIGH_RAM_SIZE],
    // IE
    interrupt_enable: InterruptEnable,
}

impl GameboyHardware {
    /// Creates new Game Boy hardware.
    #[must_use]
    pub const fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu: Cpu::new(),
            cartridge,
            dma: DmaTransfer::empty(),
            ppu: Ppu::new(),
            work_ram: [0; WORK_RAM_SIZE],
            joypad: Joypad::new(),
            serial_port: SerialPort::new(),
            timer: Timer::new(),
            interrupt_flags: InterruptFlags::from_interrupt(Interrupt::VBlank),
            apu: Apu::new(),
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable: InterruptEnable::empty(),
        }
    }

    /// Runs for a few cycles.
    pub fn step(&mut self) {
        let mut bus = AddressBus {
            cartridge: &mut self.cartridge,
            dma: &mut self.dma,
            ppu: &mut self.ppu,
            work_ram: &mut self.work_ram,
            joypad: &mut self.joypad,
            serial_port: &mut self.serial_port,
            timer: &mut self.timer,
            interrupt_flags: &mut self.interrupt_flags,
            apu: &mut self.apu,
            high_ram: &mut self.high_ram,
            interrupt_enable: &mut self.interrupt_enable,
        };

        self.cpu.step(&mut bus);
    }

    /// Returns the value contained in the provided register.
    #[must_use]
    pub const fn register_u8(&self, reg: RegisterU8) -> u8 {
        self.cpu.register_u8(reg)
    }

    /// Returns the value contained in the provided register.
    #[must_use]
    pub const fn register_u16(&self, reg: RegisterU16) -> u16 {
        self.cpu.register_u16(reg)
    }

    /// Returns the value store at the provided address in memory.
    pub fn memory(&mut self, addr: u16) -> u8 {
        let bus = AddressBus {
            cartridge: &mut self.cartridge,
            dma: &mut self.dma,
            ppu: &mut self.ppu,
            work_ram: &mut self.work_ram,
            joypad: &mut self.joypad,
            serial_port: &mut self.serial_port,
            timer: &mut self.timer,
            interrupt_flags: &mut self.interrupt_flags,
            apu: &mut self.apu,
            high_ram: &mut self.high_ram,
            interrupt_enable: &mut self.interrupt_enable,
        };

        bus.read(addr)
    }
}

pub(crate) trait BusInterface {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
    fn tick(&mut self);
    fn highest_priority_interrupt(&self) -> Option<Interrupt>;
    fn acknowledge_interrupt(&mut self, interrupt: Interrupt);
}

pub(crate) struct AddressBus<'a> {
    // ROM and External RAM
    cartridge: &'a mut Cartridge,
    dma: &'a mut DmaTransfer,
    // Picture Processing Unit
    ppu: &'a mut Ppu,
    // WRAM
    work_ram: &'a mut [u8],
    // P1/JOYP
    joypad: &'a mut Joypad,
    // Link Cable
    serial_port: &'a mut SerialPort,
    timer: &'a mut Timer,
    // IF
    interrupt_flags: &'a mut InterruptFlags,
    // Audio Processing Unit
    apu: &'a mut Apu,
    // HRAM
    high_ram: &'a mut [u8],
    // IE
    interrupt_enable: &'a mut InterruptEnable,
}

impl AddressBus<'_> {
    const fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad.bits(),
            0xFF01..=0xFF02 => self.serial_port.read_byte(addr),
            0xFF04..=0xFF07 => self.timer.read_byte(addr),
            0xFF0F => self.interrupt_flags.bits(),
            0xFF10..=0xFF3F => self.apu.read_audio(addr),
            MEM_DMA => self.dma.src_addr,
            0xFF40..=0xFF4B => self.ppu.read_display(addr),
            _ => 0xFF,
        }
    }

    fn write_io(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF00 => *self.joypad = Joypad::from_bits(value),
            0xFF01..=0xFF02 => self.serial_port.write_byte(addr, value),
            0xFF04..=0xFF07 => self.timer.write_byte(addr, value),
            0xFF0F => *self.interrupt_flags = InterruptFlags::from_bits(value),
            0xFF10..=0xFF3F => self.apu.write_audio(addr, value),
            MEM_DMA => *self.dma = DmaTransfer::with_addr(value),
            0xFF40..=0xFF4B => self.ppu.write_display(addr, value),
            _ => {}
        }
    }

    fn sprite_dma_transfer(&mut self) -> bool {
        if let Some(low) = self.dma.transfer() {
            let src_addr = u16::from_le_bytes([low, self.dma.src_addr]);
            let value = self.read(src_addr);
            self.ppu.write_sprite_unchecked(low as u16, value);
        }
        self.dma.update();

        matches!(self.dma.state, TransferState::Run(_))
    }

    const fn is_interrupt_serviceable(&self, interrupt: Interrupt) -> bool {
        self.interrupt_enable.contains(interrupt) && self.interrupt_flags.contains(interrupt)
    }
}

impl BusInterface for AddressBus<'_> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cartridge.read_rom_bank0(addr),
            0x4000..=0x7FFF => self.cartridge.read_rom_bank1(addr - 0x4000),
            0x8000..=0x9FFF => self.ppu.read_vram(addr - 0x8000),
            0xA000..=0xBFFF => self.cartridge.read_ram_bank(addr - 0xA000),
            0xC000..=0xDFFF => self.work_ram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.work_ram[(addr - 0xE000) as usize],
            0xFE00..=0xFEFF => self.ppu.read_sprite(addr - 0xFE00),
            0xFF00..=0xFF7F => self.read_io(addr),
            0xFF80..=0xFFFE => self.high_ram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable.bits(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write_mbc_register(addr, value),
            0x8000..=0x9FFF => self.ppu.write_vram(addr - 0x8000, value),
            0xA000..=0xBFFF => self.cartridge.write_ram_bank(addr - 0xA000, value),
            0xC000..=0xDFFF => self.work_ram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.work_ram[(addr - 0xE000) as usize] = value,
            0xFE00..=0xFEFF => self.ppu.write_sprite(addr - 0xFE00, value),
            0xFF00..=0xFF7F => self.write_io(addr, value),
            0xFF80..=0xFFFE => self.high_ram[(addr - 0xFF80) as usize] = value,
            0xFFFF => *self.interrupt_enable = InterruptEnable::from_bits(value),
        }
    }

    fn tick(&mut self) {
        self.timer.increment_divider(self.interrupt_flags);
        let dma_running = self.sprite_dma_transfer();
        self.ppu.step(self.interrupt_flags, dma_running);
        self.serial_port.step(self.interrupt_flags);
    }

    fn highest_priority_interrupt(&self) -> Option<Interrupt> {
        Interrupt::iter()
            .find(|&&interrupt| self.is_interrupt_serviceable(interrupt))
            .copied()
    }

    fn acknowledge_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_flags.set(interrupt, false);
    }
}

enum TransferState {
    New,
    Setup,
    Run(u8),
    Idle,
}

struct DmaTransfer {
    src_addr: u8,
    state: TransferState,
}

impl DmaTransfer {
    const fn empty() -> Self {
        Self {
            src_addr: 0xFF,
            state: TransferState::Idle,
        }
    }

    const fn with_addr(addr: u8) -> Self {
        Self {
            src_addr: addr,
            state: TransferState::New,
        }
    }

    const fn update(&mut self) {
        match self.state {
            TransferState::New => self.state = TransferState::Setup,
            TransferState::Setup => self.state = TransferState::Run(0),
            TransferState::Run(n) if n < 0x9F => self.state = TransferState::Run(n + 1),
            TransferState::Run(_) => self.state = TransferState::Idle,
            TransferState::Idle => {}
        }
    }

    const fn transfer(&self) -> Option<u8> {
        match self.state {
            TransferState::Run(n) => Some(n),
            _ => None,
        }
    }
}
