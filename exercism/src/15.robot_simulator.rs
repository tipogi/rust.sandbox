use self::Direction::*;
use self::Instructions::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub enum Instructions {
    Left,
    Right,
    Advance
}

// Implement trait to convert a char into Instructions enum
impl From<char> for Instructions {
    fn from(origin: char) -> Instructions {
        match origin {
            'L' => Instructions::Left,
            'R' => Instructions::Right,
            'A'   => Instructions::Advance,
            _       => panic!()
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction {
            North => Self { direction: East, ..self },
            East => Self { direction: South, ..self },
            South => Self { direction: West, ..self },
            West => Self { direction: North, ..self }
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self{
        Robot {
            direction: match self.direction {
                North => West,
                East => North,
                South => East,
                West => South
            },
            .. self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            North => Self { y: self.y + 1, ..self },
            West => Self { x: self.x - 1, ..self },
            South => Self { y: self.y - 1, ..self },
            East => Self { x: self.x + 1, ..self }
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // Create a vector of instructions type instead of string (not need it)
        let movements: Vec<Instructions> = instructions
            .chars()
            // Convert all chars into Instruction enum
            .map(|char| char.into())
            .collect();
        
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
        movements.iter().fold(self, |robot, instruc| {
            println!("{:?}, {:?}", robot.position(), robot.direction());
            match instruc {
                Left => robot.turn_left(),
                Right => robot.turn_right(),
                Advance => robot.advance()
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

fn main() {
    let robot = Robot::new(7, 3, Direction::North);
    let moved_robot = robot.instructions("RAALAL");
    println!("{:?}, {:?}", moved_robot.position(), moved_robot.direction());

}