use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, api_id: {}, designation: {}, diameter_min: {}, diameter_max: {}, hazardous_asteroid: {}, approach_date: {}, velocity: {}, miss_distance: {}, orbiting_body: {}",
    id,
    api_id,
    designation,
    diameter_min,
    diameter_max,
    hazardous_asteroid,
    approach_date,
    velocity,
    miss_distance,
    orbiting_body
)]
pub struct Neo {
    pub id: NeoId,
    pub api_id: i32,
    pub designation: String,
    pub diameter_min: f32,
    pub diameter_max: f32,
    pub hazardous_asteroid: bool,
    pub approach_date: String,
    pub velocity: f32,
    pub miss_distance: f32,
    pub orbiting_body: String,
}

impl Neo {
    #[allow(dead_code)]
    pub fn new(
        id: NeoId,
        api_id: i32,
        designation: String,
        diameter_min: f32,
        diameter_max: f32,
        hazardous_asteroid: bool,
        approach_date: String,
        velocity: f32,
        miss_distance: f32,
        orbiting_body: String,
    ) -> Self {
        Neo {
            id,
            api_id,
            designation,
            diameter_min,
            diameter_max,
            hazardous_asteroid,
            approach_date,
            velocity,
            miss_distance,
            orbiting_body,
        }
    }
}

impl From<i32> for NeoId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<NeoId> for i32 {
    fn from(value: NeoId) -> Self {
        value.0
    }
}

pub trait IntoNeoId {
    fn into_neo_id(self) -> NeoId;
}

impl IntoNeoId for i32 {
    fn into_neo_id(self) -> NeoId {
        NeoId::from(self)
    }
}

impl IntoNeoId for NeoId {
    fn into_neo_id(self) -> NeoId {
        self
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    sqlx::Type,
    Display,
    derive_more::Deref,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct NeoId(pub i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNeo {
    pub api_id: i32,
    pub designation: String,
    pub diameter_min: f32,
    pub diameter_max: f32,
    pub hazardous_asteroid: bool,
    pub approach_date: String,
    pub velocity: f32,
    pub miss_distance: f32,
    pub orbiting_body: String,
}

//make_db_id!(NeoId);

#[derive(Deserialize)]
pub struct GetNeoById {
    pub neo_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateRange {
    pub begin_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDateRange {
    pub begin_date: String,
    pub end_date: String,
}
