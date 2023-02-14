use serde::{Serialize, Deserialize};

pub enum FuturamaApiEndpoints {
    Info,
    Cast
}

impl FuturamaApiEndpoints {
    pub fn url(&self) -> String {
        match self {
            FuturamaApiEndpoints::Info => String::from("https://api.sampleapis.com/futurama/info"),
            FuturamaApiEndpoints::Cast => String::from("https://api.sampleapis.com/futurama/cast")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuturamaCreators {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FuturamaInfo {
    synopsis: String,
    yearsAired: String,
    creators: Vec<FuturamaCreators>,
    id: usize
}

impl FuturamaInfo {
    pub fn get_creators(&self) -> &Vec<FuturamaCreators> {
        &self.creators
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CastBio {
    text: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuturamaCast {
    id: usize,
    name: String,
    born: String,
    died: String,
    bio: CastBio
}

impl FuturamaCast {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}