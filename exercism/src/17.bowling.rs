#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default, PartialEq)]
pub enum Throw {
    #[default]
    One,
    Two,
    Extra
}

impl Throw {
    fn reverse(&mut self) {
        match self {
            Throw::One => *self = Throw::Two,
            Throw::Two => *self = Throw::One,
            _ => ()
        }
        
    }
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    score: u16,
    extra: u16,
    throw: Throw,
    prev_pins: u16,
    frame: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            throw: Throw::One, 
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let pin_state = self.result(pins);
        if pin_state == Err(Error::NotEnoughPinsLeft) {
            return pin_state;
        }
        match pins {
            10 => {
                // After the last strike, just sum the throws
                if self.frame == 9 {
                    self.extra = 0;
                    self.throw = Throw::Extra
                } else {
                    self.extra = 2
                }
                self.score = self.score + pins;
                self.frame += 1;
            },
            x if x < 11 => {
                self.calculate_points(x);
            },
            _ => ()
        }
        let result = self.score();
        self.result(pins)
    }

    pub fn score(&self) -> Option<u16> {
        // Users finishes the match
        if self.extra == 0 && self.frame == 10 {
            return Some(self.score);
        } else if self.throw == Throw::Extra {
            return Some(self.score);
        } else {
            // Not finished yet the game
            None
        }
    }

    fn calculate_points(&mut self, pins: u16) {
        self.score += pins;
        // Extra points for spare or strike
        if self.extra > 0 {
            self.score += pins;
            self.extra -= 1;
        }
        // Update match
        if self.throw == Throw::One {
            self.prev_pins = pins;
        } else if self.throw == Throw::Two {
            // Player makes spare
            if self.prev_pins + pins == 10 { self.extra += 1 }
            self.frame += 1;
            self.prev_pins = 0;
        }
        self.throw.reverse();
    }

    pub fn result(&self, pins: u16) -> Result<(), Error> {
        let total = self.prev_pins + pins;
        if total > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }
}

fn main() {
    let mut game = BowlingGame::new();
    for _ in 0..18 {
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(3);
    println!("{:?}", game.score())
}