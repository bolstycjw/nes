use crate::addr::AddressingMode;
use crate::cpu::CPU;

#[derive(Debug)]
pub enum Instruction {
    None,
    ADC,
    ANC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ISB,
    JAM,
    JMP,
    JSR,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SEI,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU, addr: u16, addr_mode: &AddressingMode) {
        match *self {
            Instruction::ADC => cpu.adc(addr),
            Instruction::ANC => cpu.anc(addr),
            Instruction::AND => cpu.and(addr),
            Instruction::ASL => cpu.asl(addr, addr_mode),
            Instruction::BCC => cpu.bcc(addr),
            Instruction::BCS => cpu.bcs(addr),
            Instruction::BEQ => cpu.beq(addr),
            Instruction::BIT => cpu.bit(addr),
            Instruction::BMI => cpu.bmi(addr),
            Instruction::BNE => cpu.bne(addr),
            Instruction::BPL => cpu.bpl(addr),
            Instruction::BRK => cpu.brk(),
            Instruction::BVC => cpu.bvc(addr),
            Instruction::BVS => cpu.bvs(addr),
            Instruction::CLC => cpu.clc(),
            Instruction::CLD => cpu.cld(),
            Instruction::CLI => cpu.cli(),
            Instruction::CLV => cpu.clv(),
            Instruction::CMP => cpu.cmp(addr),
            Instruction::CPX => cpu.cpx(addr),
            Instruction::CPY => cpu.cpy(addr),
            Instruction::DCP => cpu.dcp(addr),
            Instruction::DEC => cpu.dec(addr),
            Instruction::DEX => cpu.dex(),
            Instruction::DEY => cpu.dey(),
            Instruction::EOR => cpu.eor(addr),
            Instruction::INC => cpu.inc(addr),
            Instruction::INX => cpu.inx(),
            Instruction::INY => cpu.iny(),
            Instruction::ISB => cpu.isb(addr),
            Instruction::JAM => cpu.jam(),
            Instruction::JMP => cpu.jmp(addr),
            Instruction::JSR => cpu.jsr(addr),
            Instruction::LAX => cpu.lax(addr),
            Instruction::LDA => cpu.lda(addr),
            Instruction::LDX => cpu.ldx(addr),
            Instruction::LDY => cpu.ldy(addr),
            Instruction::LSR => cpu.lsr(addr, addr_mode),
            Instruction::NOP => cpu.nop(),
            Instruction::ORA => cpu.ora(addr),
            Instruction::PHA => cpu.pha(),
            Instruction::PHP => cpu.php(),
            Instruction::PLA => cpu.pla(),
            Instruction::PLP => cpu.plp(),
            Instruction::RLA => cpu.rla(addr, addr_mode),
            Instruction::ROL => cpu.rol(addr, addr_mode),
            Instruction::ROR => cpu.ror(addr, addr_mode),
            Instruction::RRA => cpu.rra(addr, addr_mode),
            Instruction::RTI => cpu.rti(),
            Instruction::RTS => cpu.rts(),
            Instruction::SAX => cpu.sax(addr),
            Instruction::SBC => cpu.sbc(addr),
            Instruction::SEC => cpu.sec(),
            Instruction::SED => cpu.sed(),
            Instruction::SEI => cpu.sei(),
            Instruction::SLO => cpu.slo(addr, addr_mode),
            Instruction::SRE => cpu.sre(addr, addr_mode),
            Instruction::STA => cpu.sta(addr),
            Instruction::STX => cpu.stx(addr),
            Instruction::STY => cpu.sty(addr),
            Instruction::TAX => cpu.tax(),
            Instruction::TAY => cpu.tay(),
            Instruction::TSX => cpu.tsx(),
            Instruction::TXA => cpu.txa(),
            Instruction::TXS => cpu.txs(),
            Instruction::TYA => cpu.tya(),
            _ => panic!("unsupported instruction {:?}", *self),
        }
    }
}
