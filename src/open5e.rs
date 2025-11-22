use reqwest::Error;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.open5e.com";

// Struct for the Open5e API response containing a list of results
#[derive(Serialize, Deserialize, Debug)]
pub struct Open5eResponse<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

// Struct for a D&D Class
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub slug: String,
    pub name: String,
}

// Struct for a D&D Spell
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spell {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub level: String,
    pub school: String,
    pub dnd_class: String,
}

pub async fn get_class_by_slug(slug: &str) -> Result<Option<Class>, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/classes/?slug={}", BASE_URL, slug.to_lowercase());
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Open5eResponse<Class>>()
        .await?;

    if response.results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(response.results[0].clone()))
    }
}

pub async fn get_spell_by_slug(slug: &str) -> Result<Option<Spell>, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/spells/?slug={}", BASE_URL, slug.to_lowercase());
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Open5eResponse<Spell>>()
        .await?;

    if response.results.is_empty() {
        Ok(None)
    } else {
        Ok(Some(response.results[0].clone()))
    }
}
