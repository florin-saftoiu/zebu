use piston_window::*;
use std::num::Wrapping;

mod cpu;
mod machine;
use cpu::{Z80CPU, Z80CPUState};
use machine::Z80Machine;

fn print_cpu_state(state: Z80CPUState) {
    println!("AF: {:02X}{:02X} AF': {:02X}{:02X}", state.a, state.f, state.a_alt, state.f_alt);
    println!("BC: {:02X}{:02X} BC': {:02X}{:02X}", state.b, state.c, state.b_alt, state.c_alt);
    println!("DE: {:02X}{:02X} DE': {:02X}{:02X}", state.d, state.e, state.d_alt, state.e_alt);
    println!("HL: {:02X}{:02X} HL': {:02X}{:02X}", state.h, state.l, state.h_alt, state.l_alt);
    println!("I: {:02X}, R: {:02X}", state.i, state.r);
    println!("IX: {:04X}", state.ix);
    println!("IY: {:04X}", state.iy);
    println!("SP: {:04X}", state.sp);
    println!("PC: {:04X}", state.pc);
}

pub fn print_ram_slice_state(ram_slice: &[u8]) {
    print!(" M:");
    for byte in ram_slice.iter() {
        print!(" {:02X}", byte);
    }
    println!();
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
                print_cpu_state(machine.get_cpu_state());
                print_ram_slice_state(machine.get_ram_slice_state(0, 4));
                println!(" T: {}", t_cycles);
            } else if key == Key::Space {
                machine.clock();
                t_cycles += Wrapping(1);
                print_cpu_state(machine.get_cpu_state());
                print_ram_slice_state(machine.get_ram_slice_state(0, 4));
                println!(" T: {}", t_cycles);
            }
        }
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.478, 0.8, 1.0], g);
            rectangle([0.0, 0.0, 0.0, 1.0],
                      [8.0, 8.0, 512.0, 384.0],
                      c.transform, g);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20).draw(
                "Zebu",
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(528.0, 28.0),
                g
            ).unwrap();

            glyphs.factory.encoder.flush(device);
        });

    }
}