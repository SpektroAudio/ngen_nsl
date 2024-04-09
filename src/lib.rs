
#![allow(dead_code)]
use log::{info, debug};

/*

ngen_nsl - Rust Library for encoding / decoding NSL scripts for NGEN

Developed by @Spektro Audio

*/

/// Clip a value between a minimum and maximum value
fn clip_u8(value: u8, min: u8, max: u8) -> u8 {
    if value < min {
        min
    } else if value > max {
        debug!("Clipping value {} to {}", value, max);
        max
    } else {
        value
    }
}


/// DataValue is a value that can be either a direct number or a index to a value in the Memory Buffer
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(u8),
    Buffer(u8)
}

impl DataValue {

    /// Decodes a u8 value into a DataValue
    pub fn from_u8(value: u8) -> DataValue {
        let dv =  if value > 127 {
                    DataValue::Buffer(value - 0x80)
                } else {
                    DataValue::Number(value)
                };
        // info!("{} > {:?}", value, dv);
        dv
    }

    /// Encodes the DataValue into a u8 value
    pub fn code(&self) -> u8 {
        match self {
            DataValue::Number(x) =>  {
                let mut value = *x;
                if value > 127 {
                    value = 127;
                }
                0x00 + value
            },
            DataValue::Buffer(x) => {
                x + 0x80
            }
        }
    }

    /// Clips the value between a minimum and maximum value according to the type of DataValue
    pub fn clip(&mut self, min: u8, max: u8) {
        match self {
            DataValue::Number(x) => *x = clip_u8(*x, min, max),
            DataValue::Buffer(x) => *x = clip_u8(*x, min, max)
        }
    }
}


/// DataSource is a source of data that can be used in the NSL script as a parameter for commands.
#[derive(Debug, Clone)]
pub enum DataSource {
    /// A constant value (0 - 127)
    Constant(DataValue),
    /// A random value (0 - 127)
    Random(DataValue),
    /// A step value from the pitch sequence (0 - 31)
    StepPitch(DataValue),
    /// A step value from the velocity sequence (0 - 31)
    StepVelocity(DataValue),
    /// A step value from the length sequence (0 - 31)
    StepLength(DataValue),
    /// A step value from the density sequence (0 - 31)
    StepDensity(DataValue),
    /// A value from the memory buffer (0 - 31)
    MemoryBuffer(DataValue),
    /// A value from the Params 1 - 4 (zero indexed / 0 - 3)
    Params(DataValue),
    /// A value from the Scale (0 - 127)
    Scale(DataValue),
    /// A value from the Full Scale (0 - 127)
    FullScale(DataValue),
    /// A random note value (0 - 100)
    RandomNote(DataValue)
}

impl DataSource {
    /// Encodes the DataSource into a Vec<u8> value
    pub fn code(&mut self) -> Vec<u8> {
        debug!("Converting Data Source: {:?}", self);
        self.validate();
        let code: Vec<u8> = 
        match self {
            DataSource::Constant(x) => vec![0x00, x.code()],
            DataSource::Random(x) => vec![0x01, x.code()],
            DataSource::StepPitch(x) => vec![0x02, x.code()],
            DataSource::StepVelocity(x) => vec![0x03, x.code()],
            DataSource::StepLength(x) => vec![0x04, x.code()],
            DataSource::StepDensity(x) => vec![0x05, x.code()],
            DataSource::MemoryBuffer(x) => vec![0x06, x.code()],
            DataSource::Params(x) => vec![0x07, x.code()],
            DataSource::Scale(x) => vec![0x08, x.code()],
            DataSource::FullScale(x) => vec![0x09, x.code()],
            DataSource::RandomNote(x) => vec![0x0A, x.code()]

        };
        debug!("> Converted Data Source to Vec<u8>: {:?} > {:?}", self, code);
        code
    }

