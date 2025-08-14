use thiserror::Error;

#[derive(Debug)]
struct Coordinates {
    latitutde: u16,
    longitude: u16,
}

// TODO: add three slash comments for docs

/// Represents each direction
#[derive(Debug)]
enum Direction{
    North,
    South,
    East,
    West,
}

// TODO: implement try_from_str for these
//
/// Represents options inputs for spinning the rover
#[derive(Debug)]
enum Spin {
    Left,
    Right,
}

struct RoverPosition {
    current_location: Coordinates,
    facing: Direction
}
impl RoverPosition{
    fn new(current_location:Coordinates, facing:Direction) ->RoverPosition{
        RoverPosition{
            current_location:current_location,
            facing: facing
        }
    }
}

struct RoverInformation {
    position:RoverPosition,
    top_right_location: Coordinates,
}

#[derive(Error, Debug)]
pub enum RoverError{
    // Last sentence is not always relevant to this err message
    // TODO: Could mkae this into four enums - TooFarNorthError etc.
    #[error("Rover was at location {current_location:?}, facing {current_direction:?}. It tried to move forward. Within the limits of {top_right:?} this is out of bounds.")]
    OutOfBoundsError{
        current_direction:Direction,
        current_location: Coordinates,
        top_right:Coordinates,
    },
}

impl RoverInformation{
    fn new(top_right:Coordinates, position: RoverPosition) -> RoverInformation{
        RoverInformation{position:position, top_right_location: top_right}
    }

    fn spin(mut self,spin: Spin) -> RoverInformation{
        let current_orientation = self.position.facing;
        self.position.facing = match (current_orientation, spin) {
            (Direction::North, Spin::Right) =>  Direction::East,
            (Direction::North, Spin::Left) =>  Direction::West,
            (Direction::East, Spin::Right) =>  Direction::South,
            (Direction::East, Spin::Left) =>  Direction::North,
            (Direction::South, Spin::Right) =>  Direction::East,
            (Direction::South, Spin::Left) =>  Direction::West,
            (Direction::West, Spin::Right) =>  Direction::South,
            (Direction::West, Spin::Left) =>  Direction::North,
        };
        self
    }

    fn try_move_forward(mut self) ->  Result<RoverInformation, RoverError>{
        // There is more repetition than I would like here
        // It would be possible to reduce this with more complicated types
        // But this would not be likely to be less verbose
        //
        //
        // As pseudo-code:
        // - [X] if move North and self.current_position.North == max_coord.north => Out of bounds err
        // - [X] if move North, _ => self.current_position.long +=1
        // - [X] if move South and self.current_position.South == 0 => Out of bounds err
        // - [X] if move South, _ => self.current_position.long -=1
        // - [X] same for East/West (sub East as North, West as South)
        match self.position.facing {
            Direction::North => {
                if self.position.current_location.longitude == self.top_right_location.longitude {
                    Err( RoverError::OutOfBoundsError{current_direction: self.position.facing, current_location:self.position.current_location, top_right:self.top_right_location})
                } else {
                    self.position.current_location.longitude +=1;
                    Ok(self)
                }
            },
            Direction::South=> {
                    if self.position.current_location.longitude == 0{
                        Err( RoverError::OutOfBoundsError{current_direction: self.position.facing, current_location:self.position.current_location, top_right:self.top_right_location})
                    } else {
                        self.position.current_location.longitude -=1;
                        Ok(self)
                    }
            },
            Direction::East=> {
                if self.position.current_location.latitutde== self.top_right_location.latitutde{
                    Err( RoverError::OutOfBoundsError{current_direction: self.position.facing, current_location:self.position.current_location, top_right:self.top_right_location})
                } else {
                    self.position.current_location.latitutde+=1;
                    Ok(self)
                }
            },
            Direction::West=> {
                    if self.position.current_location.latitutde== 0{
                        Err( RoverError::OutOfBoundsError{current_direction: self.position.facing, current_location:self.position.current_location, top_right:self.top_right_location})
                    } else {
                        self.position.current_location.latitutde-=1;
                        Ok(self)
                    }
            },
        }
    }
}

fn return_coordinates(top_right: Coordinates,instructions:Vec<(String, String)>) -> Vec<RoverPosition>{
    todo!()
}

fn main (){}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inp(){
        // TODO: also parse these from a string
        // This would be so that they can be received from GoLang.
        
        let top_right = Coordinates(5,5);
        let instructions = Vec![("122N", "LMLMLMLMM"), ("33E","MMRMMRMRRM")];
        let exp =Vec![RoverPosition(Coordinates(1,3), Direction::North), RoverPosition(Coordinates(5,1), Direction::East)];
        assert_eq!(return_coordinates(top_right, instructions), exp)
    }
    // TODO: add failing tests
}
