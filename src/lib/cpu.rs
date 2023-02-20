
mod instruction;
pub use instruction::{InstructionType, Instruction};

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
        }
    }

    ///  Execute an instruction
    pub fn execute(&mut self, instruction: Instruction, ram: &mut Vec<u8>) {
        println!("{}", instruction.value);
        ram[0] = 5;
    }

    /// Fetch memory pointed by program counter
    pub fn fetch(&mut self, ram: &mut Vec<u8>) -> Instruction {
        let ret = Instruction::new(ram[self.pc]);
        self.pc += 1;
        ret
    }

}
