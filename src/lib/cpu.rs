
mod instruction;
pub use instruction::{InstructionType, Instruction, AddressingMode};

#[derive(Copy, Clone)]
enum StatusFlags {
    /// carry flag
    C = (1 << 0),
    /// zero flag
    Z = (1 << 1),
    /// interrupt disable
    I = (1 << 2),
    /// decimal mode
    D = (1 << 3),
    /// break command
    B = (1 << 4),
    /// overflow flag
    V = (1 << 5),
    /// Negative flag
    N = (1 << 6),
}

/// 6502 CPU emulator
pub struct CPU {
    /// Accumulator register
    a: u8,
    /// X index register
    x: u8,
    /// Y index register
    y: u8,
    /// Program counter
    pc: usize,
    /// Stack pointer
    sp: usize,
    /// Processor status flag register
    status: u8,
    /// Cycles left for current instruction
    cycles: u8,
    /// Address pointed by the addressing mode
    address: usize,
    /// Relative address for branching
    branch_address: usize,

}

impl CPU {

    // Create an instance of the CPU.
    pub fn new() -> CPU {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            cycles: 0,
            address: 0,
            branch_address: 0,
        }
    }

    ///  Execute one clock cycle
    pub fn tick(&mut self, bus: &mut Vec<u8>) {
        if self.cycles > 0 {
            self.cycles -= 1;
            return;
        }
        let instruction = self.fetch_instruction(bus);
        self.cycles = instruction.cycles - 1; // Remove this cycle
        let mut add_cycles = self.addressing_mode(&instruction.addressing_mode, bus);
        add_cycles &= self.execute(&instruction, bus);
        if add_cycles {
            self.cycles += 1;
        }
    }

    /// Fetch memory pointed by program counter
    fn fetch_instruction(&mut self, bus: &Vec<u8>) -> Instruction {
        let ret = Instruction::new(bus[self.pc]);
        self.pc += 1;
        ret
    }

    /// Fetch memory based on addressing mode.
    fn fetch_memory(&mut self, bus: &mut Vec<u8>) -> u8 {
        return bus[self.address];
    }

    /// Execute operation related to addressing mode.
    fn addressing_mode(&mut self, mode: &AddressingMode, bus: &Vec<u8>) -> bool {
        match mode {
            AddressingMode::Immediate => {
                return self.immediate();
            }
            AddressingMode::Implied => {
                return false;
            }
            AddressingMode::ZeroPage => {
                return self.zero_page(bus);
            }
            AddressingMode::ZeroPageX => {
                return self.zero_page_x(bus);
            }
            AddressingMode::ZeroPageY => {
                return self.zero_page_y(bus);
            }
            AddressingMode::Relative => {
                return self.relative(bus);
            }
            AddressingMode::Absolute => {
                return self.absolute(bus);
            }
            AddressingMode::AbsoluteX => {
                return self.absolute_x(bus);
            }
            AddressingMode::AbsoluteY => {
                return self.absolute_y(bus);
            }
            AddressingMode::Indirect => {
                return self.indirect(bus);
            }
            AddressingMode::IndirectX => {
                return self.indirect_x(bus);
            }
            AddressingMode::IndirectY => {
                return self.indirect_y(bus);
            }
            AddressingMode::Accumulator => {
                return false;
            }
        }
    }

    /// Execute instruction.
    fn execute(&mut self, instruction: &Instruction, bus: &mut Vec<u8>) -> bool {
        match instruction.itype {
            InstructionType::ADC => {
                return self.adc(bus);
            }
            InstructionType::AND => {
                return self.and(bus);
            }
            InstructionType::ASL => {
                return self.asl(&instruction.addressing_mode, bus);
            }
            InstructionType::BCC => {
                return self.bcc();
            }
            InstructionType::BCS => {
                return self.bcs();
            }
            InstructionType::BEQ => {
                return self.beq();
            }
            InstructionType::BIT => {
                return false;
            }
            InstructionType::BMI => {
                return self.bmi();
            }
            InstructionType::BNE => {
                return self.bne();
            }
            InstructionType::BPL => {
                return self.bpl();
            }
            InstructionType::BRK => {
                return false;
            }
            InstructionType::BVC => {
                return self.bvc();
            }
            InstructionType::BVS => {
                return self.bvs();
            }
            InstructionType::CLC => {
                return self.clc();
            }
            InstructionType::CLD => {
                return self.cld();
            }
            InstructionType::CLI => {
                return self.cli();
            }
            InstructionType::CLV => {
                return self.clv();
            }
            InstructionType::CMP => {
                return self.cmp(bus);
            }
            InstructionType::CPX => {
                return self.cpx(bus);
            }
            InstructionType::CPY => {
                return self.cpy(bus);
            }
            InstructionType::DEC => {
                return self.dec(bus);
            }
            InstructionType::DEX => {
                return self.dex();
            }
            InstructionType::DEY => {
                return self.dey();
            }
            InstructionType::EOR => {
                return self.eor(bus);
            }
            InstructionType::INC => {
                return self.inc(bus);
            }
            InstructionType::INX => {
                return self.inx();
            }
            InstructionType::INY => {
                return self.iny();
            }
            InstructionType::JMP => {
                return self.jmp();
            }
            InstructionType::JSR => {
                return self.jsr(bus);
            }
            InstructionType::LDA => {
                return self.lda(bus);
            }
            InstructionType::LDX => {
                return self.ldx(bus);
            }
            InstructionType::LDY => {
                return self.ldy(bus);
            }
            InstructionType::LSR => {
                return false;
            }
            InstructionType::NOP => {
                return false;
            }
            InstructionType::ORA => {
                return false;
            }
            InstructionType::PHA => {
                return false;
            }
            InstructionType::PHP => {
                return false;
            }
            InstructionType::PLA => {
                return false;
            }
            InstructionType::PLP => {
                return false;
            }
            InstructionType::ROL => {
                return false;
            }
            InstructionType::ROR => {
                return false;
            }
            InstructionType::RTI => {
                return false;
            }
            InstructionType::RTS => {
                return false;
            }
            InstructionType::SBC => {
                return self.sbc(bus);
            }
            InstructionType::SEC => {
                return false;
            }
            InstructionType::SED => {
                return false;
            }
            InstructionType::SEI => {
                return false;
            }
            InstructionType::STA => {
                return false;
            }
            InstructionType::STX => {
                return false;
            }
            InstructionType::STY => {
                return false;
            }
            InstructionType::TAX => {
                return false;
            }
            InstructionType::TAY => {
                return false;
            }
            InstructionType::TSX => {
                return false;
            }
            InstructionType::TXA => {
                return false;
            }
            InstructionType::TXS => {
                return false;
            }
            InstructionType::TYA => {
                return false;
            }
        }
    }

    fn overflow_add(&self, a: u8, b: u8) -> (bool, u8) {
        let res = a as u16 + b as u16;
        (res > 0xFF, (res & 0xFF) as u8)
    }

    fn overflow_subtract(&self, a: u8, b: u8) -> (bool, u8) {
        let res = a as i16 - b as i16;
        (res < 0, (res & 0xFF) as u8)
    }


    fn set_flag(&mut self, flag: StatusFlags) {
        self.status |= flag as u8;
    }

    fn check_flag(&self, flag: StatusFlags) -> bool {
        return self.status & flag as u8 == flag as u8;
    }

    fn clear_flag(&mut self, flag: StatusFlags) {
        if self.check_flag(flag) {
            self.status ^= flag as u8;
        }
    }

    /* Different addressing modes: */
    fn immediate(&mut self) -> bool {
        self.address = self.pc;
        self.pc += 1;
        return false;
    }

    fn zero_page(&mut self, bus: &Vec<u8>) -> bool {
        self.address = bus[self.pc] as usize;
        self.pc += 1;
        return false;
    }

    fn zero_page_x(&mut self, bus: &Vec<u8>) -> bool {
        self.address = ((bus[self.pc] + self.x) & 0xFF) as usize;
        self.pc += 1;
        return false;
    }

    fn zero_page_y(&mut self, bus: &Vec<u8>) -> bool {
        self.address = (bus[self.pc] as usize + self.y as usize) & 0xFF;
        self.pc += 1;
        return false;
    }

    fn absolute(&mut self, bus: &Vec<u8>) -> bool {
        self.address = bus[self.pc] as usize | ((bus[self.pc + 1] as usize) << 8);
        self.pc += 2;
        return false;
    }

    fn absolute_x(&mut self, bus: &Vec<u8>) -> bool {
        self.absolute(bus);
        let prev = self.address;
        self.address += self.x as usize;
        if (prev & 0xFF00) != (self.address & 0xFF00) {
            return true;
        }
        return false;
    }

    fn absolute_y(&mut self, bus: &Vec<u8>) -> bool {
        self.absolute(bus);
        let prev = self.address;
        self.address += self.y as usize;
        if (prev & 0xFF00) != (self.address & 0xFF00) {
            return true;
        }
        return false;
    }

    fn indirect(&mut self, bus: &Vec<u8>) -> bool {
        let address = bus[self.pc] as usize | ((bus[self.pc + 1] as usize) << 8);
        let mut inc: usize = 1;
        if address & 0xFF == 0xFF {
            inc = 0;
        }
        self.address = bus[address] as usize | ((bus[address + inc] as usize) << 8);
        self.pc += 2;
        return false;
    }

    fn indirect_x(&mut self, bus: &Vec<u8>) -> bool {
        let base = bus[self.pc] as usize;
        let base_x = base + self.x as usize;
        self.address = bus[base_x] as usize | (bus[base_x + 1] as usize) << 8;

        return false;
    }

    fn indirect_y(&mut self, bus: &Vec<u8>) -> bool {
        let base = bus[self.pc] as usize;
        self.address = bus[base] as usize | (bus[base + 1] as usize) << 8;
        let prev = self.address;
        self.address += self.y as usize;

        if self.address & 0xFF00 == prev & 0xFF00 {
            return false;
        }

        return true;
    }

    fn relative(&mut self, bus: &Vec<u8>) -> bool {
        self.branch_address = bus[self.pc] as usize;
        self.pc += 1;
        if (self.branch_address & 0x80) == 0x80 {
            self.branch_address |= 0xFF00;
        }
        return false;
    }

    /* Instruction implementations */
    fn adc(&mut self, bus: &mut Vec<u8>) -> bool {
        let mut val = bus[self.address];
        if self.check_flag(StatusFlags::C) {
            val += 1;
        }
        let res = self.overflow_add(self.a, val);
        if res.0 {
            self.set_flag(StatusFlags::C);
        }
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        } else if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        if (val & 0x80 == 0x80) && (res.1 & 0x80 == 0x80) && (self.a & 0x80 != 0x80)
            || (val & 0x80 == 0) && (res.1 & 0x80 == 0) && (self.a & 0x80 != 0) {
            self.set_flag(StatusFlags::V);
        }
        self.a = res.1;
        return true;
    }

    fn sbc(&mut self, bus: &Vec<u8>) -> bool {
        let mut val = bus[self.address];

        if !self.check_flag(StatusFlags::C) {
            val += 1;
        }

        //val  ^= 0xFF;

        let res = self.overflow_subtract(self.a, val);
        if res.0 {
            self.set_flag(StatusFlags::C);
        }
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        } else if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
            self.clear_flag(StatusFlags::C);
        }
        if (val & 0x80 == 0x80) && (res.1 & 0x80 == 0x80) && (self.a & 0x80 != 0x80)
            || (val & 0x80 == 0) && (res.1 & 0x80 == 0) && (self.a & 0x80 != 0) {
            self.set_flag(StatusFlags::V);
        }
        self.a = res.1;
        return true;
    }

    fn and(&mut self, bus: &mut Vec<u8>) -> bool {
        self.a &= self.fetch_memory(bus);
        if self.a == 0 {
            self.set_flag(StatusFlags::Z);
        }
        else if self.a & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }

        return true;
    }

    fn asl(&mut self, mode: &AddressingMode, bus: &mut Vec<u8>) -> bool {
        let mut val: u8;

        match mode {
            AddressingMode::Accumulator => {
                val = self.a;
            }
            _ => {
                val = bus[self.address];
            }
        }
        if val & 0x80 == 0x80 {
            self.set_flag(StatusFlags::C);
            val ^= 0x80;
        }
        val *= 2;

        if val == 0 {
            self.set_flag(StatusFlags::Z);
        } else if val & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }

        match mode {
            AddressingMode::Accumulator => {
                self.a = val;
            }
            _ => {
                bus[self.address] = val;
            }
        }

        return false;
    }

    fn branch_if(&mut self, value: bool) -> bool {
        if value {
            self.cycles += 1;
            let prev = self.pc;
            self.pc += self.branch_address;
            if prev & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
        }
        return false;
    }

    fn bcs(&mut self) -> bool {
        return self.branch_if(self.check_flag(StatusFlags::C));
    }

    fn bcc(&mut self) -> bool {
        return self.branch_if(!self.check_flag(StatusFlags::C));
    }
    fn beq(&mut self) -> bool {
        return self.branch_if(self.check_flag(StatusFlags::Z));
    }

    fn bmi(&mut self) -> bool {
        return self.branch_if(self.check_flag(StatusFlags::N));
    }

    fn bne(&mut self) -> bool {
        return self.branch_if(!self.check_flag(StatusFlags::Z));
    }

    fn bpl(&mut self) -> bool {
        return self.branch_if(!self.check_flag(StatusFlags::N));
    }

    fn bvc(&mut self) -> bool {
        return self.branch_if(self.check_flag(StatusFlags::V));
    }

    fn bvs(&mut self) -> bool {
        return self.branch_if(!self.check_flag(StatusFlags::V));
    }

    fn clc(&mut self) -> bool {
        self.status &= 0xFF & StatusFlags::C as u8;
        return false;
    }

    fn cli(&mut self) -> bool {
        self.status &= 0xFF & StatusFlags::I as u8;
        return false;
    }

    fn cld(&mut self) -> bool {
        self.status &= 0xFF & StatusFlags::D as u8;
        return false;
    }

    fn clv(&mut self) -> bool {
        self.status &= 0xFF & StatusFlags::V as u8;
        return false;
    }

    fn compare(&mut self, bus: &Vec<u8>, value: u8) {
        if value >= bus[self.address] {
            self.set_flag(StatusFlags::C);
        }
        if value == bus[self.address] {
            self.set_flag(StatusFlags::Z);
        }
        if value & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
    }

    fn cmp(&mut self, bus: &Vec<u8>) -> bool {
        self.compare(bus, self.a);
        return true;
    }

    fn cpx(&mut self, bus: &Vec<u8>) -> bool {
        self.compare(bus, self.x);
        return true;
    }

    fn cpy(&mut self, bus: &Vec<u8>) -> bool {
        self.compare(bus, self.y);
        return true;
    }

    fn dec(&mut self, bus: &mut Vec<u8>) -> bool {
        let res = self.overflow_subtract(bus[self.address], 1);
        bus[self.address] = res.1;
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        }
        if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }

        return false;
    }

    fn dex(&mut self) -> bool {
        let res = self.overflow_subtract(self.x, 1);
        self.x = res.1;
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        }
        else if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return false;
    }
    fn dey(&mut self) -> bool {
        let res = self.overflow_subtract(self.y, 1);
        self.y = res.1;
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        }
        else if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return false;
    }

    fn eor(&mut self, bus: &Vec<u8>) -> bool {
        self.a ^= bus[self.address];
        if self.a == 0 {
            self.set_flag(StatusFlags::Z);
        }
        else if self.a & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return true;
    }

    fn inc(&mut self, bus: &mut Vec<u8>) -> bool {
        let res = self.overflow_add(bus[self.address], 1);
        if res.1 == 0 {
            self.set_flag(StatusFlags::Z);
        } else if res.1 & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return false;
    }

    fn inx(&mut self) -> bool {
        let res = self.overflow_add(self.x, 1);
        self.x = res.1;
        if self.x == 0 {
            self.set_flag(StatusFlags::Z);
        } else if self.x & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return false;
    }

    fn iny(&mut self) -> bool {
        let res = self.overflow_add(self.y, 1);
        self.y = res.1;
        if self.x == 0 {
            self.set_flag(StatusFlags::Z);
        } else if self.x & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return false;
    }

    fn jmp(&mut self) -> bool {
        self.pc = self.address;
        return false;
    }

    fn jsr(&mut self, bus: &mut Vec<u8>) -> bool {
        bus[self.sp] = ((self.pc - 1) & 0xFF) as u8;
        bus[self.sp +1] = (((self.pc - 1) & 0xFF00) >> 8) as u8;
        self.sp -= 2;

        self.pc = self.address;
        return false;
    }

    fn lda(&mut self, bus: &Vec<u8>) -> bool {
        self.a = bus[self.address];
        if self.a == 0 {
            self.set_flag(StatusFlags::Z)
        } else if self.a & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return true;
    }

    fn ldx(&mut self, bus: &Vec<u8>) -> bool {
        self.x = bus[self.address];
        if self.x == 0 {
            self.set_flag(StatusFlags::Z)
        } else if self.x & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return true;
    }

    fn ldy(&mut self, bus: &Vec<u8>) -> bool {
        self.y = bus[self.address];
        if self.y == 0 {
            self.set_flag(StatusFlags::Z)
        } else if self.y & 0x80 == 0x80 {
            self.set_flag(StatusFlags::N);
        }
        return true;
    }

}