    /// Decodes a Vec<u8> value into a DataSource
    pub fn from_u8_vec(data: Vec<u8>) -> DataSource {
        debug!("Converting u8 data to DataSource: {:?}", data);
        debug!("> Data size: {:?}", data.len());
        let value = DataValue::from_u8(data[1]);
        let ds = match data[0] {
            0x00 => DataSource::Constant(value),
            0x01 => DataSource::Random(value),
            0x02 => DataSource::StepPitch(value),
            0x03 => DataSource::StepVelocity(value),
            0x04 => DataSource::StepLength(value),
            0x05 => DataSource::StepDensity(value),
            0x06 => DataSource::MemoryBuffer(value),
            0x07 => DataSource::Params(value),
            0x08 => DataSource::Scale(value),
            0x09 => DataSource::FullScale(value),
            0x0A => DataSource::RandomNote(value),
            _ => DataSource::Constant(DataValue::from_u8(0))
        };
        debug!("> Converted Vec<u8> to DataSource: {:?} > {:?}", data, ds);
        ds
    }

    /// Returns the maximum value for the DataSource
    pub fn max(&self) -> u8 {
        match self {
            DataSource::Constant(_) => 127,
            DataSource::Random(_) => 127,
            DataSource::StepPitch(_) => 31,
            DataSource::StepVelocity(_) => 31,
            DataSource::StepLength(_) => 31,
            DataSource::StepDensity(_) => 31,
            DataSource::MemoryBuffer(_) => 31,
            DataSource::Params(_) => 3,
            DataSource::Scale(_) => 127,
            DataSource::FullScale(_) => 127,
            DataSource::RandomNote(_) => 100
        }
    }

    /// Clips the value between a minimum and maximum value according to the type of DataSource
    pub fn validate(&mut self) {
        let max = self.max();
        match self {
            DataSource::Constant(x) => x.clip(0, max),
            DataSource::Random(x) => x.clip(0, max),
            DataSource::StepPitch(x) => x.clip(0, max),
            DataSource::StepVelocity(x) => x.clip(0, max),
            DataSource::StepLength(x) => x.clip(0, max),
            DataSource::StepDensity(x) => x.clip(0, max),
            DataSource::MemoryBuffer(x) => x.clip(0, max),
            DataSource::Params(x) => x.clip(0, max),
            DataSource::Scale(x) => x.clip(0, max),
            DataSource::FullScale(x) => x.clip(0, max),
            DataSource::RandomNote(x) => x.clip(0, max),
        }
    }

    

}

macro_rules! data_source_fn {
    ($name:ident, $variant:ident) => {
        /// Quick method for creating a DataSource from a u8 value
        pub fn $name(value: u8) -> DataSource {
            DataSource::$variant(DataValue::from_u8(value))
        }
    };
}

data_source_fn!(constant, Constant);
data_source_fn!(random, Random);
data_source_fn!(step_pitch, StepPitch);
data_source_fn!(step_velocity, StepVelocity);
data_source_fn!(step_length, StepLength);
data_source_fn!(step_density, StepDensity);
data_source_fn!(memory_buffer, MemoryBuffer);



#[derive(Debug, Clone)]
pub struct Int16 {
    value_1: u8,
    value_2: u8
}

impl Int16 {
    pub fn new(value_1: u8, value_2: u8) -> Int16 {
        Int16 {
            value_1,
            value_2
        }
    }

    pub fn get_value(&self) -> u16 {
        (self.value_1 as u16) << 8 | self.value_2 as u16
    }

    pub fn code(&self) -> Vec<u8> {
        vec![self.value_1, self.value_2]
    }

    pub fn from_u8_vec(data: Vec<u8>) -> Int16 {
        Int16::new(data[0], data[1])
    }
    
}

