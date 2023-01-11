// URL: https://exercism.org/tracks/rust/exercises/tournament
use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Debug, Eq, PartialEq)]
pub enum MatchResult {
    Win,
    Loss,
    Draw
}

// When we use into(), that From trait will do the conversion to 
// MatchResult type
impl From<&str> for MatchResult {
    fn from(origin: &str) -> MatchResult {
        match origin {
            "draw" => MatchResult::Draw,
            "loss" => MatchResult::Loss,
            "win"   => MatchResult::Win,
            _       => panic!()
        }
    }
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Win => MatchResult::Loss,
            _ => MatchResult::Draw
        }
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
struct Team {
    matches_played: u32,
    win: u8,
    draw: u8,
    loss: u8,
    points: u8,
    name: String
}


impl Team {
    fn new(name: String) -> Self{
        Self {
            name,
            ..Default::default()
        }
    }

    fn add_match(&mut self, score: &MatchResult) {
        match score {
            MatchResult::Draw => self.draw(),
            MatchResult::Loss => self.loss(),
            MatchResult::Win => self.win()
        }
    }

    fn win(&mut self) {
        self.win += 1;
        self.matches_played += 1;
        self.points += 3;
    }

    fn draw(&mut self) {
        self.draw += 1;
        self.matches_played += 1;
        self.points += 1;
    }

    fn loss(&mut self) {
        self.loss += 1;
        self.matches_played += 1;
    }
}

impl From<&Team> for String {
    fn from(origin: &Team) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            origin.name, origin.matches_played, origin.win, origin.draw, origin.loss, origin.points
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut scores:HashMap<String, Team> = HashMap::new();
    // Loop line by line all the matches
    match_results.lines().for_each(|line| {
        // Get team names and score
        let event: Vec<&str> = line.split(';').collect();
        // Extract the info
        let home = event[0].trim();
        let away = event[1].trim();
        // Does conversion becuase we implement From trait
        // https://doc.rust-lang.org/std/convert/trait.From.html
        // https://doc.rust-lang.org/std/convert/trait.Into.html
        let result = event[2].into();
        
        scores
            .entry(home.to_string())
            .or_insert(Team::new(home.to_string()))
            .add_match(&result);

        scores
            .entry(away.to_string())
            .or_insert(Team::new(away.to_string()))
            .add_match(&result.reverse())
    });
    // Print the hashmap
    println!("{:#?}", scores);
    // Get the hashmap values and collect in a vector
    let mut score_values: Vec<&Team> = scores.values().collect();
    // Order by the team that has bigger points
    score_values
        .sort_by(|a, b| b.points.cmp(&a.points).then_with(|| a.name.cmp(&b.name)));
    // Print the results
    vec![String::from(HEADER)]
        .into_iter()
        .chain(score_values.into_iter().map(|t| t.into()))
        .collect::<Vec<String>>()
        .join("\n")
    
}

fn main() {
    let matches = "Allegoric Alaskans;Blithering Badgers;draw
    Allegoric Alaskans;Blithering Badgers;win";
    let seasson = tally(matches);
    println!("{}", seasson);
    println!("{:?}", MatchResult::Win == "win".into())
}