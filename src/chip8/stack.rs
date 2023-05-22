pub const STACK_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub enum StackError {
    StackOverflow,
    StackUnderflow
}

pub struct Stack {
    pub data: [u16; STACK_SIZE],
    pub top: i32
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: [0; STACK_SIZE],
            top: -1 
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        
        if self.top < (STACK_SIZE - 1).try_into().unwrap() {
            self.top += 1;
            self.data[self.top as usize] = value;

            return Ok(())
        } else {
            return Err(StackError::StackOverflow)
        }

    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.top >= 0 {

            let r = self.data[self.top as usize];
            self.top -= 1;
            
            return Ok(r);

        } else {
            return Err(StackError::StackUnderflow);
        }
    }
}