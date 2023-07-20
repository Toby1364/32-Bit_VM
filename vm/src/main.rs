use std::sync::Mutex;
use std::{thread, time};
use macroquad::prelude::*;
use std::fs;
//use rand::Rng;
extern crate rand;

#[derive(Debug)]
struct Registers {
    ax: u32,
    bx: u32,
    cx: u16,
    dx: u16,

    ay: u32,
    by: u32,
    cy: u16,
    dy: u16,

    ar: u32,
    br: u32,
    cr: u32,
    dr: u32,

    ptr: usize,
    stck_ptr: usize,
    pc: usize,
}

impl Registers {
    fn new() -> Self {
        Registers {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,

            ay: 0,
            by: 0,
            cy: 0,
            dy: 0,

            ar: 0,
            br: 0,
            cr: 0,
            dr: 0,

            ptr: 0,
            stck_ptr: 0,
            pc: 0,
        }
    }
}

static RAM: Mutex<[u8; 8 << 20]> = Mutex::new([0; 8 << 20]);

fn window_conf() -> Conf {
    Conf {
        window_title: "32VM".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    /*RAM.lock().unwrap()[0x00_0100] = 0x00;
    RAM.lock().unwrap()[0x00_0101] = 0x00;
    RAM.lock().unwrap()[0x00_0102] = 0x00;
    RAM.lock().unwrap()[0x00_0103] = 0x00;
    RAM.lock().unwrap()[0x00_0104] = 0x04;
    RAM.lock().unwrap()[0x00_0105] = 0x06;
    RAM.lock().unwrap()[0x00_0106] = 0x00;
    RAM.lock().unwrap()[0x00_0107] = 0x01;
    RAM.lock().unwrap()[0x00_0108] = 0x07;
    RAM.lock().unwrap()[0x00_0109] = 0x12;
    RAM.lock().unwrap()[0x00_010A] = 0x0A;
    RAM.lock().unwrap()[0x00_010B] = 0x00;
    RAM.lock().unwrap()[0x00_010C] = 0x00;
    RAM.lock().unwrap()[0x00_010D] = 0x00;
    RAM.lock().unwrap()[0x00_010E] = 0x00;
    RAM.lock().unwrap()[0x00_010F] = 0x04;*/ 

    let program = fs::read("out.bin").unwrap();

    //println!("{:0x?}", program);

    for i in 0..program.len() {
        RAM.lock().unwrap()[0x00_00FF + i] = program[i];
    }

    /*let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < 0x00_0600 {
        RAM.lock().unwrap()[0x7F_FA00 + i] = rng.gen_range(0x0..0x80);
        i += 1;
    }*/

    thread::spawn(move || {
        core(0x00_00FF, 0x70_0000);
    });

    let fonts = [
        load_ttf_font_from_bytes(include_bytes!("../fonts/Perfect_DOS_VGA_437_Win.ttf")).unwrap()
    ];

    while RAM.lock().unwrap()[0x00_0000] != 0x01 {
        {
            let mut tram = RAM.lock().unwrap();
            tram[0x00_0002] = (screen_width() as u16 >> 8) as u8;
            tram[0x00_0003] = screen_width() as u16 as u8;

            tram[0x00_0004] = (screen_height() as u16 >> 8) as u8;
            tram[0x00_0005] = screen_height() as u16 as u8;
        }
        next_frame().await;
        clear_background(BLACK);

        /*let mut x: Vec<u8> = Vec::new();
        for i in 0..55 {
            x.push(RAM.lock().unwrap()[0x0F_0000+i])
        }
        println!("{:?}", x);*/

        if RAM.lock().unwrap()[0x00_0001] == 0x00 { //Basic Terminal
            let mut bytes: [[u8; 70]; 0x16] = [[0x0; 70]; 0x16];

            let mut holder = 0x0;
            let mut i;
            'out: loop {
                i = holder;
                while i < holder+70 {
                    let mut byte = RAM.lock().unwrap()[0x7F_FA00 + i];
                    if byte > 0x7F {byte = b'?'}
                    bytes[holder/70][i-holder] = byte;
                    i += 1;
                    if i == 0x05FF {break 'out;}
                }
                holder += 70;
            }
            
            let mut i = 0;
            while i < bytes.len() {
                let text = std::str::from_utf8(&bytes[i]).expect("Bad UTF-8");

                draw_text_ex(
                    &text,
                    8.0,
                    22.0 * ((i+1) as f32),
                    
                    TextParams {
                        font: fonts[0],
                        font_size: 22,
                        color: Color::new(
                            1.0, 1.0, 1.0, 1.0
                        ),
                        ..Default::default()
                    },
                );

                i += 1;
            }
        }
        else if RAM.lock().unwrap()[0x00_0001] == 0x01 { //Colored Graphics
            let mut i = 0;
            while i < 0x5FFF {
                let tram = RAM.lock().unwrap();
                if tram[0x7F_A000 + i + 11] == 0 {i += 12; continue}
                let x = (((tram[0x7F_A000 + i + 0] as u16) << 8) | (tram[0x7F_A000 + i + 1] as u16)) as f32;
                let y = (((tram[0x7F_A000 + i + 2] as u16) << 8) | (tram[0x7F_A000 + i + 3] as u16)) as f32;

                let w = (((tram[0x7F_A000 + i + 4] as u16) << 8) | (tram[0x7F_A000 + i + 5] as u16)) as f32;
                let h = (((tram[0x7F_A000 + i + 6] as u16) << 8) | (tram[0x7F_A000 + i + 7] as u16)) as f32;

                draw_rectangle(
                    x,
                    y,
                    w,
                    h,
                    Color::new(
                        tram[0x7F_A000 + i + 8] as f32 / 255.,
                        tram[0x7F_A000 + i + 9] as f32 / 255.,
                        tram[0x7F_A000 + i + 10] as f32 / 255.,
                        tram[0x7F_A000 + i + 11] as f32 / 255.,
                    ),
                );
                i += 12;
            }
        }
        else if RAM.lock().unwrap()[0x00_0001] == 0x02 { //Monochrome Graphics

        }
    }
}

fn core(ptr: usize, stck_ptr: usize) {
    let mut reg: Registers = Registers::new();
    reg.ptr = ptr;
    reg.stck_ptr = stck_ptr;

    loop {
        let inst = RAM.lock().unwrap()[reg.ptr + reg.pc];
        reg.pc += 1;

        println!("{:?}", reg);

        //println!("{:#08X}: {:#04X}", reg.pc - 1, inst);

        match inst {
            0x00 => { // NOP
                //println!("NOP");
                thread::sleep(time::Duration::from_millis(0));
            }
            0x01 => { // HLT
                //println!("HLT");
                RAM.lock().unwrap()[0x00_0000] = 0x01;
            }
            0x02 => { // LOD: 5b addr, 1b reg
                //println!("LOD");
                let tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 32) | 
                                ((tram[reg.ptr + reg.pc + 1] as u64) << 24) | 
                                ((tram[reg.ptr + reg.pc + 2] as u64) << 16) | 
                                ((tram[reg.ptr + reg.pc + 3] as u64) << 8) | 
                                ((tram[reg.ptr + reg.pc + 4] as u64))) as usize;
                reg.pc += 5;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let number32 = ((tram[addr] as u32) << 24) | ((tram[addr + 1] as u32) << 16) | ((tram[addr + 2] as u32) << 8) | (tram[addr + 3] as u32);
                let number16 = ((tram[addr] as u16) << 8) | (tram[addr + 1] as u16);

                match register_index {
                    0x00 => {reg.ax = number32;}
                    0x01 => {reg.bx = number32;}
                    0x02 => {reg.cx = number16;}
                    0x03 => {reg.dx = number16;}

                    0x04 => {reg.ay = number32;}
                    0x05 => {reg.by = number32;}
                    0x06 => {reg.cy = number16;}
                    0x07 => {reg.dy = number16;}

                    0x08 => {reg.ar = number32;}
                    0x09 => {reg.br = number32;}
                    0x0A => {reg.cr = number32;}
                    0x0B => {reg.dr = number32;}

                    _ => {}
                }
            }
            0x03 => { // STO: 5b addr, 1b reg
                //println!("STO");
                let mut tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 32) |
                                ((tram[reg.ptr + reg.pc + 1] as u64) << 24) |
                                ((tram[reg.ptr + reg.pc + 2] as u64) << 16) |
                                ((tram[reg.ptr + reg.pc + 3] as u64) << 8) |
                                ((tram[reg.ptr + reg.pc + 4] as u64))) as usize;
                reg.pc += 5;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut num32= None;
                let mut num16 = 0;

                match register_index {
                    0x00 => {num32 = Some(reg.ax);}
                    0x01 => {num32 = Some(reg.bx);}
                    0x02 => {num16 = reg.cx;}
                    0x03 => {num16 = reg.dx;}

                    0x04 => {num32 = Some(reg.ay);}
                    0x05 => {num32 = Some(reg.by);}
                    0x06 => {num16 = reg.cy;}
                    0x07 => {num16 = reg.dy;}

                    0x08 => {num32 = Some(reg.ar);}
                    0x09 => {num32 = Some(reg.br);}
                    0x0A => {num32 = Some(reg.cr);}
                    0x0B => {num32 = Some(reg.dr);}
                    _ => {}
                }

                if num32 != None {
                    let num32 = num32.unwrap();
                    tram[addr] = (num32 >> 24) as u8;
                    tram[addr + 1] = (num32 >> 16) as u8;
                    tram[addr + 2] = (num32 >> 8) as u8;
                    tram[addr + 3] = num32 as u8;
                }
                else {
                    tram[addr] = (num16 >> 8) as u8;
                    tram[addr + 1] = num16 as u8;
                }
            }
            0x04 => { // LDI: 1b reg, 4/2b n
                //println!("LDI");
                let tram = RAM.lock().unwrap();

                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let number32 = ((tram[reg.ptr + reg.pc] as u32) << 24) | ((tram[reg.ptr + reg.pc + 1] as u32) << 16) | ((tram[reg.ptr + reg.pc + 2] as u32) << 8) | (tram[reg.ptr + reg.pc + 3] as u32);
                let number16 = ((tram[reg.ptr + reg.pc] as u16) << 8) | (tram[reg.ptr + reg.pc + 1] as u16);

                match register_index {
                    0x00 => {reg.ax = number32; reg.pc += 4;}
                    0x01 => {reg.bx = number32; reg.pc += 4;}
                    0x02 => {reg.cx = number16; reg.pc += 2;}
                    0x03 => {reg.dx = number16; reg.pc += 2;}

                    0x04 => {reg.ay = number32; reg.pc += 4;}
                    0x05 => {reg.by = number32; reg.pc += 4;}
                    0x06 => {reg.cy = number16; reg.pc += 2;}
                    0x07 => {reg.dy = number16; reg.pc += 2;}

                    0x08 => {reg.ar = number32; reg.pc += 4;}
                    0x09 => {reg.br = number32; reg.pc += 4;}
                    0x0A => {reg.cr = number32; reg.pc += 4;}
                    0x0B => {reg.dr = number32; reg.pc += 4;}

                    _ => {}
                }
            }
            0x05 => { //MOV: 1b reg-reg,
                //println!("MOV");
                let first_reg = (RAM.lock().unwrap()[reg.ptr + reg.pc] << 4) >> 4;
                let second_reg = RAM.lock().unwrap()[reg.ptr + reg.pc] >> 4;
                reg.pc += 1;

                let mut n = 0;

                match first_reg {
                    0x0 => {n = reg.ax;}
                    0x1 => {n = reg.bx;}
                    0x2 => {n = reg.cx as u32;}
                    0x3 => {n = reg.dx as u32;}

                    0x4 => {n = reg.ay;}
                    0x5 => {n = reg.by;}
                    0x6 => {n = reg.cy as u32;}
                    0x7 => {n = reg.dy as u32;}

                    0x8 => {n = reg.ar;}
                    0x9 => {n = reg.br;}
                    0xA => {n = reg.cr;}
                    0xB => {n = reg.dr;}

                    0xC => {n = reg.pc as u32;}

                    _ => {}
                }

                match second_reg {
                    0x0 => {reg.ax = n;}
                    0x1 => {reg.bx = n;}
                    0x2 => {reg.cx = n as u16;}
                    0x3 => {reg.dx = n as u16;}

                    0x4 => {reg.ay = n;}
                    0x5 => {reg.by = n;}
                    0x6 => {reg.cy = n as u16;}
                    0x7 => {reg.dy = n as u16;}

                    0x8 => {reg.ar = n;}
                    0x9 => {reg.br = n;}
                    0xA => {reg.cr = n;}
                    0xB => {reg.dr = n;}

                    0xC => {reg.pc = n as usize;}

                    _ => {}
                }
            }
            0x06 => { // ADD: 1b reg
                //println!("ADD");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax + reg.ay;}
                    0x01 => {reg.br = reg.bx + reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 + reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 + reg.dy as u32;}

                    0x04 => {reg.ar = reg.ax + reg.ar;}
                    0x05 => {reg.br = reg.bx + reg.br;}
                    0x06 => {reg.cr = reg.cx as u32 + reg.cr;}
                    0x07 => {reg.dr = reg.dx as u32 + reg.dr;}

                    0x08 => {reg.ar = reg.ay + reg.ar;}
                    0x09 => {reg.br = reg.by + reg.br;}
                    0x0A => {reg.cr = reg.cy as u32 + reg.cr;}
                    0x0B => {reg.dr = reg.dy as u32 + reg.dr;}
                    _ => {}
                }
            }
            0x07 => { // SUB: 1b reg
                //println!("SUB");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax - reg.ay;}
                    0x01 => {reg.br = reg.bx - reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 - reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 - reg.dy as u32;}

                    0x04 => {reg.ar = reg.ax - reg.ar;}
                    0x05 => {reg.br = reg.bx - reg.br;}
                    0x06 => {reg.cr = reg.cx as u32 - reg.cr;}
                    0x07 => {reg.dr = reg.dx as u32 - reg.dr;}

                    0x08 => {reg.ar = reg.ay - reg.ar;}
                    0x09 => {reg.br = reg.by - reg.br;}
                    0x0A => {reg.cr = reg.cy as u32 - reg.cr;}
                    0x0B => {reg.dr = reg.dy as u32 - reg.dr;}

                    0x0C => {reg.ar = reg.ar - reg.ax;}
                    0x0D => {reg.br = reg.br - reg.bx;}
                    0x0E => {reg.cr = reg.cr - reg.cx as u32;}
                    0x0F => {reg.dr = reg.dr - reg.dx as u32;}

                    0x10 => {reg.ar = reg.ar - reg.ay;}
                    0x11 => {reg.br = reg.br - reg.by;}
                    0x12 => {reg.cr = reg.cr - reg.cy as u32;}
                    0x13 => {reg.dr = reg.dr - reg.dy as u32;}

                    _ => {}
                }
            }
            0x08 => { // MUL: 1b reg
                //println!("MUL");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax * reg.ay;}
                    0x01 => {reg.br = reg.bx * reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 * reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 * reg.dy as u32;}

                    _ => {}
                }
            }
            0x09 => { // DIV: 1b reg
                //println!("DIV");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax / reg.ay;}
                    0x01 => {reg.br = reg.bx / reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 / reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 / reg.dy as u32;}

                    _ => {}
                }
            }
            0x0A => { // JMP: 5b addr
                //println!("JMP");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc = addr;
            }
            0x0B => { // JE: 5b addr
                //println!("JE");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr == 0 {
                    reg.pc = addr;
                }
            }
            0x0C => { // JNE: 5b addr
                //println!("JNE");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr != 0 {
                    reg.pc = addr;
                }
            }
            0x0D => { // JL: 5b addr
                //println!("JL");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr < reg.by {
                    reg.pc = addr;
                }
            }
            0x0E => { // JLE: 5b addr
                //println!("JLE");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr <= reg.by {
                    reg.pc = addr;
                }
            }
            0x0F => { // JG: 5b addr
                //println!("JG");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr > reg.by {
                    reg.pc = addr;
                }
            }
            0x10 => { // JGE: 5b addr
                //println!("JGE");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr >= reg.by {
                    reg.pc = addr;
                }
            }
            0x11 => { // CMP: 5b addr
                //println!("CMP");
                let tram = RAM.lock().unwrap();
                let addr = ((tram[reg.ptr + reg.pc] as usize) << 32) | ((tram[reg.ptr + reg.pc + 1] as usize) << 24) | ((tram[reg.ptr + reg.pc + 2] as usize) << 16) | ((tram[reg.ptr + reg.pc + 3] as usize) << 8) | ((tram[reg.ptr + reg.pc + 4] as usize));
                reg.pc += 5;
                if reg.cr == reg.by {
                    reg.pc = addr;
                }
            }
            0x12 => { // AND: 1b reg
                //println!("AND");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax & reg.ay;}
                    0x01 => {reg.br = reg.bx & reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 & reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 & reg.dy as u32;}

                    _ => {}
                }
            }
            0x13 => { // OR: 1b reg
                //println!("OR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax | reg.ay;}
                    0x01 => {reg.br = reg.bx | reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 | reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 | reg.dy as u32;}

                    _ => {}
                }
            }
            0x14 => { // XOR: 1b reg
                //println!("XOR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax ^ reg.ay;}
                    0x01 => {reg.br = reg.bx ^ reg.by;}
                    0x02 => {reg.cr = reg.cx as u32 ^ reg.cy as u32;}
                    0x03 => {reg.dr = reg.dx as u32 ^ reg.dy as u32;}

                    _ => {}
                }
            }
            0x15 => { // NOT: 1b reg
                //println!("NOT");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = !reg.ax;}
                    0x01 => {reg.br = !reg.bx;}
                    0x02 => {reg.cr = !reg.cx as u32;}
                    0x03 => {reg.dr = !reg.dx as u32;}

                    0x04 => {reg.ar = !reg.ay;}
                    0x05 => {reg.br = !reg.by;}
                    0x06 => {reg.cr = !reg.cy as u32;}
                    0x07 => {reg.dr = !reg.dy as u32;}

                    _ => {}
                }
            }
            0x16 => { // SHL: 1b reg, 1b n
                //println!("SHL");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;
                let n = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax << n;}
                    0x01 => {reg.br = reg.bx << n;}
                    0x02 => {reg.cr = (reg.cx as u32) << n;}
                    0x03 => {reg.dr = (reg.dx as u32) << n;}

                    0x04 => {reg.ar = reg.ay << n;}
                    0x05 => {reg.br = reg.by << n;}
                    0x06 => {reg.cr = (reg.cy as u32) << n;}
                    0x07 => {reg.dr = (reg.dy as u32) << n;}

                    _ => {}
                }
            }
            0x17 => { // SHR: 1b reg, 1b n
                //println!("SHR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;
                let n = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ar = reg.ax >> n;}
                    0x01 => {reg.br = reg.bx >> n;}
                    0x02 => {reg.cr = (reg.cx as u32) >> n;}
                    0x03 => {reg.dr = (reg.dx as u32) >> n;}

                    0x04 => {reg.ar = reg.ay >> n;}
                    0x05 => {reg.br = reg.by >> n;}
                    0x06 => {reg.cr = (reg.cy as u32) >> n;}
                    0x07 => {reg.dr = (reg.dy as u32) >> n;}

                    _ => {}
                }
            }
            0x18 => { // PSH: 1b reg
                //println!("PSH");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut value = 0;

                match register_index {
                    0x00 => {value = reg.ax;}
                    0x01 => {value = reg.bx;}
                    0x02 => {value = reg.cx as u32;}
                    0x03 => {value = reg.dx as u32;}

                    0x04 => {value = reg.ay;}
                    0x05 => {value = reg.by;}
                    0x06 => {value = reg.cy as u32;}
                    0x07 => {value = reg.dy as u32;}

                    0x08 => {value = reg.ar;}
                    0x09 => {value = reg.br;}
                    0x0A => {value = reg.cr as u32;}
                    0x0B => {value = reg.dr as u32;}

                    0x0C => {value = reg.pc as u32;}

                    _ => {}
                }

                RAM.lock().unwrap()[reg.stck_ptr] = (value >> 24) as u8;
                RAM.lock().unwrap()[reg.stck_ptr + 1] = (value >> 16) as u8;
                RAM.lock().unwrap()[reg.stck_ptr + 2] = (value >> 8) as u8;
                RAM.lock().unwrap()[reg.stck_ptr + 3] = value as u8;

                reg.stck_ptr += 4;
            }
            0x19 => { // POP: 1b reg
                //println!("POP");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;
                reg.stck_ptr -= 4;
                
                let tram = RAM.lock().unwrap();
                let value = ((tram[reg.stck_ptr] as u32) << 24) | ((tram[reg.stck_ptr + 1] as u32) << 16) | ((tram[reg.stck_ptr + 2] as u32) << 8) | (tram[reg.stck_ptr + 3] as u32);
            
                match register_index {
                    0x00 => {reg.ax = value;}
                    0x01 => {reg.bx = value;}
                    0x02 => {reg.cx = value as u16;}
                    0x03 => {reg.dx = value as u16;}

                    0x04 => {reg.ay = value;}
                    0x05 => {reg.by = value;}
                    0x06 => {reg.cy = value as u16;}
                    0x07 => {reg.dy = value as u16;}

                    0x08 => {reg.ar = value;}
                    0x09 => {reg.br = value;}
                    0x0A => {reg.cr = value;}
                    0x0B => {reg.dr = value;}

                    0x0C => {reg.pc = value as usize;}

                    _ => {}
                }
            }
            0x1A => { // RND: 1b reg
                //println!("RND");
                let value = rand::random::<u32>();

                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                match register_index {
                    0x00 => {reg.ax = value;}
                    0x01 => {reg.bx = value;}
                    0x02 => {reg.cx = value as u16;}
                    0x03 => {reg.dx = value as u16;}

                    0x04 => {reg.ay = value;}
                    0x05 => {reg.by = value;}
                    0x06 => {reg.cy = value as u16;}
                    0x07 => {reg.dy = value as u16;}

                    0x08 => {reg.ar = value;}
                    0x09 => {reg.br = value;}
                    0x0A => {reg.cr = value;}
                    0x0B => {reg.dr = value;}

                    _ => {}
                }
            }
            0x1B => { // LOR: 1b reg, 1b addr, 1b reg
                //println!("LOR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut value = 0;

                match register_index {
                    0x00 => {value = reg.ax;}
                    0x01 => {value = reg.bx;}
                    0x02 => {value = reg.cx as u32;}
                    0x03 => {value = reg.dx as u32;}

                    0x04 => {value = reg.ay;}
                    0x05 => {value = reg.by;}
                    0x06 => {value = reg.cy as u32;}
                    0x07 => {value = reg.dy as u32;}

                    0x08 => {value = reg.ar;}
                    0x09 => {value = reg.br;}
                    0x0A => {value = reg.cr;}
                    0x0B => {value = reg.dr;}

                    _ => {}
                }

                let tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 16) | (value as u64)) as usize;
                reg.pc += 1;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let number32 = ((tram[addr] as u32) << 24) | ((tram[addr + 1] as u32) << 16) | ((tram[addr + 2] as u32) << 8) | (tram[addr + 3] as u32);
                let number16 = ((tram[addr] as u16) << 8) | (tram[addr + 1] as u16);

                match register_index {
                    0x00 => {reg.ax = number32;}
                    0x01 => {reg.bx = number32;}
                    0x02 => {reg.cx = number16;}
                    0x03 => {reg.dx = number16;}

                    0x04 => {reg.ay = number32;}
                    0x05 => {reg.by = number32;}
                    0x06 => {reg.cy = number16;}
                    0x07 => {reg.dy = number16;}

                    0x08 => {reg.ar = number32;}
                    0x09 => {reg.br = number32;}
                    0x0A => {reg.cr = number32;}
                    0x0B => {reg.dr = number32;}

                    _ => {}
                }
            }
            0x1C => { // STR: 1b reg, 1b addr, 1b reg
                //println!("STR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut value = 0;

                match register_index {
                    0x00 => {value = reg.ax;}
                    0x01 => {value = reg.bx;}
                    0x02 => {value = reg.cx as u32;}
                    0x03 => {value = reg.dx as u32;}

                    0x04 => {value = reg.ay;}
                    0x05 => {value = reg.by;}
                    0x06 => {value = reg.cy as u32;}
                    0x07 => {value = reg.dy as u32;}

                    0x08 => {value = reg.ar;}
                    0x09 => {value = reg.br;}
                    0x0A => {value = reg.cr;}
                    0x0B => {value = reg.dr;}

                    _ => {}
                }

                let mut tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 16) | (value as u64)) as usize;
                reg.pc += 1;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut num32= None;
                let mut num16 = 0;

                match register_index {
                    0x00 => {num32 = Some(reg.ax);}
                    0x01 => {num32 = Some(reg.bx);}
                    0x02 => {num16 = reg.cx;}
                    0x03 => {num16 = reg.dx;}

                    0x04 => {num32 = Some(reg.ay);}
                    0x05 => {num32 = Some(reg.by);}
                    0x06 => {num16 = reg.cy;}
                    0x07 => {num16 = reg.dy;}

                    0x08 => {num32 = Some(reg.ar);}
                    0x09 => {num32 = Some(reg.br);}
                    0x0A => {num32 = Some(reg.cr);}
                    0x0B => {num32 = Some(reg.dr);}
                    _ => {}
                }

                if num32 != None {
                    let num32 = num32.unwrap();
                    tram[addr] = (num32 >> 24) as u8;
                    tram[addr + 1] = (num32 >> 16) as u8;
                    tram[addr + 2] = (num32 >> 8) as u8;
                    tram[addr + 3] = num32 as u8;
                }
                else {
                    tram[addr] = (num16 >> 8) as u8;
                    tram[addr + 1] = num16 as u8;
                }
            }
            0x1D => { // UBS: 5b addr, 1b reg
                let mut tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 32) |
                                ((tram[reg.ptr + reg.pc + 1] as u64) << 24) |
                                ((tram[reg.ptr + reg.pc + 2] as u64) << 16) |
                                ((tram[reg.ptr + reg.pc + 3] as u64) << 8) |
                                ((tram[reg.ptr + reg.pc + 4] as u64))) as usize;
                reg.pc += 5;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut num: u8 = 0;

                match register_index {
                    0x00 => {num = reg.ax as u8;}
                    0x01 => {num = reg.bx as u8;}
                    0x02 => {num = reg.cx as u8;}
                    0x03 => {num = reg.dx as u8;}

                    0x04 => {num = reg.ay as u8;}
                    0x05 => {num = reg.by as u8;}
                    0x06 => {num = reg.cy as u8;}
                    0x07 => {num = reg.dy as u8;}

                    0x08 => {num = reg.ar as u8;}
                    0x09 => {num = reg.br as u8;}
                    0x0A => {num = reg.cr as u8;}
                    0x0B => {num = reg.dr as u8;}
                    _ => {}
                }
                tram[addr] = num;
            }
            0x1E => { // SBR: 1b reg, 1b addr, 1b reg
                //println!("SBR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut value = 0;

                match register_index {
                    0x00 => {value = reg.ax;}
                    0x01 => {value = reg.bx;}
                    0x02 => {value = reg.cx as u32;}
                    0x03 => {value = reg.dx as u32;}

                    0x04 => {value = reg.ay;}
                    0x05 => {value = reg.by;}
                    0x06 => {value = reg.cy as u32;}
                    0x07 => {value = reg.dy as u32;}

                    0x08 => {value = reg.ar;}
                    0x09 => {value = reg.br;}
                    0x0A => {value = reg.cr;}
                    0x0B => {value = reg.dr;}

                    _ => {}
                }

                let mut tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 16) | (value as u64)) as usize;
                reg.pc += 1;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut num: u8 = 0;

                match register_index {
                    0x00 => {num = reg.ax as u8;}
                    0x01 => {num = reg.bx as u8;}
                    0x02 => {num = reg.cx as u8;}
                    0x03 => {num = reg.dx as u8;}

                    0x04 => {num = reg.ay as u8;}
                    0x05 => {num = reg.by as u8;}
                    0x06 => {num = reg.cy as u8;}
                    0x07 => {num = reg.dy as u8;}

                    0x08 => {num = reg.ar as u8;}
                    0x09 => {num = reg.br as u8;}
                    0x0A => {num = reg.cr as u8;}
                    0x0B => {num = reg.dr as u8;}
                    _ => {}
                }
                tram[addr] = num;
            }
            0x1F => { // INC: 1b reg
                //println!("INC");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;


                match register_index {
                    0x00 => {reg.ax += 1;}
                    0x01 => {reg.bx += 1;}
                    0x02 => {reg.cx += 1;}
                    0x03 => {reg.dx += 1;}

                    0x04 => {reg.ay += 1;}
                    0x05 => {reg.by += 1;}
                    0x06 => {reg.cy += 1;}
                    0x07 => {reg.dy += 1;}

                    0x08 => {reg.ar += 1;}
                    0x09 => {reg.br += 1;}
                    0x0A => {reg.cr += 1;}
                    0x0B => {reg.dr += 1;}

                    _ => {}
                }
            }
            0x20 => { // DEC: 1b reg
                //println!("DEC");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;


                match register_index {
                    0x00 => {reg.ax -= 1;}
                    0x01 => {reg.bx -= 1;}
                    0x02 => {reg.cx -= 1;}
                    0x03 => {reg.dx -= 1;}

                    0x04 => {reg.ay -= 1;}
                    0x05 => {reg.by -= 1;}
                    0x06 => {reg.cy -= 1;}
                    0x07 => {reg.dy -= 1;}

                    0x08 => {reg.ar -= 1;}
                    0x09 => {reg.br -= 1;}
                    0x0A => {reg.cr -= 1;}
                    0x0B => {reg.dr -= 1;}

                    _ => {}
                }
            }
            0x21 => { // WIT
                //println!("WIT");
                thread::sleep(time::Duration::from_millis(1));
            }
            0x22 => { // UBL: 5b addr, 1b reg
                //println!("UBL");
                let tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 32) | 
                                ((tram[reg.ptr + reg.pc + 1] as u64) << 24) | 
                                ((tram[reg.ptr + reg.pc + 2] as u64) << 16) | 
                                ((tram[reg.ptr + reg.pc + 3] as u64) << 8) | 
                                ((tram[reg.ptr + reg.pc + 4] as u64))) as usize;
                reg.pc += 5;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let number = tram[addr] as u32;

                match register_index {
                    0x00 => {reg.ax = number;}
                    0x01 => {reg.bx = number;}
                    0x02 => {reg.cx = number as u16;}
                    0x03 => {reg.dx = number as u16;}

                    0x04 => {reg.ay = number;}
                    0x05 => {reg.by = number;}
                    0x06 => {reg.cy = number as u16;}
                    0x07 => {reg.dy = number as u16;}

                    0x08 => {reg.ar = number;}
                    0x09 => {reg.br = number;}
                    0x0A => {reg.cr = number;}
                    0x0B => {reg.dr = number;}

                    _ => {}
                }
            }
            0x23 => { // LBR: 1b reg, 1b addr, 1b reg
                //println!("LBR");
                let register_index = RAM.lock().unwrap()[reg.ptr + reg.pc];
                reg.pc += 1;

                let mut value = 0;

                match register_index {
                    0x00 => {value = reg.ax;}
                    0x01 => {value = reg.bx;}
                    0x02 => {value = reg.cx as u32;}
                    0x03 => {value = reg.dx as u32;}

                    0x04 => {value = reg.ay;}
                    0x05 => {value = reg.by;}
                    0x06 => {value = reg.cy as u32;}
                    0x07 => {value = reg.dy as u32;}

                    0x08 => {value = reg.ar;}
                    0x09 => {value = reg.br;}
                    0x0A => {value = reg.cr;}
                    0x0B => {value = reg.dr;}

                    _ => {}
                }

                let tram = RAM.lock().unwrap();
                let addr: usize = (((tram[reg.ptr + reg.pc] as u64) << 16) | (value as u64)) as usize;
                reg.pc += 1;
                let register_index = tram[reg.ptr + reg.pc];
                reg.pc += 1;

                let number = tram[addr] as u32;

                match register_index {
                    0x00 => {reg.ax = number;}
                    0x01 => {reg.bx = number;}
                    0x02 => {reg.cx = number as u16;}
                    0x03 => {reg.dx = number as u16;}

                    0x04 => {reg.ay = number;}
                    0x05 => {reg.by = number;}
                    0x06 => {reg.cy = number as u16;}
                    0x07 => {reg.dy = number as u16;}

                    0x08 => {reg.ar = number;}
                    0x09 => {reg.br = number;}
                    0x0A => {reg.cr = number;}
                    0x0B => {reg.dr = number;}

                    _ => {}
                }
            }

            _ => {}
        }
    }
}
