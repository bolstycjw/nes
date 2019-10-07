use std::env;
use std::fs;
use std::fs::File;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

use crate::controller::Controller;
use crate::cpu::CPU;
use crate::mem::{Memory, NESMemory};
use crate::ppu::PPU;
use crate::ines::CartridgeError;
use crate::ines;
use crate::serde::Storeable;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

lazy_static!{
    pub static ref NES_CPU_DEBUG: bool = match env::var("NES_CPU_DEBUG") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };

    pub static ref NES_CPU_NESTEST: bool = match env::var("NES_CPU_NESTEST") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };

    pub static ref NES_PPU_DEBUG: bool = match env::var("NES_PPU_DEBUG") {
        Ok(val) => val != "" && val != "0",
        Err(_)  => false,
    };
}

const NES_FPS: f64 = 60.0;
const FRAME_DURATION: Duration = Duration::from_millis(((1.0 / NES_FPS) * 1000.0) as u64);

pub struct Console {
    sdl_ctx:   Sdl,
    canvas:    Canvas<Window>,
    cpu:       CPU,
    save_path: String,
}

impl Console {
    pub fn new_nes_console() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut width = 256 * 3;
        let height = 240 * 3;

        if *NES_PPU_DEBUG {
            // Make room for the two pattern tables, side by side
            width += 2 * 144 + 20;
        }

        let window = video_subsystem.window("nes", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(0, 0, width, height)).unwrap();
        canvas.present();

        let ppu = PPU::new_nes_ppu();
        let controller = Controller::new_controller();
        let mem = NESMemory::new_nes_mem(ppu, controller);

        Self {
            sdl_ctx:   sdl_context,
            canvas:    canvas,
            cpu:       CPU::new_nes_cpu(mem),
            save_path: String::new(),
        }
    }

    pub fn insert_cartridge(&mut self, filename: &String)
        -> Result<(), CartridgeError>
    {
        let full_path = fs::canonicalize(filename).map_err(CartridgeError::IO)?;

        info!("loading cartridge: {}", full_path.display());

        let path = full_path.file_name().unwrap()
            .to_str().unwrap();
        self.save_path = format!("{:x}.data", md5::compute(path)).into();

        let mut fh = File::open(full_path).map_err(CartridgeError::IO)?;
        ines::load_file_into_memory(&mut fh, &mut self.cpu.mem)?;

        Ok(())
    }

    // Reads a null-terminated string starting at `addr'
    fn read_string(&mut self, addr: u16) -> Result<String, String> {
        let mut addr = addr;

        let mut rv = String::new();

        loop {
            let b = self.cpu.mem.read(addr)?;

            if b == 0 {
                break;
            }

            rv.push(b as char);

            addr += 1;
        }

        Ok(rv)
    }

    // Detects if we're running a instr_test-v5 rom, and if so, it will output
    // the test results.
    fn debug_tests(&mut self) {
        let a = self.cpu.mem.read(0x6001).unwrap();
        let b = self.cpu.mem.read(0x6002).unwrap();
        let c = self.cpu.mem.read(0x6003).unwrap();

        if a == 0xDE && b == 0xB0 && c == 0x61 {
            let result = self.cpu.mem.read(0x6000).unwrap();

            if result <= 0x7F {
                let result_string = self.read_string(0x6004).unwrap();
                println!("{}", result_string);

                println!("Emulator test complete, final status: 0x{:02X}", result);
                process::exit(0);
            }
        }
    }

    fn save(&mut self) {
        let mut fh = File::create(&self.save_path).unwrap();
        self.cpu.save(&mut fh).expect("unable to save CPU state");
        self.cpu.mem.save(&mut fh).expect("unable to save memory state");
        self.cpu.mem.ppu.save(&mut fh).expect("unable to save PPU state");
        println!("saved state to {}", self.save_path);
    }

    fn load(&mut self) {
        if let Ok(mut fh) = File::open(&self.save_path) {
            self.cpu.load(&mut fh).expect("unable to load CPU state");
            self.cpu.mem.load(&mut fh).expect("unable to load memory state");
            self.cpu.mem.ppu.load(&mut fh).expect("unable to save PPU state");
            println!("loaded state from {}", self.save_path);
        }
    }

    pub fn power_up(&mut self) {
        info!("powering up");

        self.cpu.reset();

        let mut event_pump = self.sdl_ctx.event_pump().unwrap();
        let mut fps_start = Instant::now();
        let mut paused = false;

        'running: loop {
            let mut poll_keyboard = false;
            self.debug_tests();

            if paused {
                poll_keyboard = true;
                thread::sleep(Duration::from_millis(200));
            }
            else {

                let cpu_cycles = self.cpu.step();

                let ppu_cycles = cpu_cycles * 3;

                let mut frame_finished = false;
                for _ in 0 .. ppu_cycles {
                    let res = self.cpu.mem.ppu.step(&mut self.canvas);

                    // TODO uglyyyyy
                    if self.cpu.mem.ppu.data.mapper.irq_flag() {
                        self.cpu.trigger_irq();
                    }

                    if res.signal_scanline {
                        // TODO uuuuuuugly
                        self.cpu.mem.ppu.data.mapper.signal_scanline();
                    }

                    if res.trigger_nmi {
                        self.cpu.trigger_nmi();
                    }

                    if res.frame_finished {
                        frame_finished = true;
                    }
                }

                if frame_finished {
                    self.canvas.present();

                    if let Some(delay) = FRAME_DURATION.checked_sub(fps_start.elapsed()) {
                        debug!("sleeping for {}ms", delay.as_millis());
                        thread::sleep(delay);
                    }

                    fps_start = Instant::now();

                    // Polling for events once per loop slows the emulator
                    // right the fuck down, so I've moved to when a frame has
                    // finished instead.
                    poll_keyboard = true;
                }

            }

            if poll_keyboard {
                // I feel like this shouldn't be so damned slow...
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => { break 'running },

                        Event::KeyDown { keycode: Some(key), .. } => {
                            match key {
                                Keycode::W => { self.cpu.mem.controller.up(true) },
                                Keycode::A => { self.cpu.mem.controller.left(true) },
                                Keycode::S => { self.cpu.mem.controller.down(true) },
                                Keycode::D => { self.cpu.mem.controller.right(true) },

                                Keycode::Return => { self.cpu.mem.controller.start(true) },
                                Keycode::Space  => { self.cpu.mem.controller.select(true) },

                                Keycode::N => { self.cpu.mem.controller.a(true) },
                                Keycode::M => { self.cpu.mem.controller.b(true) },

                                Keycode::P => { paused = ! paused },

                                Keycode::F2  => { self.save() },
                                Keycode::F3  => { self.load() },

                                Keycode::F12 => { self.cpu.reset() },

                                _ => {},
                            }
                        },

                        Event::KeyUp { keycode: Some(key), .. } => {
                            match key {
                                Keycode::W => { self.cpu.mem.controller.up(false) },
                                Keycode::A => { self.cpu.mem.controller.left(false) },
                                Keycode::S => { self.cpu.mem.controller.down(false) },
                                Keycode::D => { self.cpu.mem.controller.right(false) },

                                Keycode::Return => { self.cpu.mem.controller.start(false) },
                                Keycode::Space  => { self.cpu.mem.controller.select(false) },

                                Keycode::N => { self.cpu.mem.controller.a(false) },
                                Keycode::M => { self.cpu.mem.controller.b(false) },

                                _ => {},
                            }
                        },

                        _ => {},
                    }
                }

            }
        }

        info!("powering down");
    }
}
