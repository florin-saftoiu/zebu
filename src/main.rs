use piston_window::*;
use std::num::Wrapping;

mod cpu;
mod machine;
use cpu::Z80CPU;
use machine::Z80Machine;

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
                machine.print_state();
                println!("T: {}", t_cycles);
            } else if key == Key::Space {
                machine.clock();
                t_cycles += Wrapping(1);
                machine.print_state();
                println!("T: {}", t_cycles);
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