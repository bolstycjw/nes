use crate::cpu::CPU;
use crate::mem::NESMemory;
use crate::ppu::PPU;
use crate::ines::{Cartridge, CartridgeError};

use std::fs::File;

pub struct Console {
    cpu: CPU,
}

impl Console {
    pub fn new_nes_console() -> Console {
        let ppu = PPU::new_nes_ppu();
        let mem = NESMemory::new_nes_mem(ppu);
        Console {
            cpu: CPU::new_nes_cpu(mem),
        }
    }

    pub fn insert_cartridge(&mut self, filename: &str)
        -> Result<(), CartridgeError>
    {
        debug!("loading cartridge: {}", filename);
        let mut fh = File::open(filename).map_err(CartridgeError::IO)?;
        Cartridge::load_file_into_memory(&mut fh, &mut self.cpu.mem)?;
        Ok(())
    }

    pub fn power_up(&mut self) {
        debug!("powering up");

        self.cpu.init();

        loop {
            let cpu_cycles = self.cpu.step();
            let ppu_cycles = cpu_cycles * 3;

            for _ in 1 .. ppu_cycles {
                let res = self.cpu.mem.ppu.step();

                if res.vblank_nmi {
                    self.cpu.nmi()
                }
            }
        }
    }
}
