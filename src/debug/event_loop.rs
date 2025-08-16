use crate::debug::target::GameBoyTarget;
use std::io::{Write, stdin, stdout};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn event_loop(target: GameBoyTarget) {
    match run_blocking(target) {
        Ok(disconnect_reason) => match disconnect_reason {
            ExitReason::Quit => println!("Debugger exited normally"),
        },
        Err(err) => {
            println!("Debugger exited with error: {err}");
        }
    }
}

enum ExitReason {
    Quit,
}

fn run_blocking(mut target: GameBoyTarget) -> Result<ExitReason, Box<dyn std::error::Error>> {
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
                    target.add_breakpoint(addr);
                }
            }
            "catch" => {
                if let Ok(opcode) = parse_numeric(words[1]) {
                    target.add_catchpoint(opcode);
                }
            }
            "continue" | "c" => {
                target.continue_mode();
                target.run();
            }
            "clear" => target.clear_breakpoints(),
            "delete" | "del" | "d" => {
                if let Ok(idx) = words[1].parse::<usize>() {
                    target.remove_breakpoint(idx);
                }
            }
            "disassemble" | "disass" => target.disassemble(5),
            "exit" | "quit" | "q" => return Ok(ExitReason::Quit),
            "info" | "i" => match words[1] {
                "breakpoints" => target.print_breakpoints(),
                "mem" => {
                    if let Ok(addr) = parse_numeric(words[2]) {
                        target.print_addrs(addr, 16);
                    }
                }
                "registers" | "reg" | "r" => target.print_regs(),
                _ => println!("Unknown command: {}", words[1]),
            },
            "next" | "n" => {
                target.step_mode();
                target.run();
            }
            _ => println!("Unknown command: {}", words[0]),
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
