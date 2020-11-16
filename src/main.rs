use std::io;
use std::io::prelude::*;
use std::num::Wrapping;
use std::fmt::Write;
use std::fs::File;

use piston_window::*;

mod cpu;
mod machine;
use cpu::{Z80CPU, Z80CPUState};
use machine::Z80Machine;

const WINDOW_PADDING: f64 = 8.0;
const WINDOW_FONTSIZE: f64 = 16.0;
const SCREEN_WIDTH: f64 = 256.0;
const SCREEN_HEIGHT: f64 = 192.0;
const SCREEN_SCALE: f64 = 2.0;

const BACKGROUND: [f32; 4] = [0.0, 0.478, 0.8, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

fn print_cpu_state(state: Z80CPUState) {
    println!("  AF: {:02X}{:02X} AF': {:02X}{:02X}", state.a, state.f, state.a_alt, state.f_alt);
    println!("  BC: {:02X}{:02X} BC': {:02X}{:02X}", state.b, state.c, state.b_alt, state.c_alt);
    println!("  DE: {:02X}{:02X} DE': {:02X}{:02X}", state.d, state.e, state.d_alt, state.e_alt);
    println!("  HL: {:02X}{:02X} HL': {:02X}{:02X}", state.h, state.l, state.h_alt, state.l_alt);
    println!("   I: {:02X}     R: {:02X}", state.i, state.r);
    println!("  IX: {:04X}", state.ix);
    println!("  IY: {:04X}", state.iy);
    println!("  SP: {:04X}", state.sp);
    println!("  PC: {:04X}", state.pc);
}

fn print_ram_slice_state(ram_slice: &[u8], offset: u16) {
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                println!();
            }
            print!("{:04X}: ", offset + nb_bytes);
        }
        print!("{:02X} ", byte);
        nb_bytes += 1;
    }
    println!();
}

fn print_next_cpu_instructions(instructions: Vec<String>) {
    for i in 0..instructions.len() {
        if i == 0 {
            print!(">");
        } else {
            print!(" ")
        }
        println!("{}", instructions[i]);
    }
}

fn draw_cpu_state(state: Z80CPUState, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let lines = format!(
        "AF: {:02X}{:02X} AF': {:02X}{:02X}\n\
         BC: {:02X}{:02X} BC': {:02X}{:02X}\n\
         DE: {:02X}{:02X} DE': {:02X}{:02X}\n\
         HL: {:02X}{:02X} HL': {:02X}{:02X}\n\
         \x20I: {:02X}     R: {:02X}\n\
         IX: {:04X}\n\
         IY: {:04X}\n\
         SP: {:04X}\n\
         PC: {:04X}",
        state.a, state.f, state.a_alt, state.f_alt,
        state.b, state.c, state.b_alt, state.c_alt,
        state.d, state.e, state.d_alt, state.e_alt,
        state.h, state.l, state.h_alt, state.l_alt,
        state.i, state.r,
        state.ix,
        state.iy,
        state.sp,
        state.pc
    );
    let mut y = WINDOW_PADDING + WINDOW_FONTSIZE;
    for line in lines.split("\n") {
        text::Text::new_color(WHITE, WINDOW_FONTSIZE as u32).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn draw_ram_slice_state(ram_slice: &[u8], offset: u16, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let mut ram_string = String::new();
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                writeln!(ram_string).unwrap();
            }
            write!(ram_string, "{:04X}: ", offset + nb_bytes).unwrap();
        }
        write!(ram_string, "{:02X} ", byte).unwrap();
        nb_bytes += 1;
    }
    let mut y = WINDOW_PADDING + SCREEN_HEIGHT * SCREEN_SCALE + WINDOW_PADDING + WINDOW_FONTSIZE;
    for line in ram_string.split("\n") {
        text::Text::new_color(WHITE, WINDOW_FONTSIZE as u32).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn draw_next_cpu_instructions(instructions: Vec<String>, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let mut y = WINDOW_PADDING + WINDOW_FONTSIZE;
    for i in 0..instructions.len() {
        text::Text::new_color(if i == 0 { YELLOW } else { WHITE }, WINDOW_FONTSIZE as u32).draw(
            &instructions[i],
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING + 240.0, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn main() -> io::Result<()> {
    println!("Zebu");
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let mut cpu = Z80CPU::new();
    let mut rom_file = File::open(assets.join("hello.bin"))?;
    let mut rom = [0; 16 * 1024];
    rom_file.read(&mut rom)?;
    let mut ram = [0; 48 * 1024];
    let mut machine = Z80Machine::new(&mut cpu, &rom, &mut ram);
    
    let mut window: PistonWindow = WindowSettings::new("Zebu", [1024, 768])
            .resizable(false)
            .exit_on_esc(true)
            .automatic_close(true)
            .build()
            .unwrap();
    let mut glyphs = window.load_font(assets.join("3270Medium.ttf")).unwrap();

    let mut t_cycles = Wrapping(0usize);
    let mut paused = true;
    while let Some(e) = window.next() {
        if paused {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::S {
                    println!("   T: {}", t_cycles);
                    print_cpu_state(machine.get_cpu_state());
                    print_ram_slice_state(machine.get_ram_slice_state(0, 32), 0x4000);
                    print_next_cpu_instructions(machine.get_next_cpu_instructions(3));
                } else if key == Key::F10 {
                    loop {
                        machine.clock();
                        t_cycles += Wrapping(1);
                        if machine.cpu_instruction_complete() {
                            break;
                        }
                    }
                    println!("   T: {}", t_cycles);
                    print_cpu_state(machine.get_cpu_state());
                    print_ram_slice_state(machine.get_ram_slice_state(0, 32), 0x4000);
                    print_next_cpu_instructions(machine.get_next_cpu_instructions(3));
                } else if key == Key::F5 {
                    paused = false;
                }
            }
         } else {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::F6 {
                    paused = true;
                    // finish current instruction before pausing
                    if !machine.cpu_instruction_complete() {
                        loop {
                            machine.clock();
                            t_cycles += Wrapping(1);
                            if machine.cpu_instruction_complete() {
                                break;
                            }
                        }
                    }
                }
            }
            // user may have just paused and finished the current instruction
            if !paused {
                machine.clock();
                t_cycles += Wrapping(1);
            }
        }
        window.draw_2d(&e, |c, g, device| {
            clear(BACKGROUND, g);
            rectangle(BLACK,
                      [WINDOW_PADDING, WINDOW_PADDING, SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE],
                      c.transform, g);
            draw_cpu_state(machine.get_cpu_state(), c, g, &mut glyphs);
            draw_ram_slice_state(machine.get_ram_slice_state(0, 256), 0x4000, c, g, &mut glyphs);
            draw_next_cpu_instructions(machine.get_next_cpu_instructions(10), c, g, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });
        window.set_title(format!("Zebu - T: {}", t_cycles));
    }

    Ok(())
}