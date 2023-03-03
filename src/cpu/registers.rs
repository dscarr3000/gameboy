/// The Game Boy CPU contains 8 registers, each register is 8 bits (1 byte). The registers are labeled as: a, b, c, d, e, f, h, l.
///
/// The CPU only has 8 bit registers, but there are instructions that can read and write 16 bits. We'll need "virtual" 16 bit registers, which are: af, bc, de, hl
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

/// ## Summary
/// Register "f" is a special register called the "flags" register. The lower four bits of the register are ALWAYS 0s and the CPU automatically writes to the upper four bits when certain things happen.
///
/// The names and positions of the flags are:
/// - Bit 7: zero
/// - Bit 6: subtraction
/// - Bit 5: half carry
/// - Bit 4: carry
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl Registers {
    /// ## Summary
    /// Gets the value from the "virtual" 16 bit register af. The value in register a being the 8 left-most bits and register 
    /// f being the 8 right-most bits.
    /// ## Returns
    /// A 16 bit unsigned integer.
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    /// ## Summary
    /// Sets the value of the "virtual" 16 bit register af. Register a will contain the 8 left-most bits and register f 
    /// will contain the 8 right-most bits.
    /// ## Parameters
    /// - value: A 16 bit unsigned integer.
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    /// ## Summary
    /// Gets the value from the "virtual" 16 bit register bc. The value in register b being the 8 left-most bits and register 
    /// c being the 8 right-most bits.
    /// ## Returns
    /// A 16 bit unsigned integer.
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// ## Summary
    /// Sets the value of the "virtual" 16 bit register bc. Register b will contain the 8 left-most bits and register c 
    /// will contain the 8 right-most bits.
    /// ## Parameters
    /// - value: A 16 bit unsigned integer.
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    /// ## Summary
    /// Gets the value from the "virtual" 16 bit register de. The value in register d being the 8 left-most bits and register 
    /// e being the 8 right-most bits.
    /// ## Returns
    /// A 16 bit unsigned integer.
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// ## Summary
    /// Sets the value of the "virtual" 16 bit register de. Register d will contain the 8 left-most bits and register e 
    /// will contain the 8 right-most bits.
    /// ## Parameters
    /// - value: A 16 bit unsigned integer.
    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    /// ## Summary
    /// Gets the value from the "virtual" 16 bit register hl. The value in register h being the 8 left-most bits and register 
    /// l being the 8 right-most bits.
    /// ## Returns
    /// A 16 bit unsigned integer.
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// ## Summary
    /// Sets the value of the "virtual" 16 bit register hl. Register h will contain the 8 left-most bits and register l 
    /// will contain the 8 right-most bits.
    /// ## Parameters
    /// - value: A 16 bit unsigned integer.
    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    /// ## Summary
    /// Converts the flag booleans to an 8 bit unsigned integer representation.
    /// ## Returns
    /// An 8 bit Unsigned Integer.
    fn from(flag: FlagsRegister) -> u8 {
        // (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        // (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        // (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        // (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
        (flag.zero as u8) << ZERO_FLAG_BYTE_POSITION |
        (flag.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION |
        (flag.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION |
        (flag.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    /// ## Summary
    /// Converts an 8 bit unsigned integer into the appropriate flag booleans.
    /// ## Parameters
    /// byte: An 8 bit Unsigned Integer.
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}