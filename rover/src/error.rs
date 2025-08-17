use thiserror::Error;

/// All errors that can come from processing the rover.
///
/// Some folks criticise rust error handling like this for creating a massive enum, or struct.
/// However, I have found this quite ergonomic, particularly when I can keep the freedom for
/// self-contained/separate errors by implementing them in a separate crate  (see the
/// implementation of another Err enum in `c_rover`).
#[derive(Error, Debug, PartialEq)]
pub enum RoverError {
    #[error("Instructions list must contain pairs of strings")]
    InvalidInput,

    #[error("expected 3 whitespace-separated tokens: <lat> <lon> <dir>, got {got}")]
    TokenCount { got: usize },

    #[error("invalid integer for {which}: '{value}'")]
    InvalidInt { which: &'static str, value: String },

    #[error("direction must be a single character, got '{value}'")]
    DirectionLen { value: String },

    #[error(
        "Start position of rover was out of bounds {which}. Max value in this direction is {max}, whereas the start position was {value}."
    )]
    OutOfBoundsStartPosition {
        which: &'static str,
        max: u16,
        value: u16,
    },

    // Last sentence is not always relevant to this err message
    #[error("Instruction tried to send Rover too far {direction:?}")]
    OutOfBoundsError { direction: String },

    #[error("Input was not a valid Direction. Directions can be N, S, W, E. Input was {received}")]
    DirectionError { received: char },

    #[error(
        "Input was not a valid instruction. Rover instructions can either be L, R, M. Input recevied was {received}"
    )]
    InstructionError { received: char },
}
