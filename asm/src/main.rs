use std::{fs, process::exit, env};
use colored::Colorize;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filepath = args[1].clone();

    let mut bytes: Vec<u8> = Vec::new();

    let mut assets: Vec<String> = Vec::new();

    let mut raw_file = String::new();
    let lines;

    if args.len() > 3 && args[3] == "p" {
        if args.len() > 4 {raw_file.push_str(&format!("jmp {}\n", args[4]))}

        let files = fs::read_dir(filepath.clone()).unwrap();

        for file in files {
            let path = file.unwrap();
            let mut file = fs::read_to_string(path.path().clone()).unwrap().replace("\r", "");
            
            if path.file_name().into_string().unwrap().ends_with(".h") {
                for num in file.replace("\n", " ").split(" ") {
                    
                    let result: u8;
                    if num.starts_with("0x") {
                        if num.len() < 3 {error(path.path().into_os_string().into_string().unwrap(), 0, &vec![String::from("header")], "invalid number")}
                        result = u8::from_str_radix(&num.replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !num.parse::<u8>().is_ok() {error(path.path().into_os_string().into_string().unwrap(), 0, &vec![String::from("header")], "invalid number")}
                        result = num.parse::<u8>().unwrap();
                    }

                    bytes.push(result);
                }
                file = String::new();
            }
            else if path.file_name().into_string().unwrap().ends_with(".ast") {
                let asts: Vec<String> = file.replace("\r", "").split("\n").map(|s| s.replace("\\n", "\n").to_string()).collect();
                for s in asts {
                    assets.push(s);
                }

                file = String::new();
            }

            let mut lines: Vec<String> = file.split("\n").map(|s| s.to_string()).collect();
            let mut i = 0;
            while i < lines.len() {                
                if lines[i].contains(".") && !lines[i].contains("::") {
                    lines[i] = lines[i].replace(".", &format!(".{}::", path.file_name().into_string().unwrap().replace(".asm", "")));
                }

                i += 1;
            }

            file = lines.join("\n");

            raw_file.push_str(&format!("\nFILE_START: {}\n", path.path().into_os_string().into_string().unwrap()));
            raw_file.push_str(&file);
        }
    }
    else {
        raw_file = fs::read_to_string(filepath.clone()).unwrap().replace("\r", "");
    }

    raw_file = raw_file.replace(";call", ";cal").replace("call", "remove_line: 5\nmov pc br\nldi bx 16\nadd bx\npsh br\njmp")
                        .replace("", "");

    lines = raw_file.split("\n");

    //println!("{}", raw_file);

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
        ("ptr", 0x0D),
    ]);

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
            "ldd" => {label_bytes += 7}
            "std" => {label_bytes += 7}
            "ldr" => {label_bytes += 4}
            "sdr" => {label_bytes += 4}
            "sbd" => {label_bytes += 7}
            "sbdr" => {label_bytes += 4}
            "ldb" => {label_bytes += 7}
            "ldbr" => {label_bytes += 4}
            "jcid" => {label_bytes += 6}
            "jnci" => {label_bytes += 6}
            "ldcd" => {label_bytes += 7}
            "stcd" => {label_bytes += 7}
            "lcdr" => {label_bytes += 4}
            "scdr" => {label_bytes += 4}
            "sbcd" => {label_bytes += 7}
            "sbcdr" => {label_bytes += 4}
            "lcdb" => {label_bytes += 7}
            "lcdbr" => {label_bytes += 4}
            "ptrm" => {label_bytes += 9}

            "ast" => {label_bytes += 20}

            _ => {}
        }
    }

    let mut ast_req: Vec<(usize, usize)> = Vec::new();

    let mut name = String::new();

    let mut line_number = 1;
    for line in lines {
        let line = line.trim().to_lowercase();
        let cmd: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        filepath = name.clone();

        'work: {
            if cmd[0].starts_with(".") || cmd[0].starts_with(";") {break 'work}

            match cmd[0].as_str() {
                "nop" => {bytes.push(0x00);}
                "hlt" => {bytes.push(0x01);}
                "lod" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x02);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sto" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x03);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ldi" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x04);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);

                    let n: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
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
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x05);

                    if !regs.contains_key(cmd[1].as_str()) || !regs.contains_key(cmd[2].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push((regs[cmd[2].as_str()] << 4) | regs[cmd[1].as_str()]); 
                }
                "add" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
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

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "sub" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
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

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register combination")}
                    }
                }
                "mul" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x08);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "div" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x09);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "jmp" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0A);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "je" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0B);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jne" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0C);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jl" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0D);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jle" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0E);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jg" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x0F);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jge" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x10);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "cmp" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x11);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "and" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x12);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "or" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x13);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "xor" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x14);
                    
                    match cmd[1].as_str() {
                        "ar" => {bytes.push(0x00)}
                        "br" => {bytes.push(0x01)}
                        "cr" => {bytes.push(0x02)}
                        "dr" => {bytes.push(0x03)}

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "not" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
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

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }
                }
                "shl" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
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

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }

                    bytes.push(n);
                }
                "shr" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
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

                        _ => {error(filepath.clone(), line_number, &cmd, "invalid register option")}
                    }

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }

                    bytes.push(n);
                }
                "psh" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x18);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "pop" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x19);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "rnd" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1A);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lor" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1B);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "str" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1C);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ubs" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1D);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sbr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1E);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n); 

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "inc" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x1F);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "dec" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x20);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "wit" => {bytes.push(0x21);}
                "ubl" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x22);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lbr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x23);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ldd" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x24);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "std" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x25);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ldr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x26);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sdr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x27);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sbd" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x28);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sbdr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x29);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n); 

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ldb" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2A);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ldbr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2B);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }         
                "jcid" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2C);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }
                "jnci" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2D);

                    if !labels.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown label")}
                    let addr = labels[cmd[1].as_str()];

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);
                }                
                "ldcd" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2E);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "stcd" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x2F);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lcdr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x30);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "scdr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x31);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sbcd" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x32);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "sbcdr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x33);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n); 

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lcdb" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x34);

                    let addr: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        addr = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((addr >> 32) as u8);
                    bytes.push((addr >> 24) as u8);
                    bytes.push((addr >> 16) as u8);
                    bytes.push((addr >> 8) as u8);
                    bytes.push(addr as u8);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "lcdbr" => {
                    if cmd.len() < 4 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x35);

                    if !regs.contains_key(cmd[3].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[3].as_str()]);

                    let n: u8;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u8::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u8>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u8>().unwrap();
                    }
                    bytes.push(n);

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}
                    bytes.push(regs[cmd[1].as_str()]);
                }
                "ptrm" => {
                    if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x36);

                    let n: u64;
                    if cmd[1].starts_with("0x") {
                        if cmd[1].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u64::from_str_radix(&cmd[1].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[1].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[1].parse::<u64>().unwrap();
                    }

                    bytes.push((n >> 24) as u8);
                    bytes.push((n >> 16) as u8);
                    bytes.push((n >> 8) as u8);
                    bytes.push(n as u8);

                    let n: u64;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = u64::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<u64>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<u64>().unwrap();
                    }

                    bytes.push((n >> 24) as u8);
                    bytes.push((n >> 16) as u8);
                    bytes.push((n >> 8) as u8);
                    bytes.push(n as u8);
                }


                "dir" => {
                    if cmd.len() < 2 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    if cmd[1].len() < 3 || !u8::from_str_radix(&cmd[1][2..], 16).is_ok() {error(filepath.clone(), line_number, &cmd, "invalid 8-bit hexadecimal number")}
                    bytes.push(u8::from_str_radix(&cmd[1][2..], 16).unwrap())
                }
                "ast" => {
                    bytes.push(0x18); // psh ax
                    bytes.push(0x00); 
                    bytes.push(0x18); // psh ay
                    bytes.push(0x04);

                   if cmd.len() < 3 {error(filepath.clone(), line_number, &cmd, "missing argument")}
                    bytes.push(0x04);
                    
                    if (cmd[1] == "cx") || (cmd[1] == "cy") || (cmd[1] == "dx") || (cmd[1] == "dy") {error(filepath.clone(), line_number, &cmd, "32 bit register only instruction")}

                    if !regs.contains_key(cmd[1].as_str()) {error(filepath.clone(), line_number, &cmd, "unknown register")}

                    bytes.push(0x00);

                    let n: usize;
                    if cmd[2].starts_with("0x") {
                        if cmd[2].len() < 3 {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = usize::from_str_radix(&cmd[2].replace("_", "")[2..], 16).unwrap();
                    } 
                    else {
                        if !cmd[2].parse::<usize>().is_ok() {error(filepath.clone(), line_number, &cmd, "invalid number")}
                        n = cmd[2].parse::<usize>().unwrap();
                    }

                    ast_req.push((n, bytes.len()));
                    
                    bytes.push(0x00);
                    bytes.push(0x00); 
                    bytes.push(0x00);
                    bytes.push(0x00);

                    bytes.push(0x05); // mov ptr ay
                    bytes.push(0x4D);

                    bytes.push(0x06); // add ar
                    bytes.push(0x00);

                    bytes.push(0x19); // pop ax
                    bytes.push(0x00); 
                    bytes.push(0x19); // pop ay
                    bytes.push(0x04);

                    bytes.push(0x05); // mov ar reg
                    bytes.push(0x08 | regs[cmd[1].as_str()] << 4);
                }
                
                
                "file_start:" => {
                    name = cmd[1].clone();
                    line_number = 0;
                }
                "remove_line:" => {
                    line_number -= cmd[1].parse::<usize>().unwrap();
                }

                "" => {}
                _ => {error(filepath.clone(), line_number, &cmd, "unexpected command")}
            }
        }
        line_number += 1;
    }

    let mut asset_addresses: Vec<usize> = Vec::new();

    for asset in assets.clone() {
        asset_addresses.push(bytes.len());
        
        for byte in asset.as_bytes() {
            bytes.push(*byte);
        }
        bytes.push(0x00);
    }

    for req in ast_req {
        bytes[req.1] = (asset_addresses[req.0] >> 24) as u8;
        bytes[req.1+1] = (asset_addresses[req.0] >> 16) as u8;
        bytes[req.1+2] = (asset_addresses[req.0] >> 8) as u8;
        bytes[req.1+3] = asset_addresses[req.0] as u8;
    }

    //println!("{:?}", asset_addresses);

    //println!("{:0x?}", bytes);
    
    fs::write(&args[2], &bytes).unwrap();

    println!("\n{} {} {}\n", "Successfully compiled".bright_green().bold(), format!("{} bytes", bytes.len()).bright_yellow(), "".bright_green().bold());
}

fn error(filepath: String, line_number: usize, cmd: &Vec<String>, error: &str) {
    println!("\n{}: {}: {} \"{}\"\n", "Error".bright_red().bold(), format!("{} at", error).bright_yellow(), format!("{}:{}", filepath, line_number).bright_blue().underline(), cmd[0].to_uppercase().bold().bright_red());
    exit(0);
}
