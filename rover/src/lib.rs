use std::char;

mod error;

use crate::error::RoverError;

/// Coordinates for the Mars Rover's location
#[derive(Copy, Clone, Debug, PartialEq)]
struct Coordinates {
    latitude: u16,
    longitude: u16,
}
impl TryFrom<String> for Coordinates {
    type Error = RoverError;

    fn try_from(value: String) -> Result<Self, RoverError> {
        let mut it = value.split_whitespace();
        let lat_s = it.next().ok_or(RoverError::TokenCount { got: 0 })?;
        let lon_s = it.next().ok_or(RoverError::TokenCount { got: 1 })?;
        if it.next().is_some() {
            let got = value.split_whitespace().count();
            return Err(RoverError::TokenCount { got });
        }

        let latitude: u16 = lat_s.parse().map_err(|_| RoverError::InvalidInt {
            which: "latitude",
            value: lat_s.to_string(),
        })?;
        let longitude: u16 = lon_s.parse().map_err(|_| RoverError::InvalidInt {
            which: "longitude",
            value: lon_s.to_string(),
        })?;
        Ok(Coordinates {
            latitude,
            longitude,
        })
    }
}

/// Represents each direction that the Rover can be facing
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<Direction> for String {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => "N".to_string(),
            Direction::South => "S".to_string(),
            Direction::West => "W".to_string(),
            Direction::East => "E".to_string(),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = RoverError;

    fn try_from(value: char) -> Result<Self, RoverError> {
        match value {
            'N' => Ok(Direction::North),
            'S' => Ok(Direction::South),
            'E' => Ok(Direction::East),
            'W' => Ok(Direction::West),
            other => Err(RoverError::DirectionError { received: other }),
        }
    }
}

/// Represents inputs for spinning the rover
#[derive(Debug)]
enum Spin {
    Left,
    Right,
}

/// The types of instructions which the rover can receive
enum Instruction {
    Forward,
    Pivot(Spin),
}

impl TryFrom<char> for Instruction {
    type Error = RoverError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Pivot(Spin::Left)),
            'R' => Ok(Instruction::Pivot(Spin::Right)),
            'M' => Ok(Instruction::Forward),
            _ => Err(RoverError::InstructionError { received: value }),
        }
    }
}

/// The position of the Rover at a point in time
#[derive(Debug, PartialEq)]
struct RoverPosition {
    current_location: Coordinates,
    facing: Direction,
}

impl TryFrom<String> for RoverPosition {
    type Error = RoverError;

    /// Parses two digits and a letter: `<lat> <lon> <dir>`, e.g. "0 1 N".
    /// - `lat` and `lon` must be integer digits.
    /// - `dir` must be one of N/S/E/W.
    fn try_from(s: String) -> Result<Self, RoverError> {
        let mut it = s.split_whitespace();
        let lat_s = it.next().ok_or(RoverError::TokenCount { got: 0 })?;
        let lon_s = it.next().ok_or(RoverError::TokenCount { got: 1 })?;
        let dir_s = it.next().ok_or(RoverError::TokenCount { got: 2 })?;
        if it.next().is_some() {
            let got = s.split_whitespace().count();
            return Err(RoverError::TokenCount { got });
        }

        let latitude: u16 = lat_s.parse().map_err(|_| RoverError::InvalidInt {
            which: "latitude",
            value: lat_s.to_string(),
        })?;
        let longitude: u16 = lon_s.parse().map_err(|_| RoverError::InvalidInt {
            which: "longitude",
            value: lon_s.to_string(),
        })?;

        let mut chars = dir_s.chars();
        let dch = chars.next().ok_or_else(|| RoverError::DirectionLen {
            value: dir_s.to_string(),
        })?;
        if chars.next().is_some() {
            return Err(RoverError::DirectionLen {
                value: dir_s.to_string(),
            });
        }
        let facing = Direction::try_from(dch)?;

        Ok(RoverPosition {
            current_location: Coordinates {
                latitude,
                longitude,
            },
            facing,
        })
    }
}

// The position of the rover, and the (immutable, in implementation but not in rust compiler)
// top right position of the Rover
struct RoverInformation {
    position: RoverPosition,
    top_right_location: Coordinates,
}

