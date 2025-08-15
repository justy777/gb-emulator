use gb_core::hardware::GameboyHardware;
use gb_core::{RegisterU8, RegisterU16};
use std::cmp::min;
use std::fmt::Display;
use std::io::{Write, stdin, stdout};
use std::num::ParseIntError;
use std::str::FromStr;

#[rustfmt::skip]
const OPCODE_NAMES: [&str; 0x100] = [
    "NOP",          "LD BC, u16",   "LD (BC), A",   "INC BC",       "INC B",        "DEC B",        "LD B, u8",     "RLCA",         // $00
    "LD (u16), SP", "ADD HL, BC",   "LD A, (BC)",   "DEC BC",       "INC C",        "DEC C",        "LD C, u8",     "RRCA",         // $08
    "STOP",         "LD DE, u16",   "LD (DE), A",   "INC DE",       "INC D",        "DEC D",        "LD D, u8",     "RLA",          // $10
    "JR i8",        "ADD HL, DE",   "LD A, (DE)",   "DEC DE",       "INC E",        "DEC E",        "LD E, u8",     "RRA",          // $18
    "JR NZ, i8",    "LD HL, u16",   "LD (HL+), A",  "INC HL",       "INC H",        "DEC H",        "LD H, u8",     "DAA",          // $20
    "JR Z, i8",     "ADD HL, HL",   "LD A, (HL+)",  "DEC HL",       "INC L",        "DEC L",        "LD L, u8",     "CPL",          // $28
    "JR NC, i8",    "LD SP, u16",   "LD (HL-), A",  "INC SP",       "INC (HL)",     "DEC (HL)",     "LD (HL), u8",  "SCF",          // $30
    "JR C, i8",     "ADD HL, SP",   "LD A, (HL-)",  "DEC SP",       "INC A",        "DEC A",        "LD A, u8",     "CCF",          // $38
    "LD B, B",      "LD B, C",      "LD B, D",      "LD B, E",      "LD B, H",      "LD B, L",      "LD B, (HL)",   "LD B, A",      // $40
    "LD C, B",      "LD C, C",      "LD C, D",      "LD C, E",      "LD C, H",      "LD C, L",      "LD C, (HL)",   "LD C, A",      // $48
    "LD D, B",      "LD D, C",      "LD D, D",      "LD D, E",      "LD D, H",      "LD D, L",      "LD D, (HL)",   "LD D, A",      // $50
    "LD E, B",      "LD E, C",      "LD E, D",      "LD E, E",      "LD E, H",      "LD E, L",      "LD E, (HL)",   "LD E, A",      // $58
    "LD H, B",      "LD H, C",      "LD H, D",      "LD H, E",      "LD H, H",      "LD H, L",      "LD H, (HL)",   "LD H, A",      // $60
    "LD L, B",      "LD L, C",      "LD L, D",      "LD L, E",      "LD L, H",      "LD L, L",      "LD L, (HL)",   "LD L, A",      // $68
    "LD (HL), B",   "LD (HL), C",   "LD (HL), D",   "LD (HL), E",   "LD (HL), H",   "LD (HL), L",   "HALT",         "LD (HL), A",   // $70
    "LD A, B",      "LD A, C",      "LD A, D",      "LD A, E",      "LD A, H",      "LD A, L",      "LD A, (HL)",   "LD A, A",      // $78
    "ADD A, B",     "ADD A, C",     "ADD A, D",     "ADD A, E",     "ADD A, H",     "ADD A, L",     "ADD A, (HL)",  "ADD A, A",     // $80
    "ADC A, B",     "ADC A, C",     "ADC A, D",     "ADC A, E",     "ADC A, H",     "ADC A, L",     "ADC A, (HL)",  "ADC A, A",     // $88
    "SUB B",        "SUB C",        "SUB D",        "SUB E",        "SUB H",        "SUB L",        "SUB (HL)",     "SUB A",        // $90
    "SBC B",        "SBC C",        "SBC D",        "SBC E",        "SBC H",        "SBC L",        "SBC (HL)",     "SBC A",        // $98
    "AND B",        "AND C",        "AND D",        "AND E",        "AND H",        "AND L",        "AND (HL)",     "AND A",        // $A0
    "XOR B",        "XOR C",        "XOR D",        "XOR E",        "XOR H",        "XOR L",        "XOR (HL)",     "XOR A",        // $A8
    "OR B",         "OR C",         "OR D",         "OR E",         "OR H",         "OR L",         "OR (HL)",      "OR A",         // $B0
    "CP B",         "CP C",         "CP D",         "CP E",         "CP H",         "CP L",         "CP (HL)",      "CP A",         // $B8
    "RET NZ",       "POP BC",       "JP NZ, u16",   "JP u16",       "CALL NZ, u16", "PUSH BC",      "AND A, u8",    "RST 00",       // $C0
    "RET Z",        "RET",          "JP Z, u16",    "PREFIX CB",    "CALL Z, u16",  "CALL u16",     "ADC A, u8",    "RST 08",       // $C8
    "RET NC",       "POP DE",       "JP NC, u16",   "INVALID",      "CALL NC, u16", "PUSH DE",      "SUB u8",       "RST 10",       // $D0
    "RET C",        "RETI",         "JP C, u16",    "INVALID",      "CALL C, u16",  "INVALID",      "SBC A, u8",    "RST 18",       // $D8
    "LDH (a8), A",  "POP HL",       "LD (C), A",    "INVALID",      "INVALID",      "PUSH HL",      "AND u8",       "RST 20",       // $E0
    "ADD SP, i8",   "JP (HL)",      "LD (u16), A",  "INVALID",      "INVALID",      "INVALID",      "XOR u8",       "RST 28",       // $E8
    "LDH A, (a8)",  "POP AF",       "LD A, (C)",    "DI",           "INVALID",      "PUSH AF",      "OR u8",        "RST 30",       // $F0
    "LD HL, SP+i8", "LD SP, HL",    "LD A, (u16)",  "EI",           "INVALID",      "INVALID",      "CP u8",        "RST 38"        // $F8
];