/// NSL Commands are the instructions that the NSL script will execute
#[derive(Debug, Clone)]
pub enum Commands {
    None,
    /// Sets x to y
    Set(DataSource, DataSource),
    /// Copies x to y
    Copy(DataSource, DataSource),
    /// Adds y to x
    Add(DataSource, DataSource),
    /// Subtracts y from x
    Subtract(DataSource, DataSource),
    /// Multiplies x by y
    Multiply(DataSource, DataSource),
    /// Divides x by y
    Divide(DataSource, DataSource),
    /// Sets the loop to x repetitions
    LoopSet(DataSource),
    /// Ends the loop
    LoopEnd,
    /// Jumps to x
    Jump(Int16),
    /// Clears the active track
    ClearTrack,
    /// Clears the memory buffer
    ClearMemory,
    /// Clears all tracks and memory
    ClearAll,
    /// Selects a track
    SelectTrack,
    /// Quantizes all steps of the active track's pitch sequence
    QuantizePitch,
    /// Generates a chord progression 
    GenerateProgression,
    /// Generates a velocity sequence to the active track usign the Euclidean algorithm
    GenerateEuclidean(DataSource, DataSource),
    /// Sets up a conditional statement for equality (x == y)
    CondE(DataSource, DataSource),
    /// Sets up a conditional statement for inequality (x != y)
    CondNE(DataSource, DataSource),
    /// Sets up a conditional statement for greater than (x > y)
    CondGT(DataSource, DataSource),
    /// Sets up a conditional statement for less than (x < y)
    CondLT(DataSource, DataSource),
    /// Sets up a conditional statement for greater than or equal to (x >= y)
    CondGTE(DataSource, DataSource),
    /// Sets up a conditional statement for less than or equal to (x <= y)
    CondLTE(DataSource, DataSource),
    /// Ends a conditional statement
    CondEnd,
    /// Ends the script
    End,
}

impl Commands {
    /// Creates a new command
    pub fn new() -> Commands {
        Commands::End
    }

    /// Returns the length of the command in bytes
    pub fn len(&self) -> usize {
        match self {
            Commands::Set(_, _) => 5,
            Commands::Copy(_, _) => 5,
            Commands::Add(_, _) => 5,
            Commands::Subtract(_, _) => 5,
            Commands::Multiply(_, _) => 5,
            Commands::Divide(_, _) => 5,
            Commands::GenerateEuclidean(_, _) => 5,
            Commands::CondE(_, _) => 5,
            Commands::CondNE(_, _) => 5,
            Commands::CondGT(_, _) => 5,
            Commands::CondLT(_, _) => 5,
            Commands::CondGTE(_, _) => 5,
            Commands::CondLTE(_, _) => 5,
            Commands::LoopSet(_) => 3,
            Commands::Jump(_) => 3,
            _ => 1
        }
    }

    /// Returns the command hex code
    pub fn cmd_code(&self) -> u8 {
        match self {
            Commands::Set(_, _) => 0xA1,
            Commands::Copy(_, _) => 0xA2,
            Commands::ClearTrack => 0xA3,
            Commands::ClearMemory => 0xA4,
            Commands::ClearAll => 0xA5,
            Commands::Add(_, _) => 0xB0,
            Commands::Subtract(_, _) => 0xB1,
            Commands::Multiply(_, _) => 0xB2,
            Commands::Divide(_, _) => 0xB3,
            Commands::QuantizePitch => 0xB4,
            Commands::GenerateProgression => 0xB5,
            Commands::GenerateEuclidean(_, _) => 0xB6,
            Commands::CondE(_, _) => 0xD0,
            Commands::CondNE(_, _) => 0xD1,
            Commands::CondGT(_, _) => 0xD2,
            Commands::CondLT(_, _) => 0xD3,
            Commands::CondGTE(_, _) => 0xD4,
            Commands::CondLTE(_, _) => 0xD5,
            Commands::CondEnd => 0xD6,
            Commands::LoopSet(_) => 0xC0,
            Commands::LoopEnd => 0xC1,
            Commands::Jump(_) => 0xC2,
            Commands::End => 0xFF,
            _ => 0x00
        }
    }