impl RoverInformation {
    /// Parses coordinates and a position, so this is already quite far in the parsing of the
    /// instruction. Can fail if the start position of the rover is out of bounds. We don't have to
    /// worry about a negative location, because we are parsing the Coordinates as unsigned
    /// integers, which can never be negative.
    fn try_new(
        top_right: Coordinates,
        position: RoverPosition,
    ) -> Result<RoverInformation, RoverError> {
        if position.current_location.longitude > top_right.longitude {
            Err(RoverError::OutOfBoundsStartPosition {
                which: "North",
                value: position.current_location.longitude,
                max: top_right.longitude,
            })
        } else if position.current_location.latitude > top_right.latitude {
            Err(RoverError::OutOfBoundsStartPosition {
                which: "East",
                value: position.current_location.longitude,
                max: top_right.longitude,
            })
        } else {
            Ok(RoverInformation {
                position,
                top_right_location: top_right,
            })
        }
    }

    /// Spins the rover's current location. For example, spin right instruction for a nort-facing
    /// rover mutates the rover to be facing East.
    fn spin(&mut self, spin: Spin) {
        let new_facing = match (&self.position.facing, spin) {
            (&Direction::North, Spin::Right) => Direction::East,
            (&Direction::North, Spin::Left) => Direction::West,
            (&Direction::East, Spin::Right) => Direction::South,
            (&Direction::East, Spin::Left) => Direction::North,
            (&Direction::South, Spin::Right) => Direction::West,
            (&Direction::South, Spin::Left) => Direction::East,
            (&Direction::West, Spin::Right) => Direction::North,
            (&Direction::West, Spin::Left) => Direction::South,
        };
        self.position.facing = new_facing;
    }

    /// Tries to move the rover forward. If the rover will go out of bounds in any direction,
    /// returns an err.
    /// Pseudo-code I wrote for myself when implemnting:
    /// - [X] if move North and self.current_position.North == max_coord.north => Out of bounds err
    /// - [X] if move North, _ => self.current_position.long +=1
    /// - [X] if move South and self.current_position.South == 0 => Out of bounds err
    /// - [X] if move South, _ => self.current_position.long -=1
    /// - [X] same for East/West (sub East as North, West as South)
    fn try_move_forward(&mut self) -> Result<(), RoverError> {
        // There is more repetition than I would like here
        // It would be possible to reduce this with more complicated types
        // But this would not be likely to be less verbose
        match self.position.facing {
            Direction::North => {
                if self.position.current_location.longitude == self.top_right_location.longitude {
                    Err(RoverError::OutOfBoundsError {
                        direction: self.position.facing.into(),
                    })
                } else {
                    self.position.current_location.longitude += 1;
                    Ok(())
                }
            }
            Direction::South => {
                if self.position.current_location.longitude == 0 {
                    Err(RoverError::OutOfBoundsError {
                        direction: self.position.facing.into(),
                    })
                } else {
                    self.position.current_location.longitude -= 1;
                    Ok(())
                }
            }
            Direction::East => {
                if self.position.current_location.latitude == self.top_right_location.latitude {
                    Err(RoverError::OutOfBoundsError {
                        direction: self.position.facing.into(),
                    })
                } else {
                    self.position.current_location.latitude += 1;
                    Ok(())
                }
            }
            Direction::West => {
                if self.position.current_location.latitude == 0 {
                    Err(RoverError::OutOfBoundsError {
                        direction: self.position.facing.into(),
                    })
                } else {
                    self.position.current_location.latitude -= 1;
                    Ok(())
                }
            }
        }
    }

    /// Mutates the rover by implementing the instruction to spin or move forward. Pivot/spins
    /// cannot, fail, but move forwards can.
    fn try_instruction(&mut self, instruction: Instruction) -> Result<(), RoverError> {
        match instruction {
            Instruction::Pivot(spin) => {
                self.spin(spin);
                Ok(())
            }
            Instruction::Forward => self.try_move_forward(),
        }
    }
}

