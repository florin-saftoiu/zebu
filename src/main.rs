use std::num::Wrapping;
use std::fmt::Write;

use piston_window::*;

mod cpu;
mod machine;
use cpu::{Z80CPU, Z80CPUState};
use machine::Z80Machine;

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

fn print_ram_slice_state(ram_slice : &[u8]) {
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                println!();
            }
            print!("{:04X}: ", nb_bytes);
        }
        print!("{:02X} ", byte);
        nb_bytes += 1;
    }
    println!();
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
    let mut y = 28.0;
    for line in lines.split("\n") {
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(528.0, y),
            g
        ).unwrap();
        y += 20.0;
    }
}

fn draw_ram_slice_state(ram_slice: &[u8], c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let mut ram_string = String::new();
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                writeln!(ram_string).unwrap();
            }
            write!(ram_string, "{:04X}: ", nb_bytes).unwrap();
        }
        write!(ram_string, "{:02X} ", byte).unwrap();
        nb_bytes += 1;
    }
    let mut y = 420.0;
    for line in ram_string.split("\n") {
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(8.0, y),
            g
        ).unwrap();
        y += 20.0;
    }
}

fn main() {
    println!("Zebu");

    let mut cpu = Z80CPU::new();
    let rom = [
        0x3e, 0x2a, // LD A, 42
        0x21, 0x01, 0x00, // LD HL, 1
        0x77  // LD (HL), A
    ];
    let mut ram = [0; 48 * 1024];
    let mut machine = Z80Machine::new(&mut cpu, &rom, &mut ram);
    
    let mut window: PistonWindow = WindowSettings::new("Zebu", [1024, 768])
            .resizable(false)
            .exit_on_esc(true)
            .automatic_close(true)
            .build()
            .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("3270Medium.ttf")).unwrap();

    let mut t_cycles = Wrapping(0usize);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::S {
                println!("   T: {}", t_cycles);
                print_cpu_state(machine.get_cpu_state());
                print_ram_slice_state(machine.get_ram_slice_state(0, 32));
            } else if key == Key::Space {
                machine.clock();
                t_cycles += Wrapping(1);
                println!("   T: {}", t_cycles);
                print_cpu_state(machine.get_cpu_state());
                print_ram_slice_state(machine.get_ram_slice_state(0, 32));
            }
        }
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.478, 0.8, 1.0], g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                      [8.0, 8.0, 512.0, 384.0],
                      c.transform, g);
            draw_cpu_state(machine.get_cpu_state(), c, g, &mut glyphs);
            draw_ram_slice_state(machine.get_ram_slice_state(0, 32), c, g, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });
        window.set_title(format!("Zebu - T: {}", t_cycles));
    }
}