#[cfg(test)]
mod tests {
    use super::CPU;
    use super::StatusFlags;

    #[test]
    pub fn test_flags() {
        let mut cpu = CPU::new();
        cpu.set_flag(StatusFlags::Z);
        assert!(cpu.check_flag(StatusFlags::Z));
    }

    #[test]
    pub fn test_subtract() {
        let cpu = CPU::new();
        assert_eq!(cpu.overflow_subtract(10, 10).1, 0);
    }

    #[test]
    pub fn test_arithmetic() {
        let mut cpu = CPU::new();
        let mut bus: Vec<u8> = vec![0; 1000];
        bus[0] = 0x69;
        bus[1] = 10;
        bus[2] = 0x69;
        bus[3] = 10;
        cpu.pc = 0;

        cpu.tick(&mut bus);
        cpu.tick(&mut bus);

        assert_eq!(cpu.a, 10);
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);

        assert_eq!(cpu.a, 20);

        cpu.pc = 0;
        bus[0] = 0xE9;
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);
        assert_eq!(cpu.a, 9);

        cpu.pc = 0;
        bus[1] = 9;
        cpu.set_flag(StatusFlags::C);
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);
        assert_eq!(cpu.a, 0);

        cpu.pc = 0;
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);
        assert_eq!(cpu.a, cpu.overflow_subtract(0, 9).1);
        assert!(!cpu.check_flag(StatusFlags::C));

        cpu.a = 0;
        cpu.pc = 0;
        bus[0] = 0x69;
        bus[1] = 0;

        cpu.status = 0;
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);

        assert_eq!(cpu.a, 0);
        assert!(cpu.check_flag(StatusFlags::Z));
        assert!(!cpu.check_flag(StatusFlags::N));
        assert!(!cpu.check_flag(StatusFlags::V));

        cpu.pc = 0;
        bus[1] = 0x80;
        cpu.tick(&mut bus);
        cpu.tick(&mut bus);

        assert!(cpu.check_flag(StatusFlags::N));
        assert!(cpu.check_flag(StatusFlags::V));

    }

}