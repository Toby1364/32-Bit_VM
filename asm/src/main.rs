use std::{fs, process::exit};
use colored::Colorize;
use std::collections::HashMap;

fn main() {
    let filename = "main.asm";
    let raw_file = fs::read_to_string(filename).unwrap().replace("\r", "");
    let lines = raw_file.split("\n");

    let regs = HashMap::from([
        ("ax", 0x00),
        ("bx", 0x01),
        ("cx", 0x02),
        ("dx", 0x03),

        ("ay", 0x04),
        ("by", 0x05),
        ("cy", 0x06),
        ("dy", 0x07),

        ("ar", 0x08),
        ("br", 0x09),
        ("cr", 0x0A),
        ("dr", 0x0B),
    ]);

    let mut bytes: Vec<u8> = Vec::new();

    let mut line_number = 1;
    for line in lines {
        let line = line.trim().to_lowercase();
        let cmd: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        match cmd[0].as_str() {
            "nop" => {bytes.push(0x00);}
            "hlt" => {bytes.push(0x01);}
            "lod" => {
                if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x02);

                let addr: u64;
                if cmd[2].starts_with("0x") {
                    addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                } 
                else {
                    if !cmd[2].parse::<u64>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                    addr = cmd[2].parse::<u64>().unwrap();
                }

                bytes.push((addr >> 32) as u8);
                bytes.push((addr >> 24) as u8);
                bytes.push((addr >> 16) as u8);
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);

                if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                bytes.push(regs[cmd[1].as_str()]);
            }
            "sto" => {
                if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x03);

                let addr: u64;
                if cmd[2].starts_with("0x") {
                    addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                } 
                else {
                    if !cmd[2].parse::<u64>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                    addr = cmd[2].parse::<u64>().unwrap();
                }

                bytes.push((addr >> 32) as u8);
                bytes.push((addr >> 24) as u8);
                bytes.push((addr >> 16) as u8);
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);

                if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                bytes.push(regs[cmd[1].as_str()]);
            }
            "ldi" => {
                if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x04);

                if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                bytes.push(regs[cmd[1].as_str()]);

                let n: u64;
                if cmd[2].starts_with("0x") {
                    n = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                } 
                else {
                    if !cmd[2].parse::<u64>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                    n = cmd[2].parse::<u64>().unwrap();
                }

                if !((cmd[1] == "cx") || (cmd[1] == "cy") || (cmd[1] == "dx") || (cmd[1] == "dy")) { 
                    bytes.push((n >> 24) as u8);
                    bytes.push((n >> 16) as u8);
                }
                bytes.push((n >> 8) as u8);
                bytes.push(n as u8);
            }
            "mov" => {
                if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x05);

                if !regs.contains_key(cmd[1].as_str()) || !regs.contains_key(cmd[2].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                bytes.push((regs[cmd[2].as_str()] << 4) | regs[cmd[1].as_str()]); 
            }
            "add" => {
                if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x06);

                match cmd[1].as_str() {
                    "ar" => {bytes.push(0x00)}
                    "br" => {bytes.push(0x01)}
                    "cr" => {bytes.push(0x02)}
                    "dr" => {bytes.push(0x03)}

                    "ax" => {bytes.push(0x04)}
                    "bx" => {bytes.push(0x05)}
                    "cx" => {bytes.push(0x06)}
                    "dx" => {bytes.push(0x07)}

                    "ay" => {bytes.push(0x08)}
                    "by" => {bytes.push(0x09)}
                    "cy" => {bytes.push(0x0A)}
                    "dy" => {bytes.push(0x0B)}

                    _ => {error(filename, line_number, &cmd, "invalid register option")}
                }
            }
            "sub" => {
                if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                bytes.push(0x07);

                let mut both = cmd[1].clone();
                both.push_str(cmd[2].as_str());

                match both.as_str() {
                    "ayax" => {bytes.push(0x00)}
                    "bybx" => {bytes.push(0x01)}
                    "cycx" => {bytes.push(0x02)}
                    "dydx" => {bytes.push(0x03)}

                    "arax" => {bytes.push(0x04)}
                    "brbx" => {bytes.push(0x05)}
                    "crcx" => {bytes.push(0x06)}
                    "drdx" => {bytes.push(0x07)}

                    "aray" => {bytes.push(0x08)}
                    "brby" => {bytes.push(0x09)}
                    "crcy" => {bytes.push(0x0A)}
                    "drdy" => {bytes.push(0x0B)}

                    "axar" => {bytes.push(0x0C)}
                    "bxbr" => {bytes.push(0x0D)}
                    "cxcr" => {bytes.push(0x0E)}
                    "dxdr" => {bytes.push(0x0F)}

                    "ayar" => {bytes.push(0x10)}
                    "bybr" => {bytes.push(0x11)}
                    "cycr" => {bytes.push(0x12)}
                    "dydr" => {bytes.push(0x13)}

                    _ => {error(filename, line_number, &cmd, "invalid register combination")}
                }
            }

            "dir" => {
                if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                if cmd[1].len() < 3 || !u8::from_str_radix(&cmd[1][2..], 16).is_ok() {error(filename, line_number, &cmd, "invalid 8-bit hexadecimal number")}
                bytes.push(u8::from_str_radix(&cmd[1][2..], 16).unwrap())
            }

            "" => {}
            _ => {error(filename, line_number, &cmd, "unexpected command")}
        }
        line_number += 1;
    }
    
    fs::write("out.bin", &bytes).unwrap();

    println!("\n{} {} {}\n", "Successfully compiled".bright_green().bold(), format!("{} bytes", bytes.len()).bright_yellow(), "".bright_green().bold());
}

fn error(filename: &str, line_number: usize, cmd: &Vec<String>, error: &str) {
    println!("\n{}: {}: {} \"{}\"\n", "Error".bright_red().bold(), format!("{} at", error).bright_yellow(), format!("{}:{}", filename, line_number).bright_blue().underline(), cmd[0].to_uppercase().bold().bright_red());
    exit(0);
}