const OPCODE_LENGTHS: [u16; 0x100] = [
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1,
    2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
];

#[rustfmt::skip]
const PREFIX_OPCODE_NAMES: [&str; 0x100] = [
    "RLC B",    "RLC C",    "RLC D",    "RLC E",    "RLC H",    "RLC L",    "RLC (HL)",    "RLC A",    // $00
    "RRC B",    "RRC C",    "RRC D",    "RRC E",    "RRC H",    "RRC L",    "RRC (HL)",    "RRC A",    // $08
    "RL B",     "RL C",     "RL D",     "RL E",     "RL H",     "RL L",     "RL (HL)",     "RL A",     // $10
    "RR B",     "RR C",     "RR D",     "RR E",     "RR H",     "RR L",     "RR (HL)",     "RR A",     // $18
    "SLA B",    "SLA C",    "SLA D",    "SLA E",    "SLA H",    "SLA L",    "SLA (HL)",    "SLA A",    // $20
    "SRA B",    "SRA C",    "SRA D",    "SRA E",    "SRA H",    "SRA L",    "SRA (HL)",    "SRA A",    // $28
    "SWAP B",   "SWAP C",   "SWAP D",   "SWAP E",   "SWAP H",   "SWAP L",   "SWAP (HL)",   "SWAP A",   // $30
    "SRL B",    "SRL C",    "SRL D",    "SRL E",    "SRL H",    "SRL L",    "SRL (HL)",    "SRL A",    // $38
    "BIT 0, B", "BIT 0, C", "BIT 0, D", "BIT 0, E", "BIT 0, H", "BIT 0, L", "BIT 0, (HL)", "BIT 0, A", // $40
    "BIT 1, B", "BIT 1, C", "BIT 1, D", "BIT 1, E", "BIT 1, H", "BIT 1, L", "BIT 1, (HL)", "BIT 1, A", // $48
    "BIT 2, B", "BIT 2, C", "BIT 2, D", "BIT 2, E", "BIT 2, H", "BIT 2, L", "BIT 2, (HL)", "BIT 2, A", // $50
    "BIT 3, B", "BIT 3, C", "BIT 3, D", "BIT 3, E", "BIT 3, H", "BIT 3, L", "BIT 3, (HL)", "BIT 3, A", // $58
    "BIT 4, B", "BIT 4, C", "BIT 4, D", "BIT 4, E", "BIT 4, H", "BIT 4, L", "BIT 4, (HL)", "BIT 4, A", // $60
    "BIT 5, B", "BIT 5, C", "BIT 5, D", "BIT 5, E", "BIT 5, H", "BIT 5, L", "BIT 5, (HL)", "BIT 5, A", // $68
    "BIT 6, B", "BIT 6, C", "BIT 6, D", "BIT 6, E", "BIT 6, H", "BIT 6, L", "BIT 6, (HL)", "BIT 6, A", // $70
    "BIT 7, B", "BIT 7, C", "BIT 7, D", "BIT 7, E", "BIT 7, H", "BIT 7, L", "BIT 7, (HL)", "BIT 7, A", // $78
    "RES 0, B", "RES 0, C", "RES 0, D", "RES 0, E", "RES 0, H", "RES 0, L", "RES 0, (HL)", "RES 0, A", // $80
    "RES 1, B", "RES 1, C", "RES 1, D", "RES 1, E", "RES 1, H", "RES 1, L", "RES 1, (HL)", "RES 1, A", // $88
    "RES 2, B", "RES 2, C", "RES 2, D", "RES 2, E", "RES 2, H", "RES 2, L", "RES 2, (HL)", "RES 2, A", // $90
    "RES 3, B", "RES 3, C", "RES 3, D", "RES 3, E", "RES 3, H", "RES 3, L", "RES 3, (HL)", "RES 3, A", // $98
    "RES 4, B", "RES 4, C", "RES 4, D", "RES 4, E", "RES 4, H", "RES 4, L", "RES 4, (HL)", "RES 4, A", // $A0
    "RES 5, B", "RES 5, C", "RES 5, D", "RES 5, E", "RES 5, H", "RES 5, L", "RES 5, (HL)", "RES 5, A", // $A8
    "RES 6, B", "RES 6, C", "RES 6, D", "RES 6, E", "RES 6, H", "RES 6, L", "RES 6, (HL)", "RES 6, A", // $B0
    "RES 7, B", "RES 7, C", "RES 7, D", "RES 7, E", "RES 7, H", "RES 7, L", "RES 7, (HL)", "RES 7, A", // $B8
    "SET 0, B", "SET 0, C", "SET 0, D", "SET 0, E", "SET 0, H", "SET 0, L", "SET 0, (HL)", "SET 0, A", // $C0
    "SET 1, B", "SET 1, C", "SET 1, D", "SET 1, E", "SET 1, H", "SET 1, L", "SET 1, (HL)", "SET 1, A", // $C8
    "SET 2, B", "SET 2, C", "SET 2, D", "SET 2, E", "SET 2, H", "SET 2, L", "SET 2, (HL)", "SET 2, A", // $D0
    "SET 3, B", "SET 3, C", "SET 3, D", "SET 3, E", "SET 3, H", "SET 3, L", "SET 3, (HL)", "SET 3, A", // $D8
    "SET 4, B", "SET 4, C", "SET 4, D", "SET 4, E", "SET 4, H", "SET 4, L", "SET 4, (HL)", "SET 4, A", // $E0
    "SET 5, B", "SET 5, C", "SET 5, D", "SET 5, E", "SET 5, H", "SET 5, L", "SET 5, (HL)", "SET 5, A", // $E8
    "SET 6, B", "SET 6, C", "SET 6, D", "SET 6, E", "SET 6, H", "SET 6, L", "SET 6, (HL)", "SET 6, A", // $F0
    "SET 7, B", "SET 7, C", "SET 7, D", "SET 7, E", "SET 7, H", "SET 7, L", "SET 7, (HL)", "SET 7, A", // $F8
];

