use std::{fs, process::exit, env};
use colored::Colorize;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();

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

        ("pc", 0x0C),
    ]);

    let mut bytes: Vec<u8> = Vec::new();

    let mut labels: HashMap<String, usize> = HashMap::new();

    let mut label_bytes: usize = 0;

    for line in lines.clone() {
        let line = line.trim().to_lowercase();
        let cmd: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        
        if cmd[0].starts_with(".") {
            let _ = labels.insert(cmd[0].clone(), label_bytes);
        }

        match cmd[0].as_str() {
            "nop" => {label_bytes += 1}
            "hlt" => {label_bytes += 1}
            "lod" => {label_bytes += 7}
            "sto" => {label_bytes += 7}
            "ldi" => {label_bytes += 4; if !((cmd[1] == "cx") || (cmd[1] == "cy") || (cmd[1] == "dx") || (cmd[1] == "dy")) {label_bytes += 2}}
            "mov" => {label_bytes += 2}
            "add" => {label_bytes += 2}
            "sub" => {label_bytes += 2}
            "mul" => {label_bytes += 2}
            "div" => {label_bytes += 2}
            "jmp" => {label_bytes += 6}
            "je" => {label_bytes += 6}
            "jne" => {label_bytes += 6}
            "jl" => {label_bytes += 6}
            "jle" => {label_bytes += 6}
            "jg" => {label_bytes += 6}
            "jge" => {label_bytes += 6}
            "cmp" => {label_bytes += 6}
            "and" => {label_bytes += 2}
            "or" => {label_bytes += 2}
            "xor" => {label_bytes += 2}
            "not" => {label_bytes += 2}
            "shl" => {label_bytes += 3}
            "shr" => {label_bytes += 3}
            "psh" => {label_bytes += 2}
            "pop" => {label_bytes += 2}
            "rnd" => {label_bytes += 2}
            "lor" => {label_bytes += 4}
            "str" => {label_bytes += 4}
            "ubs" => {label_bytes += 7}
            "sbr" => {label_bytes += 4}
            "inc" => {label_bytes += 2}
            "dec" => {label_bytes += 2}
            "wit" => {label_bytes += 1}
            "ubl" => {label_bytes += 7}
            "lbr" => {label_bytes += 4}

            _ => {}
        }
    }

    let mut line_number = 1;
    for line in lines {
        let line = line.trim().to_lowercase();
        let cmd: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        'work: {
            if cmd[0].starts_with(".") || cmd[0].starts_with(";") {break 'work}

            match cmd[0].as_str() {
                "nop" => {bytes.push(0x00);}
                "hlt" => {bytes.push(0x01);}
                "lod" => {
                    if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x02);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
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
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
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
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
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
                "mul" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x08);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "div" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x09);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "jmp" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0A);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "je" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0B);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jne" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0C);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jl" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0D);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jle" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0E);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jg" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x0F);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jge" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x10);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "cmp" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x11);

                    if !labels.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "and" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x12);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "or" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x13);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "xor" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x14);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "not" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x15);
                    
                    match cmd[1].as_str() {
                        "ax" => {bytes.push(0x00)}
                        "bx" => {bytes.push(0x01)}
                        "cx" => {bytes.push(0x02)}
                        "dx" => {bytes.push(0x03)}

                        "ay" => {bytes.push(0x04)}
                        "by" => {bytes.push(0x05)}
                        "cy" => {bytes.push(0x06)}
                        "dy" => {bytes.push(0x07)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }
                }
                "shl" => {
                    if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x16);
                    
                    match cmd[1].as_str() {
                        "ax" => {bytes.push(0x00)}
                        "bx" => {bytes.push(0x01)}
                        "cx" => {bytes.push(0x02)}
                        "dx" => {bytes.push(0x03)}

                        "ay" => {bytes.push(0x04)}
                        "by" => {bytes.push(0x05)}
                        "cy" => {bytes.push(0x06)}
                        "dy" => {bytes.push(0x07)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }

                    bytes.push(n);
                }
                "shr" => {
                    if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x17);
                    
                    match cmd[1].as_str() {
                        "ax" => {bytes.push(0x00)}
                        "bx" => {bytes.push(0x01)}
                        "cx" => {bytes.push(0x02)}
                        "dx" => {bytes.push(0x03)}

                        "ay" => {bytes.push(0x04)}
                        "by" => {bytes.push(0x05)}
                        "cy" => {bytes.push(0x06)}
                        "dy" => {bytes.push(0x07)}

                        _ => {error(filename, line_number, &cmd, "invalid register option")}
                    }

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }

                    bytes.push(n);
                }
                "psh" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x18);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "pop" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x19);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "rnd" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1A);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lor" => {
                    if cmd.len() < 4 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1B);

                    if !regs.contains_key(cmd[3].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "str" => {
                    if cmd.len() < 4 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1C);

                    if !regs.contains_key(cmd[3].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ubs" => {
                    if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1D);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
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
                "sbr" => {
                    if cmd.len() < 4 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1E);

                    if !regs.contains_key(cmd[3].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n); 

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "inc" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x1F);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "dec" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x20);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "wit" => {bytes.push(0x21);}
                "ubl" => {
                    if cmd.len() < 3 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x22);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
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
                "lbr" => {
                    if cmd.len() < 4 {error(filename, line_number, &cmd, "missing argument")}
                    bytes.push(0x23);

                    if !regs.contains_key(cmd[3].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filename, line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filename, line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filename, line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }

                "dir" => {
                    if cmd.len() < 2 {error(filename, line_number, &cmd, "missing argument")}
                    if cmd[1].len() < 3 || !u8::from_str_radix(&cmd[1][2..], 16).is_ok() {error(filename, line_number, &cmd, "invalid 8-bit hexadecimal number")}
                    bytes.push(u8::from_str_radix(&cmd[1][2..], 16).unwrap())
                }

                "" => {}
                _ => {error(filename, line_number, &cmd, "unexpected command")}
            }
        }
        line_number += 1;
    }
    
    fs::write(&args[2], &bytes).unwrap();

    println!("\n{} {} {}\n", "Successfully compiled".bright_green().bold(), format!("{} bytes", bytes.len()).bright_yellow(), "".bright_green().bold());
}

fn error(filename: &str, line_number: usize, cmd: &Vec<String>, error: &str) {
    println!("\n{}: {}: {} \"{}\"\n", "Error".bright_red().bold(), format!("{} at", error).bright_yellow(), format!("{}:{}", filename, line_number).bright_blue().underline(), cmd[0].to_uppercase().bold().bright_red());
    exit(0);
}
