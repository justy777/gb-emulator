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
        let words: Vec<String> = input.split_whitespace().map(str::to_lowercase).collect();

        if let Some(command) = words.first() {
            match command.as_str() {
                "break" => {
                    if let Some(num) = words.get(1)
                        && let Ok(addr) = parse_numeric(num)
                    {
                        target.add_breakpoint(addr);
                    }
                }
                "catch" => {
                    if let Some(num) = words.get(1)
                        && let Ok(opcode) = parse_numeric(num)
                    {
                        target.add_catchpoint(opcode);
                    }
                }
                "continue" | "c" => {
                    target.continue_mode();
                    target.run();
                }
                "delete" | "del" => match words.get(1) {
                    Some(num) => {
                        if let Ok(idx) = parse_numeric(num) {
                            target.remove_breakpoint(idx);
                        }
                    }
                    None => target.clear_breakpoints(),
                },
                "disassemble" => target.disassemble(5),
                "exit" | "quit" | "q" => return Ok(ExitReason::Quit),
                "info" => {
                    if let Some(sub) = words.get(1) {
                        match sub.as_str() {
                            "address" => {
                                if let Some(num) = words.get(2)
                                    && let Ok(addr) = parse_numeric(num)
                                {
                                    target.print_addrs(addr, 16);
                                }
                            }
                            "breakpoints" | "breaks" => target.print_breakpoints(),
                            "flags" => target.print_flags(),
                            "registers" | "regs" => target.print_regs(),
                            "stack" => target.print_stack(),
                            c => println!("Undefined info command: {c}"),
                        }
                    }
                }
                "next" | "n" | "step" | "s" => {
                    target.step_mode();
                    target.run();
                }
                c => println!("Undefined command: {c}"),
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

macro_rules! from_str_radix_impl {
    ($($int_ty:ty),*) => {
        $(
            impl FromStrRadix for $int_ty {
                fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                    Self::from_str_radix(s, radix)
                }
            }
        )*
    };
}

from_str_radix_impl! {isize, usize, i8, u8, i16, u16, i32, u32, i64, u64}

fn parse_numeric<T>(input: &str) -> Result<T, ParseIntError>
where
    T: FromStrRadix + FromStr<Err = ParseIntError>,
{
    let input = input.replace('_', "");
    // Hex
    if let Some(input) = ["$", "0x", "0X"]
        .iter()
        .find_map(|&pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 16)
    }
    // Octal
    else if let Some(input) = ["&", "0o", "0O"]
        .iter()
        .find_map(|&pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 8)
    }
    // Binary
    else if let Some(input) = ["%", "0b", "0B"]
        .iter()
        .find_map(|&pat| input.strip_prefix(pat))
    {
        T::from_str_radix(input, 2)
    } else {
        input.parse::<T>()
    }
}
