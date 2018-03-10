use cpu::CPU;
use inst::Instruction;

#[derive(Debug)]
pub struct Opcode(pub Instruction, pub AddressingMode);

pub const OPCODES: [Opcode; 256] = [
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::PHP, AddressingMode::Implied),
    Opcode(Instruction::ORA, AddressingMode::Immediate),
    Opcode(Instruction::ASL, AddressingMode::Accumulator),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BPL, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CLC, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::JSR, AddressingMode::Absolute),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BIT, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::PLP, AddressingMode::Implied),
    Opcode(Instruction::AND, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BMI, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::SEC, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::RTI, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::PHA, AddressingMode::Implied),
    Opcode(Instruction::EOR, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::JMP, AddressingMode::Absolute),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BVC, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::RTS, AddressingMode::Implied),
    Opcode(Instruction::ADC, AddressingMode::IndexedIndirect),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::ADC, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::PLA, AddressingMode::Implied),
    Opcode(Instruction::ADC, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::JMP, AddressingMode::Indirect),
    Opcode(Instruction::ADC, AddressingMode::Absolute),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BVS, AddressingMode::Relative),
    Opcode(Instruction::ADC, AddressingMode::IndirectIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::ADC, AddressingMode::ZeroPageAbsoluteX),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::SEI, AddressingMode::Implied),
    Opcode(Instruction::ADC, AddressingMode::AbsoluteY),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::ADC, AddressingMode::AbsoluteX),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::STY, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::STA, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::STX, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::DEY, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::TXA, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::STA, AddressingMode::Absolute),
    Opcode(Instruction::STX, AddressingMode::Absolute),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BCC, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::TYA, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::TXS, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::LDY, AddressingMode::Immediate),
    Opcode(Instruction::LDA, AddressingMode::IndexedIndirect),
    Opcode(Instruction::LDX, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::LDA, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::TAY, AddressingMode::Implied),
    Opcode(Instruction::LDA, AddressingMode::Immediate),
    Opcode(Instruction::TAX, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::LDA, AddressingMode::Absolute),
    Opcode(Instruction::LDX, AddressingMode::Absolute),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BCS, AddressingMode::Relative),
    Opcode(Instruction::LDA, AddressingMode::IndirectIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::LDA, AddressingMode::ZeroPageAbsoluteX),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CLV, AddressingMode::Implied),
    Opcode(Instruction::LDA, AddressingMode::AbsoluteY),
    Opcode(Instruction::TSX, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::LDA, AddressingMode::AbsoluteX),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CPY, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CMP, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::INY, AddressingMode::Implied),
    Opcode(Instruction::CMP, AddressingMode::Immediate),
    Opcode(Instruction::DEX, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BNE, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CLD, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::CPX, AddressingMode::Immediate),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::INC, AddressingMode::ZeroPageIndexed),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::INX, AddressingMode::Implied),
    Opcode(Instruction::SBC, AddressingMode::Immediate),
    Opcode(Instruction::NOP, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::BEQ, AddressingMode::Relative),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::SED, AddressingMode::Implied),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
    Opcode(Instruction::None, AddressingMode::None),
];

#[derive(Debug)]
pub enum AddressingMode {
    None,
    Immediate,
    ZeroPageAbsolute,
    Absolute,
    Implied,
    Accumulator,
    AbsoluteX,
    AbsoluteY,
    IndexedY,
    ZeroPageIndexed,
    ZeroPageAbsoluteX,
    ZeroPageAbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
}

impl AddressingMode {
    pub fn n_bytes(&self) -> Result<usize, ()> {
        match *self {
              AddressingMode::Implied
            | AddressingMode::Accumulator => Ok(1),

              AddressingMode::Immediate
            | AddressingMode::ZeroPageAbsolute
            | AddressingMode::ZeroPageIndexed
            | AddressingMode::Relative
            | AddressingMode::ZeroPageAbsoluteX
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => Ok(2),

              AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::Indirect => Ok(3),

            _ => Err(()),
        }
    }

    pub fn get_bytes(&self, cpu: &CPU) -> Vec<u8> {
        let n_bytes = self.n_bytes().unwrap() as u16;
        (0 .. n_bytes).map(|n| cpu.mem.read(cpu.pc + n).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn get_data(&self, cpu: &CPU, pc: u16) -> Result<(u16, u8), ()> {
        match *self {
            AddressingMode::Immediate => {
                let addr = pc + 1;
                let val = cpu.mem.read(addr)
                    .expect("Immediate val");
                Ok((addr, val))
            },
            AddressingMode::ZeroPageAbsolute => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsolute arg") as u16;
                let addr = (0x00 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Absolute addr");
                Ok((addr, val))
            },
            AddressingMode::Absolute => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Absolute arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Absolute arg 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Absolute addr");
                Ok((addr, val))
            },
            AddressingMode::Implied => Ok((0, 0)),
            AddressingMode::Accumulator => Ok((0, cpu.a)),
            AddressingMode::ZeroPageIndexed => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageIndexed arg") as u16;
                let addr = (0x00 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageIndexed addr");
                Ok((addr, val))
            },
            AddressingMode::Relative => {
                let offset = cpu.mem.read(pc + 1)
                    .expect("Relative arg") as u16;

                let is_neg = (offset as i16) < 0;
                if is_neg {
                    panic!("negatory");
                }

                // TODO negative offset?

                // NOTE This has to be based off the current program counter,
                // _after_ it has been advanced, but before the instruction is
                // being executed. I don't know why though?
                Ok((cpu.pc + offset, 0))
            },
            AddressingMode::AbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("AbsoluteX arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("AbsoluteX arg 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("AbsoluteX addr");
                Ok((0, val + cpu.x))
            },
            AddressingMode::AbsoluteY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("AbsoluteY arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("AbsoluteY arg 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("AbsoluteY addr");
                Ok((0, val + cpu.y))
            },
            AddressingMode::Indirect => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("Indirect arg 1") as u16;
                let hi = cpu.mem.read(pc + 2)
                    .expect("Indirect arg 2") as u16;
                let addr = (hi << 8) | lo;

                let lo = cpu.mem.read(addr)
                    .expect("Indirect addr 1") as u16;
                let hi = cpu.mem.read(addr + 1)
                    .expect("Indirect addr 2") as u16;
                let addr = (hi << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("Indirect addr val");

                Ok((addr, val))
            }
            AddressingMode::ZeroPageAbsoluteX => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteX arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteX addr");
                Ok((0, val + cpu.x))
            },
            AddressingMode::ZeroPageAbsoluteY => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("ZeroPageAbsoluteY arg 1") as u16;
                let addr = (0 << 8) | lo;
                let val = cpu.mem.read(addr)
                    .expect("ZeroPageAbsoluteY addr");
                Ok((0, val + cpu.y))
            },
            AddressingMode::IndexedIndirect => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("IndexedIndirect arg 1");
                let addr = lo.wrapping_add(cpu.x) as u16;
                let val = cpu.mem.read(addr)
                    .expect("IndexedIndirect val");
                Ok((addr, val))
            },
            AddressingMode::IndirectIndexed => {
                let lo = cpu.mem.read(pc + 1)
                    .expect("IndirectIndexed arg 1") as u16;
                let val = cpu.mem.read(lo)
                    .expect("IndirectIndexed val");
                Ok((lo, val + cpu.y))
            },
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr_mode_immediate() {
        let _cpu = CPU::new_nes_cpu();
        // write ROM data
    }
}
