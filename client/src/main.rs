use chrono::NaiveDate;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct NeoDateJson {
    pub near_earth_objects: Vec<NeoDate>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NeoDate {
    pub date: Vec<NeoJson>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NeoJson {
    pub id: String,
    pub designation: String,
    pub estimated_diameter: Diameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<ApproachData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Diameter {
    pub miles: MilesDiameter,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MilesDiameter {
    pub estimated_diameter_min: f32,
    pub estimated_diameter_max: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApproachData {
    pub close_approach_date: String,
    pub relative_velocity: Velocity,
    pub miss_distance: MissedDistance,
    pub orbiting_body: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Velocity {
    pub miles_per_hour: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MissedDistance {
    pub miles: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let begin_date = NaiveDate::parse_from_str("2015-09-07", "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str("2015-09-08", "%Y-%m-%d")?;

    let response = client
        .get("https://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-07&end_date=2015-09-10&api_key=DEMO_KEY")
        .send()
        .await?;

    let body = response.text().await?;
    // println!("{}", body);
    let v: Value = serde_json::from_str(&body)?;
    println!("{:?}", v);
    // for neos in v["near_earth_objects"].as_object().unwrap() {
    //     for neo in neos.1.as_array().unwrap() {
    //         println!("{:?}", neo["close_approach_data"][0]);
    //     }
    // }

    Ok(())
}
