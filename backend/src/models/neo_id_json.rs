use serde::Deserialize;

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