#[derive(Debug, Clone)]
enum Point {
    Break(u16),
    Catch(u8),
    Watch { addr: u16, old_value: u8 },
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Break(addr) => write!(f, "Break: {addr:#06X}"),
            Self::Catch(addr) => write!(f, "Catch: {addr:#04x}"),
            Self::Watch { addr, .. } => write!(f, "Watch: {addr:#06X}"),
        }
    }
}

pub struct Debugger {
    points: Vec<Point>,
}

impl Debugger {
    pub const fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn debug(&mut self, gb: &mut GameboyHardware) -> std::io::Result<bool> {
        loop {
            print!("(debug) ");
            stdout().flush()?;

            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let input = input.to_lowercase();
            let words: Vec<&str> = input.split_whitespace().collect();

            match words[0] {
                "break" | "b" => {
                    if let Ok(addr) = parse_numeric(words[1]) {
                        self.points.push(Point::Break(addr));
                    }
                }
                "catch" => {
                    if let Ok(opcode) = parse_numeric(words[1]) {
                        self.points.push(Point::Catch(opcode));
                    }
                }
                "continue" | "c" => return Ok(false),
                "clear" => self.points.clear(),
                "delete" | "d" => {
                    if let Ok(idx) = words[1].parse::<usize>() {
                        self.points.remove(idx);
                    }
                }
                "disassemble" | "disass" => disassemble(gb),
                "exit" | "quit" | "q" => return Ok(true),
                "info" | "i" => match words[1] {
                    "points" | "p" => self.print_points(),
                    "mem" => {
                        if let Ok(addr) = parse_numeric(words[2]) {
                            print_memory(gb, addr);
                        }
                    }
                    "registers" | "reg" | "r" => print_registers(gb),
                    _ => println!("Unknown command: {}", words[1]),
                },
                "next" | "n" => {
                    gb.step();
                    println!("{:#06X}", gb.register_u16(RegisterU16::PC));
                }
                "watch" => {
                    if let Ok(addr) = parse_numeric(words[1]) {
                        let value = gb.memory(addr);
                        self.points.push(Point::Watch {
                            addr,
                            old_value: value,
                        });
                    }
                }
                _ => println!("Unknown command: {}", words[0]),
            }
        }
    }

