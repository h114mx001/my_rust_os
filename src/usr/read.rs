use crate::api::console;
use crate::api::console::Style;
use crate::api::fs;
use crate::api::process::ExitCode;
use crate::api::syscall;
use crate::{api, sys, usr};
use alloc::format;
use core::convert::TryInto;

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    if args.len() != 2 {
        help();
        return Err(ExitCode::UsageError);
    }
    if args[1] == "-h" || args[1] == "--help" {
        help();
        return Ok(());
    }
    let mut path = args[1];

    // The commands `read /usr/alice/` and `read /usr/alice` are equivalent,
    // but `read /` should not be modified.
    if path.len() > 1 {
        path = path.trim_end_matches('/');
    }

    // TODO: Create device drivers for `/net` hardcoded commands
    if let Some(info) = syscall::info(path) {
        if info.is_file() {
            if let Ok(contents) = api::fs::read_to_string(path) {
                print!("{}", contents);
                Ok(())
            } else {
                error!("Could not read '{}'", path);
                Err(ExitCode::Failure)
            }
        } else if info.is_dir() {
            usr::list::main(args)
        } else if info.is_device() {
            // TODO: Improve device file usage
            let is_char_device = info.size() == 4;
            let is_float_device = info.size() == 8;
            let is_eof_device = info.size() > 8;
            loop {
                if sys::console::end_of_text() || sys::console::end_of_transmission() {
                    println!();
                    return Ok(());
                }
                if let Ok(bytes) = fs::read_to_bytes(path) {
                    if is_char_device && bytes.len() == 1 {
                        match bytes[0] as char {
                            console::ETX_KEY => {
                                println!("^C");
                                return Ok(());
                            }
                            console::EOT_KEY => {
                                println!("^D");
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    if is_float_device && bytes.len() == 8 {
                        println!("{:.6}", f64::from_be_bytes(bytes[0..8].try_into().unwrap()));
                        return Ok(());
                    }
                    for b in bytes {
                        print!("{}", b as char);
                    }
                    if is_eof_device {
                        println!();
                        return Ok(());
                    }
                } else {
                    error!("Could not read '{}'", path);
                    return Err(ExitCode::Failure);
                }
            }
        } else {
            error!("Could not read type of '{}'", path);
            Err(ExitCode::Failure)
        }
    } else {
        error!("Could not find file '{}'", path);
        Err(ExitCode::Failure)
    }
}

fn help() {
    let csi_option = Style::color("LightCyan");
    let csi_title = Style::color("Yellow");
    let csi_reset = Style::reset();
    println!(
        "{}Usage:{} read {}<path>{}",
        csi_title, csi_reset, csi_option, csi_reset
    );
    println!();
    println!("{}Paths:{}", csi_title, csi_reset);
    println!("  {0}<dir>/{1}     Read directory", csi_option, csi_reset);
    println!("  {0}<file>{1}     Read file", csi_option, csi_reset);
}
