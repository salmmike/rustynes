
#[derive(Debug)]
pub enum InstructionType {
    /// Arithmetic
    ADC,
    SBC,
    /// Boolean operator
    AND, ORA,
    /// Shift
    ASL, LSR,
    /// Bit test
    BIT,
    /// Interrupt
    BRK, RTI,
    /// Branch
    BEQ, BCC, BCS, BMI, BNE, BPL, BVC, BVS,
    /// Clear
    CLC, CLD, CLI, CLV,
    /// Compare
    CMP, CPX, CPY,
    /// Decrement
    DEC, DEX, DEY,
    EOR,
    /// Increment
    INC, INX, INY,
    /// Jump
    JMP, RTS,
    /// Subroutine
    JSR,
    /// Load
    LDA, LDX, LDY,
    /// No operation
    NOP,
    /// Push
    PHA, PHP,
    /// Pull
    PLA, PLP,
    /// Rotate
    ROL, ROR,
    /// Set
    SEC, SED, SEI,
    /// Store
    STA, STX, STY,
    /// Transfer
    TAX, TAY, TSX, TXA, TXS, TYA,
}

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Implied,
}

/// 6502 instruction
pub struct Instruction {
    pub value: u8,
    pub itype: InstructionType,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub bytes: u8,
}

fn adc(value: u8) -> Instruction {
    let itype = InstructionType::ADC;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x69 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0x65 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x75 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x6D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0x7D => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0x79 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0x61 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0x71 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }
    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn and(value: u8) -> Instruction {
    let itype = InstructionType::AND;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x29 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0x25 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x35 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x2D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0x3D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0x39 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0x21 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0x31 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }
    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn asl(value: u8) -> Instruction {
    let itype = InstructionType::ASL;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x0A => {
            bytes = 1;
            cycles = 2;
            addressing_mode = AddressingMode::Accumulator;
        }
        0x06 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x16 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x0E => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0x1E => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn bit(value: u8) -> Instruction {
    let itype = InstructionType::BIT;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x24 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x2C => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        _ => {
        }
    }
    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn cmp(value: u8) -> Instruction {
    let itype = InstructionType::CMP;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xC9 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xC5 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xD5 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0xCD => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;

        }
        0xDD => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0xD9 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0xC1 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0xD1 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }
    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn cpy(value: u8) -> Instruction {
    let itype = InstructionType::CPY;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xC0 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xC4 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xCC => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn dec(value: u8) -> Instruction {
    let itype = InstructionType::DEC;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xC6 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xD6 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;

        }
        0xCE => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0xDE => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn eor(value: u8) -> Instruction {
    let itype = InstructionType::EOR;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x49 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0x45 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x55 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x4D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0x5D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0x59 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0x41 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX
        }
        0x51 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn inc(value: u8) -> Instruction {
    let itype = InstructionType::INC;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xE6 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xF6 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0xEE => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0xFE => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn jmp(value: u8) -> Instruction {
    let itype = InstructionType::JMP;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x4C => {
            bytes = 3;
            cycles = 3;
            addressing_mode = AddressingMode::Absolute;
        }
        0x6C => {
            bytes = 3;
            cycles = 5;
            addressing_mode = AddressingMode::Indirect;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn lda(value: u8) -> Instruction {
    let itype = InstructionType::LDA;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xA9 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xA5 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xB5 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0xAD => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0xBD => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0xB9 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0xA1 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0xB1 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}


fn ldx(value: u8) -> Instruction {
    let itype = InstructionType::LDX;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xA2 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xA6 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xB6 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageY;
        }
        0xAE => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0xBE => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn ldy(value: u8) -> Instruction {
    let itype = InstructionType::LDY;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xA0 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xA4 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xB4 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0xAC => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0xBC => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn lsr(value: u8) -> Instruction {
    let itype = InstructionType::LSR;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x4A => {
            bytes = 1;
            cycles = 2;
            addressing_mode = AddressingMode::Accumulator;
        }
        0x46 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x56 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x4E => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0x5E => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn ora(value: u8) -> Instruction {
    let itype = InstructionType::ORA;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x09 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0x05 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x15 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x0D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0x1D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0x19 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0x01 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0x11 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn rol(value: u8) -> Instruction {
    let itype = InstructionType::ROL;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x2A => {
            bytes = 1;
            cycles = 2;
            addressing_mode = AddressingMode::Accumulator;
        }
        0x26 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x36 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x2E => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0x3E => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}


fn ror(value: u8) -> Instruction {
    let itype = InstructionType::ROR;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x2A => {
            bytes = 1;
            cycles = 2;
            addressing_mode = AddressingMode::Accumulator;
        }
        0x26 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x36 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x2E => {
            bytes = 3;
            cycles = 6;
            addressing_mode = AddressingMode::Absolute;
        }
        0x3E => {
            bytes = 3;
            cycles = 7;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn sbc(value: u8) -> Instruction {
    let itype = InstructionType::SBC;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0xE9 => {
            bytes = 2;
            cycles = 2;
            addressing_mode = AddressingMode::Immediate;
        }
        0xE5 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0xF5 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0xED => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0xFD => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0xF9 => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0xE1 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0xF1 => {
            bytes = 2;
            cycles = 5;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn sta(value: u8) -> Instruction {
    let itype = InstructionType::STA;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x85 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x95 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x8D => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        0x9D => {
            bytes = 3;
            cycles = 5;
            addressing_mode = AddressingMode::AbsoluteX;
        }
        0x99 => {
            bytes = 3;
            cycles = 5;
            addressing_mode = AddressingMode::AbsoluteY;
        }
        0x81 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectX;
        }
        0x91 => {
            bytes = 2;
            cycles = 6;
            addressing_mode = AddressingMode::IndirectY;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn stx(value: u8) -> Instruction {
    let itype = InstructionType::STX;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x86 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x96 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageY;
        }
        0x8E => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

fn sty(value: u8) -> Instruction {
    let itype = InstructionType::STY;
    let mut bytes = 0;
    let mut cycles = 0;
    let mut addressing_mode = AddressingMode::Implied;

    match value {
        0x84 => {
            bytes = 2;
            cycles = 3;
            addressing_mode = AddressingMode::ZeroPage;
        }
        0x94 => {
            bytes = 2;
            cycles = 4;
            addressing_mode = AddressingMode::ZeroPageX;
        }
        0x8C => {
            bytes = 3;
            cycles = 4;
            addressing_mode = AddressingMode::Absolute;
        }
        _ => {
        }
    }

    Instruction {
        value: value,
        itype: itype,
        addressing_mode: addressing_mode,
        cycles: cycles,
        bytes: bytes,
    }
}

impl Instruction {
    pub fn new(value: u8) -> Instruction {
        let itype = InstructionType::NOP;
        let addressing_mode = AddressingMode::Immediate;
        let cycles: u8 = 1;
        let bytes: u8 = 1;

        match value {
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                return adc(value);
            }
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                return and(value);
            }
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                return asl(value);

            }
            0xB0 => {
                return Instruction {
                    value: value,
                    itype: InstructionType::BCC, bytes: 2, cycles: 2,
                    addressing_mode: AddressingMode::Relative,
                }
            }
            0x90 => {
                return Instruction {
                    value: value, itype: InstructionType::BCS,
                    bytes: 2, cycles: 2, addressing_mode: AddressingMode::Relative
                }
            }
            0xF0 => {
                return Instruction {
                    value: value, itype: InstructionType::BEQ,
                    bytes: 2, cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0x24 | 0x2C => {
                return bit(value);
            }
            0x30 => {
                return Instruction {
                    value: value, itype: InstructionType::BMI, bytes: 2,
                    cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0xD0 => {
                return Instruction {
                    value: value, itype: InstructionType::BNE, bytes: 2,
                    cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0x10 => {
                return Instruction {
                    value: value, itype: InstructionType::BPL, bytes: 2,
                    cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0x00 => {
                return Instruction {
                    value: value, itype: InstructionType::BRK, bytes: 1,
                    cycles: 7, addressing_mode: AddressingMode::Implied,
                }
            }
            0x50 => {
                return Instruction {
                    value: value, itype: InstructionType::BVC, bytes: 2,
                    cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0x70 => {
                return Instruction {
                    value: value, itype: InstructionType::BVS, bytes: 2,
                    cycles: 2, addressing_mode: AddressingMode::Relative,
                }
            }
            0x18 => {
                return Instruction {
                    value: value, itype: InstructionType::CLC, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                }
            }
            0xD8 => {
                return Instruction {
                    value: value, itype: InstructionType::CLD, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x58 => {
                return Instruction {
                    value: value, itype: InstructionType::CLI, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xB8 => {
                return Instruction {
                    value: value, itype: InstructionType::CLV, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                return cmp(value);
            }
            0xC0 | 0xC4 | 0xCC => {
                return cpy(value);
            }
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                return dec(value);
            }
            0xCA => {
                return Instruction {
                    value: value, itype: InstructionType::DEX, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x88 => {
                return Instruction {
                    value: value, itype: InstructionType::DEY, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                return eor(value);
            }
            0xE6 | 0xF6 | 0xEE | 0xFE => {
                return inc(value);
            }
            0xE8 => {
                return Instruction {
                    value: value, itype: InstructionType::INX, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xC8 => {
                return Instruction {
                    value: value, itype: InstructionType::INY, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x4C | 0x6C => {
                return jmp(value);
            }
            0x20 => {
                return Instruction {
                    value: value, itype: InstructionType::JSR, bytes: 3,
                    cycles: 6, addressing_mode: AddressingMode::Absolute,
                };
            }
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                return lda(value);
            }
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                return ldx(value);
            }
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                return ldy(value);
            }
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                return lsr(value);
            }
            0xEA => {
                return Instruction {
                    value: value, itype: InstructionType::NOP, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x09 | 0x15 | 0x05 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                return ora(value);
            }
            0x48 => {
                return Instruction {
                    value: value, itype: InstructionType::PHA, bytes: 1,
                    cycles: 3, addressing_mode: AddressingMode::Implied,
                };
            }
            0x08 => {
                return Instruction {
                    value: value, itype: InstructionType::PHP, bytes:1,
                    cycles: 3, addressing_mode: AddressingMode::Implied,
                };
            }
            0x68 => {
                return Instruction {
                    value: value, itype: InstructionType::PLA, bytes: 1,
                    cycles: 4, addressing_mode: AddressingMode::Implied,
                };
            }
            0x28 => {
                return Instruction {
                    value: value, itype: InstructionType::PLP, bytes: 1,
                    cycles: 4, addressing_mode: AddressingMode::Implied,
                };
            }
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                return rol(value);
            }
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                return ror(value);
            }
            0x40 => {
                return Instruction {
                    value: value, itype: InstructionType::RTI, bytes: 1,
                    cycles: 6, addressing_mode: AddressingMode::Implied,
                };
            }
            0x60 => {
                return Instruction {
                    value: value, itype: InstructionType::RTS, bytes: 1,
                    cycles: 6, addressing_mode: AddressingMode::Implied,
                };
            }
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                return sbc(value);
            }
            0x38 => {
                return Instruction {
                    value: value, itype: InstructionType::SEC, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xF8 => {
                return Instruction {
                    value: value, itype: InstructionType::SED, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x78 => {
                return Instruction {
                    value: value, itype: InstructionType::SEI, bytes: 1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                return sta(value);
            }
            0x86 | 0x96 | 0x8E => {
                return stx(value);
            }
            0x84 | 0x94 | 0x8C => {
                return sty(value);
            }
            0xAA => {
                Instruction {
                    value: value, itype: InstructionType::TAX, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xA8 => {
                Instruction {
                    value: value, itype: InstructionType::TAY, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0xBA => {
                Instruction {
                    value: value, itype: InstructionType::TSX, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x8A => {
                Instruction {
                    value: value, itype: InstructionType::TXA, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            0x9A => {
                Instruction {
                    value: value, itype: InstructionType::TXS, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };           }
            0x98 => {
                Instruction {
                    value: value, itype: InstructionType::TYA, bytes:1,
                    cycles: 2, addressing_mode: AddressingMode::Implied,
                };
            }
            _ => {
                println!("Illeagal instruction? {}", value);
            }
        }

        Instruction {
            value: value,
            itype: itype,
            addressing_mode: addressing_mode,
            cycles: cycles,
            bytes: bytes,
        }
    }
    pub fn print(&self) {
        println!("Instruction: {:?}, Addressing {:?}", self.itype, self.addressing_mode);
    }

}