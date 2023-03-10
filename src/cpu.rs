use registers::Register;
mod registers;
use instruction::Instruction;
mod instruction;
use memory_bus::MemoryBus;
mod memory_bus;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
          instruction_byte = self.bus.read_byte(self.pc + 1);
        }
    
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
          self.execute(instruction)
        } else {
          let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
          panic!("Unkown instruction found for: {}", description)
        };
    
        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => todo!(),
                ArithmeticTarget::B => todo!(),
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                },
                ArithmeticTarget::D => todo!(),
                ArithmeticTarget::E => todo!(),
                ArithmeticTarget::H => todo!(),
                ArithmeticTarget::L => todo!(),
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
          // Gameboy is little endian so read pc + 2 as most significant bit
          // and pc + 1 as least significant bit
          let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
          let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
          (most_significant_byte << 8) | least_significant_byte
        } else {
          // If we don't jump we need to still move the program
          // counter forward by 3 since the jump instruction is
          // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
          self.pc.wrapping_add(3)
        }
    }
}