    /// Encodes the command into a Vec<u8> value
    pub fn code(&mut self) -> Vec<u8> {
        info!("Converting command to Vec<u8>: {:?}", self);
        let mut code: Vec<u8> = vec![self.cmd_code()];
        match self {
            Commands::Set(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::Copy(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::Add(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::Subtract(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::Multiply(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::Divide(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::LoopSet(x) => {
                code.extend(x.code());
            },
            Commands::Jump(x) => {
                code.extend(x.code());
            },
            Commands::GenerateEuclidean(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondE(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondNE(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondGT(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondLT(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondGTE(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            Commands::CondLTE(x, y) => {
                code.extend(x.code());
                code.extend(y.code());
            },
            _ => {
                debug!("No additional data");
            }
        }
        debug!("> Converted command to Vec<u8>: {:?} > {:?}", self, code);
        code
    }

    /// Decodes a u8 value into a Command
    pub fn from_u8(data: u8) -> Commands {
        match data {
            0xA1 => Commands::Set(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xA2 => Commands::Copy(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xA3 => Commands::ClearTrack,
            0xA4 => Commands::ClearMemory,
            0xA5 => Commands::ClearAll,
            0xB0 => Commands::Add(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xB1 => Commands::Subtract(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xB2 => Commands::Multiply(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xB3 => Commands::Divide(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xB4 => Commands::QuantizePitch,
            0xB5 => Commands::GenerateProgression,
            0xB6 => Commands::GenerateEuclidean(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xC0 => Commands::LoopSet(DataSource::Constant(DataValue::from_u8(0))),
            0xC1 => Commands::LoopEnd,
            0xC2 => Commands::Jump(Int16::new(0, 0)),
            0xD0 => Commands::CondE(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD1 => Commands::CondNE(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD2 => Commands::CondGT(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD3 => Commands::CondLT(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD4 => Commands::CondGTE(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD5 => Commands::CondLTE(DataSource::Constant(DataValue::from_u8(0)), DataSource::Constant(DataValue::from_u8(0))),
            0xD6 => Commands::CondEnd,
            0xFF => Commands::End,
            _ => Commands::None
        }
    }

    /// Decodes a Vec<u8> value into a Command
    pub fn from_u8_vec(data: Vec<u8>) -> Commands {
        let mut cmd = Commands::from_u8(data[0]);
        cmd = match data.len() {
            5 => {
                let x = DataSource::from_u8_vec(data[1..3].to_vec());
                let y = DataSource::from_u8_vec(data[3..5].to_vec());
                match cmd {
                    Commands::Set(_, _) => Commands::Set(x, y),
                    Commands::Copy(_, _) => Commands::Copy(x, y),
                    Commands::Add(_, _) => Commands::Add(x, y),
                    Commands::Subtract(_, _) => Commands::Subtract(x, y),
                    Commands::Multiply(_, _) => Commands::Multiply(x, y),
                    Commands::Divide(_, _) => Commands::Divide(x, y),
                    Commands::CondE(_, _) => Commands::CondE(x, y),
                    Commands::CondNE(_, _) => Commands::CondNE(x, y),
                    Commands::CondGT(_, _) => Commands::CondGT(x, y),
                    Commands::CondLT(_, _) => Commands::CondLT(x, y),
                    Commands::CondGTE(_, _) => Commands::CondGTE(x, y),
                    Commands::CondLTE(_, _) => Commands::CondLTE(x, y),
                    Commands::GenerateEuclidean(_, _) => Commands::GenerateEuclidean(x, y),
                    _ => Commands::None
                }
            },
            3 => {
                let x = DataSource::from_u8_vec(data[1..3].to_vec());
                match cmd {
                    Commands::LoopSet(_) => Commands::LoopSet(x),
                    _ => Commands::None
                }
            },
            _ => {
                cmd
            }
        };
        debug!("Converted u8 to command: {:?} > {:?}", data, cmd);
        cmd
    }
    


}

/// NSLScript is the main structure used for creating and manipulating NSL scripts.
/// It can encode and decode NSL scripts into a Vec<u8> value.
#[derive(Debug, Clone)]
pub struct NSLScript {
    pub commands: Vec<Commands>
}

impl NSLScript {
    pub fn new() -> NSLScript {
        NSLScript {
            commands: Vec::new()
        }
    }

    pub fn add_command(&mut self, command: Commands) {
        self.commands.push(command);
    }

    pub fn add_commands(&mut self, commands: Vec<Commands>) {
        for command in commands {
            self.commands.push(command);
        }
    }

    pub fn code(&mut self) -> Vec<u8> {
        // Add the NSL header
        let mut code: Vec<u8> = vec![0x4E, 0x53, 0x4C, 0x01];
        for command in &mut self.commands {
            let cmd_code = command.code();
            debug!("Command: {:?} > {:?}", command, cmd_code);
            code.extend(cmd_code);

            debug!("Code: {:?}", code);
        }
        code
    }

    pub fn from_u8_vec(data: Vec<u8>) -> Option<NSLScript> {
        info!("Converting u8 data to NSLScript");
        info!("Data size: {:?}", data.len());
        let mut cmds: Vec<Commands> = Vec::new();
        let mut i = 0;
        // Match first 3 characters to "NSL"
        if data[0] != 0x4E || data[1] != 0x53 || data[2] != 0x4C {
            return None;
        }
        i += 4;
        while i < data.len() {
            debug!("----------------");
            debug!("Index: {}", i);
            let mut cmd = Commands::from_u8(data[i]);
            let len = cmd.len();
            debug!("Matching command {:#04x}: {:?} (Len: {})", data[i], cmd, len);
            let cmd_data = data[i..i+len].to_vec();
            cmd = Commands::from_u8_vec(cmd_data);
            info!("Converted step {}: {:?}", i, cmd);
            cmds.push(cmd);
            i += len;
        }
        Some(NSLScript {
            commands: cmds
        })
    }

    pub fn get_info(&self) {
        for command in &self.commands {
            info!(">> {:?}", command);
        }
    }

    pub fn import_hex(path: &str) -> Option<NSLScript> {
        let data = std::fs::read(path).unwrap();
        NSLScript::from_u8_vec(data)
    }

    pub fn import_hex_as_vec(path: &str) -> Vec<u8> {
        std::fs::read(path).unwrap()
    }

    pub fn export_hex(&mut self, path: &str) {
        let code = self.code();
        std::fs::write(path, code).unwrap();
    }

}

// Implement a simple test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        let test_script: Vec<u8> = vec![0x4E, 0x53, 0x4C, 0x01, 0xA1, 0x06, 0x00, 0x07, 0x00, 0xB3, 0x06, 0x00, 0x00, 0x03, 0xA1, 0x06, 0x01, 0x07, 0x01, 0xB3, 0x06, 0x01, 0x00, 0x03, 0xB6, 0x06, 0x00, 0x06, 0x01, 0xA1, 0x06, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x20, 0xD2, 0x03, 0x80, 0x00, 0x00, 0xA1, 0x06, 0x01, 0x0A, 0x28, 0xA1, 0x02, 0x80, 0x08, 0x81, 0xB0, 0x02, 0x80, 0x00, 0x30, 0xA1, 0x04, 0x80, 0x00, 0x01, 0xA1, 0x05, 0x80, 0x01, 0x31, 0xD6, 0xB0, 0x06, 0x00, 0x00, 0x01, 0xC1];
        let mut script = NSLScript::from_u8_vec(test_script.clone()).unwrap();
        let code = script.code();

        assert_eq!(test_script, code, "Verifying code... File: {} / Generated {}", test_script.len(), code.len());

    }

}