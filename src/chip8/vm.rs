use macroquad::prelude::debug;

use super::{Instruction, InstructionDecodeError, Stack, Display};

pub const MEMORY_SIZE: usize = 4096;
pub const VREG_COUNT: usize = 16;

pub struct VM {
    pub memory: [u8; MEMORY_SIZE],
    pub program_counter: usize,
    pub index_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub variable_registers: [u8; VREG_COUNT],
    pub stack: Stack,
    pub display: Display,
    pub shift_legacy: bool,
    pub chip48_mode: bool
}

#[derive(Debug)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
    UnsupportedInstruction
}

impl VM {
    pub fn new() -> VM {
        VM { 
            memory: [0; MEMORY_SIZE],
            program_counter: 0,
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; VREG_COUNT],
            stack: Stack {  },
            display: Display {  },
            shift_legacy: false,
            chip48_mode: true
        }
    }
    pub fn mem_copy(&mut self, buf: &[u8], offset: usize) {
        let end = offset + buf.len();
        self.memory[offset..end].copy_from_slice(buf);
    }
    pub fn load_program_from_file(&mut self, file_path: &std::path::Path, load_location: usize) {
        // Read the contents of the file into a byte vector
        let mut buffer = Vec::new();
        if let Ok(mut file) = std::fs::File::open(file_path) {
            if let Ok(metadata) = file.metadata() {
                buffer.reserve_exact(metadata.len() as usize);
            }
            std::io::Read::read_to_end(&mut file, &mut buffer).ok();
        }

        // Load the program into memory at the specified location
        self.mem_copy(&buffer, load_location);
    }
    
    pub fn fetch(&self) -> Result<Instruction, InstructionDecodeError> {
        let raw_inst: u16 = (((self.memory[self.program_counter] as u16) << 8) | (self.memory[self.program_counter + 1] as u16)).into();

        let opcode = raw_inst & 0xF000;

        let x = (raw_inst & 0x0F00) >> 8;
        let y = (raw_inst & 0x00F0) >> 4;
        
        let n: u8 = (raw_inst & 0x000F).try_into().expect("Instruction chunk N should fit into u8");
        let nn: u8 = (raw_inst & 0x00FF).try_into().expect("Instruction chunk NN should fit into u8");
        let nnn = raw_inst & 0x0FFF;

        match opcode {
            0x0000 => match raw_inst {
                0x00E0 => Ok(Instruction::DisplayClear),
                0x00EE => Ok(Instruction::SubReturn),
                _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
            },
            0x1000 => Ok(Instruction::Jump { target: nnn }),
            0x6000 => Ok(Instruction::SetVX { index: x.into(), value: nn }),
            0x7000 => Ok(Instruction::AddVX { index: x.into(), value: nn }),
            0xA000 => Ok(Instruction::SetIR { value: nnn }),
            0xD000 => Ok(Instruction::Draw { x: x.into(), y: y.into(), height: raw_inst & 0x000F }),
            0x2000 => Ok(Instruction::SubCall { target: nnn }),
            0x3000 => Ok(Instruction::SkipEq { vx: x.into(), nn }),
            0x4000 => Ok(Instruction::SkipNotEq { vx: x.into(), nn }),
            0x5000 => Ok(Instruction::SkipVEq { vx: x.into(), vy: y.into() }),
            0x9000 => Ok(Instruction::SkipVNotEq { vx: x.into(), vy: y.into() }),
            0x8000 => match n {
                0x0 => Ok(Instruction::MSetVReg { vx: x.into(), vy: y.into() }),
                0x1 => Ok(Instruction::MSetVRegOr { vx: x.into(), vy: y.into() }),
                0x2 => Ok(Instruction::MSetVRegAnd { vx: x.into(), vy: y.into() }),
                0x3 => Ok(Instruction::MSetVRegXor { vx: x.into(), vy: y.into() }),
                0x4 => Ok(Instruction::MAddWithCarry { vx: x.into(), vy: y.into() }),
                0x5 => Ok(Instruction::MSubWithBorrow { vx: x.into(), vy: y.into() }),
                0x7 => Ok(Instruction::MSubInvWithBorrow { vx: x.into(), vy: y.into() }),
                0x6 => Ok(Instruction::MShiftRight { vx: x.into(), vy: y.into() }),
                0xE => Ok(Instruction::MShiftLeft { vx: x.into(), vy: y.into() }),
                _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
            },
            0xB000 => Ok(Instruction::JumpOffset { vx: x.into(), offset: opcode & 0x0FFF }),
            0xC000 => Ok(Instruction::Random { vx: x.into(), nn }),
            0xE000 => match nn {
                0x9E => Ok(Instruction::SkipIfKey { vx: x.into() }),
                0xA1 => Ok(Instruction::SkipIfNotKey { vx: x.into() }),
                _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
            },
            0xF000 => match nn {
                0x7 => Ok(Instruction::SetVXToDelayTimer { vx: x.into() }),
                0x15 => Ok(Instruction::SetDelayTimerToVX { vx: x.into() }),
                0x18 => Ok(Instruction::SetSoundTimerToVX { vx: x.into() }),
                0x1E => Ok(Instruction::AddVXToIndexRegister { vx: x.into() }),
                0x0A => Ok(Instruction::GetKeyBlock { vx: x.into() }),
                0x29 => Ok(Instruction::FontChar { vx: x.into() }),
                0x33 => Ok(Instruction::BinaryCodedDecimalConversion { vx: x.into() }),
                0x55 => Ok(Instruction::SaveVXToMem { vx: x.into() }),
                0x65 => Ok(Instruction::LoadVXFromMem { vx: x.into() }),
                _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
            }
            _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
        }
    }
    
    pub fn tick(&mut self) -> Result<(), VMError> {
        let instruction = self.fetch().map_err(|err| {
            match err {
                InstructionDecodeError::UnsupportedOpcode { raw_inst: _ } => VMError::UnsupportedInstruction
            }
        })?;

        let mut increment: usize = 2;

        match instruction {
            Instruction::DisplayClear => { self.display.clear(false) },
            Instruction::SubReturn => {
                let target = self.stack.pop().map_err(|err| {
                    match err {
                        crate::chip8::stack::StackError::StackOverflow => VMError::StackOverflow,
                        crate::chip8::stack::StackError::StackUnderflow => VMError::StackUnderflow
                    }
                })?;

                self.program_counter = target as usize;
                increment = 0;
            },
            Instruction::Jump { target } => {
                self.program_counter = target as usize;
                increment = 0;
            },
            Instruction::SetVX { index, value } => self.variable_registers[index] = value,
            Instruction::AddVX { index, value } => self.variable_registers[index] += value,
            Instruction::SetIR { value } => self.index_register = value,
            Instruction::Draw { x, y, height } => todo!("Draw"),
            Instruction::SubCall { target } => {
                self.stack.push((self.program_counter + increment).try_into().unwrap()).map_err(|err|  {
                    match err {
                        crate::chip8::stack::StackError::StackOverflow => VMError::StackOverflow,
                        crate::chip8::stack::StackError::StackUnderflow => VMError::StackUnderflow
                    }
                })?;
                increment = 0;
                self.program_counter = target as usize;
            },
            Instruction::SkipEq { vx, nn } => {
                if self.variable_registers[vx] == nn {
                    increment += 2;
                }
            },
            Instruction::SkipNotEq { vx, nn } => {
                if self.variable_registers[vx] != nn {
                    increment += 2;
                }
            },
            Instruction::SkipVEq { vx, vy } => {
                if self.variable_registers[vx] == self.variable_registers[vy] {
                    increment += 2;
                }
            },
            Instruction::SkipVNotEq { vx, vy } => {
                if self.variable_registers[vx] != self.variable_registers[vy] {
                    increment += 2;
                }
            },
            Instruction::MSetVReg { vx, vy } => self.variable_registers[vx] = self.variable_registers[vy],
            Instruction::MSetVRegOr { vx, vy } => self.variable_registers[vx] = self.variable_registers[vx] | self.variable_registers[vy],
            Instruction::MSetVRegAnd { vx, vy } => self.variable_registers[vx] = self.variable_registers[vx] & self.variable_registers[vy],
            Instruction::MSetVRegXor { vx, vy } => self.variable_registers[vx] = self.variable_registers[vx] ^ self.variable_registers[vy],
            Instruction::MAddWithCarry { vx, vy } => {
                if self.variable_registers[vx] > (u8::MAX - self.variable_registers[vy]) {
                    self.variable_registers[0xF] = 1;
                }else {
                    self.variable_registers[0xF] = 0;
                }

                self.variable_registers[vx] += self.variable_registers[vy];
            },
            Instruction::MSubWithBorrow { vx, vy } => {
                if self.variable_registers[vx] > self.variable_registers[vy] {
                    self.variable_registers[0xF] = 1;
                } else {
                    self.variable_registers[0xF] = 0;
                }
                self.variable_registers[vx] -= self.variable_registers[vy];
            },
            Instruction::MSubInvWithBorrow { vx, vy } => {
                if self.variable_registers[vy] > self.variable_registers[vx] {
                    self.variable_registers[0xF] = 1;
                } else {
                    self.variable_registers[0xF] = 0;
                }

                self.variable_registers[vx] = self.variable_registers[vy] - self.variable_registers[vx];
            },
            Instruction::MShiftRight { vx, vy } => {
                if self.shift_legacy {
                    self.variable_registers[0xF] = self.variable_registers[vx] & 0x01;
                    self.variable_registers[vx] = self.variable_registers[vx] >> 1;
                } else {
                    self.variable_registers[0xF] = self.variable_registers[vx] & 0x01;
                    self.variable_registers[vx] = self.variable_registers[vy] >> 1; 
                }
            },
            Instruction::MShiftLeft { vx, vy } => {
                if self.shift_legacy {
                    self.variable_registers[0xF] = self.variable_registers[vx] & 0x01;
                    self.variable_registers[vx] = self.variable_registers[vx] << 1;
                } else {
                    self.variable_registers[0xF] = self.variable_registers[vx] & 0x01;
                    self.variable_registers[vx] = self.variable_registers[vy] << 1; 
                }
            },
            Instruction::JumpOffset { vx, offset } => {
                if self.chip48_mode {
                    self.program_counter = (self.variable_registers[vx] + ((offset & 0x0FFF) as u8)) as usize;
                } else {
                    self.program_counter = (self.variable_registers[0] + ((offset & 0x0FFF) as u8)) as usize;
                }
                increment = 0;
            },
            Instruction::Random { vx, nn } => todo!("RANDOM"),
            

        }

        self.program_counter += increment;
        debug!("Ins: {:?} PC: {}", instruction, self.program_counter);

        return Ok(());
    }
}