    fn print_points(&self) {
        if self.points.is_empty() {
            println!("No breakpoints, catchpoints, or watchpoints.");
            return;
        }

        self.points
            .iter()
            .enumerate()
            .for_each(|(idx, point)| println!("{idx} {point}"));
    }

    pub fn check_points(&self, gb: &mut GameboyHardware) -> bool {
        let pc = gb.register_u16(RegisterU16::PC);
        self.points.iter().any(|point| match point {
            Point::Break(addr) => *addr == pc,
            Point::Catch(opcode) => {
                let instr = gb.memory(pc);
                *opcode == instr
            }
            Point::Watch { addr, old_value } => {
                let value = gb.memory(*addr);
                value != *old_value
            }
        })
    }

    pub fn update_watchpoints(&mut self, gb: &mut GameboyHardware) {
        for point in &mut self.points {
            if let Point::Watch { addr, old_value } = point {
                let value = gb.memory(*addr);
                if value != *old_value {
                    *point = Point::Watch {
                        addr: *addr,
                        old_value: value,
                    };
                }
            }
        }
    }
}

trait FromStrRadix
where
    Self: Sized,
{
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;
}

impl FromStrRadix for u8 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, radix)
    }
}

impl FromStrRadix for u16 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, radix)
    }
}

fn parse_numeric<T>(input: &str) -> Result<T, ParseIntError>
where
    T: FromStrRadix + FromStr<Err = ParseIntError>,
{
    let input = input.replace('_', "");
    // Hex
    if let Some(input) = ["$", "0x", "0X"]
        .into_iter()
        .find_map(|pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 16)
    }
    // Octal
    else if let Some(input) = ["&", "0o", "0O"]
        .into_iter()
        .find_map(|pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 8)
    }
    // Binary
    else if let Some(input) = ["%", "0b", "0B"]
        .into_iter()
        .find_map(|pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 2)
    } else {
        input.parse::<T>()
    }
}

fn print_registers(gb: &GameboyHardware) {
    println!("A  {:#04X}", gb.register_u8(RegisterU8::A));
    println!("F  {:#04X}", gb.register_u8(RegisterU8::F));
    println!("B  {:#04X}", gb.register_u8(RegisterU8::B));
    println!("C  {:#04X}", gb.register_u8(RegisterU8::C));
    println!("D  {:#04X}", gb.register_u8(RegisterU8::D));
    println!("E  {:#04X}", gb.register_u8(RegisterU8::E));
    println!("H  {:#04X}", gb.register_u8(RegisterU8::H));
    println!("L  {:#04X}", gb.register_u8(RegisterU8::L));
    println!("SP {:#06X}", gb.register_u16(RegisterU16::SP));
    println!("PC {:#06X}", gb.register_u16(RegisterU16::PC));
}

fn print_memory(gb: &mut GameboyHardware, addr: u16) {
    let end = min(addr + 16, 0xFFFF);
    for i in addr..end {
        println!("{i:#06X} {:#04X}", gb.memory(i));
    }
}

fn disassemble(gb: &mut GameboyHardware) {
    let mut pc = gb.register_u16(RegisterU16::PC);
    for _ in 0..5 {
        let opcode = gb.memory(pc) as usize;
        let (name, len) = if opcode == 0xCB {
            let opcode = gb.memory(pc + 1) as usize;
            (PREFIX_OPCODE_NAMES[opcode], 2)
        } else {
            (OPCODE_NAMES[opcode], OPCODE_LENGTHS[opcode])
        };
        print!("{pc:#06X} {name}");
        for i in 0..len {
            let arg = gb.memory(pc + i);
            print!(" {arg:#04X}");
        }
        println!();
        pc += len;
    }
}