/// Tries to process the rover from the received instructions.
/// The amount of parsing here is relatively minimal - the top right co-ordinates have already been
/// parsed, because they are always the same. However, the remaining start position/instructions
/// have not been parsed so they can fail due to mal-formed inputs.
fn try_process_rover(
    top_right: Coordinates,
    starting_position: String,
    instructions: String,
) -> Result<RoverPosition, RoverError> {
    let starting_position: RoverPosition = starting_position.try_into()?;
    let mut rover_info = RoverInformation::try_new(top_right, starting_position)?;
    let instruction_chars = instructions.chars();
    for char in instruction_chars {
        let instruction: Instruction = char.try_into()?;
        rover_info.try_instruction(instruction)?;
    }
    Ok(rover_info.position)
}

/// Answers the problem exercise instructions - given a top right position and a string of
/// instructions, return the final position of the rover(s).
///
/// Result<Vec<String>, RoverError> rather than Result<Vec<Result<String, RoverError>,RoverError> -
/// both are possible, but failing everything if there are any failures on an individual rover
/// seems fine for this problem. The second type seems quite  complex and more (needlessly) complex
/// to print from the return type.
///
/// Since I have used the go function as the *real* exercise interface, which outputs a single
/// newline string, the success condition goes into a newline joined String output via the go
/// interface. However, this would be  straightforward to refactor to output here instead to be
/// independent from C/Go.
pub fn return_coordinates(
    top_right: String,
    instructions: String,
) -> Result<Vec<String>, RoverError> {
    let mut rovers = Vec::new();
    let top_right_coordinates = top_right.try_into()?;
    let parts: Vec<&str> = instructions.split("\n").collect();
    let pairs = parts.chunks_exact(2);
    if !pairs.remainder().is_empty() {
        return Err(RoverError::InvalidInput);
    }
    for pair in pairs {
        let starting_position = pair[0].to_string();
        let instruction_str = pair[1].to_string();

        let rover_final_position =
            try_process_rover(top_right_coordinates, starting_position, instruction_str)?;
        let direction: String = rover_final_position.facing.into();
        let position_as_string = format!(
            "{latitude} {longitude} {direction}",
            latitude = rover_final_position.current_location.latitude,
            longitude = rover_final_position.current_location.longitude,
        );
        rovers.push(position_as_string);
    }
    Ok(rovers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass() {
        let top_right = "5 5".to_string();
        let instructions = "1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM"
            .to_string();
        let exp = Ok(vec!["1 3 N".to_string(), "5 1 E".to_string()]);
        let res = return_coordinates(top_right, instructions);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_oob_start_position() {
        let top_right = "1 1".to_string();
        let instructions = "20 2 N
MMMLMM"
            .to_string();
        let res = return_coordinates(top_right, instructions);
        assert!(matches!(
            res,
            Err(RoverError::OutOfBoundsStartPosition { .. })
        ));
    }

    #[test]
    fn test_drive_oob() {
        let top_right = "5 5".to_string();
        let instructions = "1 2 N
MMMMMMMMMMMMMM"
            .to_string();

        let res = return_coordinates(top_right, instructions);
        assert!(matches!(res, Err(RoverError::OutOfBoundsError { .. })));
    }

    #[test]
    fn test_non_digit_start_position() {
        let faulty_start_position = "WERF 3 N".to_string();
        let res = RoverPosition::try_from(faulty_start_position);

        dbg!(&res);
        assert!(matches!(res, Err(RoverError::InvalidInt { .. })));
    }

    #[test]
    fn test_incorrect_shape_start_position() {
        let wrong_form_start_position = "13N".to_string();
        let res = RoverPosition::try_from(wrong_form_start_position);
        assert!(matches!(res, Err(RoverError::TokenCount { .. })));
    }

    #[test]
    fn test_faulty_direction() {
        let faulty_start_position = "3 3 X".to_string();
        let res = RoverPosition::try_from(faulty_start_position);

        assert!(matches!(res, Err(RoverError::DirectionError { .. })));
    }
    #[test]
    fn test_faulty_instruction() {
        let top_right = "5 5".to_string();
        let instructions = "2 2 N
MMXLMM"
            .to_string();
        let res = return_coordinates(top_right, instructions);

        dbg!(&res);
        assert!(matches!(res, Err(RoverError::InstructionError { .. })));
    }
}
