use axum::Json;
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};

use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::error::AppError;
use crate::models::neo::{IntoNeoId, Neo, NeoId};
use crate::models::neo_id_json::NeoJson;
use crate::models::page::PagePackageNeo;
use crate::models::user::{User, UserSignup};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self { conn_pool: pool }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    pub async fn check_admin(&self, email: String) -> Result<bool, sqlx::Error> {
        let row = sqlx::query!("SELECT * FROM users WHERE email = $1", 
            email)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(row.admin)
    }

    pub async fn check_banned(&self, email: String) -> Result<bool, sqlx::Error> {
        let row = sqlx::query!("SELECT * FROM users WHERE email = $1",
            email)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(row.banned)
    }

    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT email, password FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.conn_pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        // TODO: Encrypt/bcrypt user passwords
        let result = sqlx::query("INSERT INTO users(email, password) values ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }

    pub async fn get_all_neos(&self) -> Result<Vec<Neo>, AppError> {
        let neo_rows = sqlx::query!("SELECT * from neos",)
            .fetch_all(&self.conn_pool)
            .await?;

        let neos: Vec<_> = neo_rows
            .into_iter()
            .map(|row| {
                Neo {
                    id: row.id.into(), // Assuming you have a From<u32> for NeoId
                    api_id: row.api_id,
                    designation: row.designation,
                    diameter_min: row.diameter_min,
                    diameter_max: row.diameter_max,
                    hazardous_asteroid: row.is_potentially_hazardous_asteroid,
                    approach_date: row.close_approach_date.to_string(),
                    velocity: row.relative_velocity,
                    miss_distance: row.miss_distance,
                    orbiting_body: row.orbiting_body,
                }
            })
            .collect();

        Ok(neos)
    }

    pub async fn add_neo(
        &mut self,
        api_id: i32,
        designation: String,
        diameter_min: f32,
        diameter_max: f32,
        hazardous_asteroid: bool,
        approach_date: String,
        velocity: f32,
        miss_distance: f32,
        orbiting_body: String,
    ) -> Result<Neo, AppError> {
        let date = NaiveDate::parse_from_str(&approach_date, "%Y-%m-%d")?;
        let res = sqlx::query!(
            r#"INSERT INTO "neos"(api_id, designation, diameter_min, diameter_max, is_potentially_hazardous_asteroid, close_approach_date, relative_velocity, miss_distance, orbiting_body)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING *
        "#,
            api_id,
            designation,
            diameter_min,
            diameter_max,
            hazardous_asteroid,
            date,
            velocity,
            miss_distance,
            orbiting_body,
        )
        .fetch_one(&self.conn_pool)
        .await?;

        let neo = Neo {
            id: NeoId(res.id),
            api_id: res.api_id,
            designation: res.designation,
            diameter_min: res.diameter_min,
            diameter_max: res.diameter_max,
            hazardous_asteroid: res.is_potentially_hazardous_asteroid,
            approach_date: res.close_approach_date.to_string(),
            velocity: res.relative_velocity,
            miss_distance: res.miss_distance,
            orbiting_body: res.orbiting_body,
        };

        Ok(neo)
    }

    pub async fn get_all_neo_pages(&self) -> Result<PagePackageNeo, AppError> {
        let neo_rows = sqlx::query!("SELECT * from neos",)
            .fetch_all(&self.conn_pool)
            .await?;

        let neos: Vec<_> = neo_rows
            .into_iter()
            .map(|row| {
                Neo {
                    id: row.id.into(), // Assuming you have a From<u32> for NeoId
                    api_id: row.api_id,
                    designation: row.designation,
                    diameter_min: row.diameter_min,
                    diameter_max: row.diameter_max,
                    hazardous_asteroid: row.is_potentially_hazardous_asteroid,
                    approach_date: row.close_approach_date.to_string(),
                    velocity: row.relative_velocity,
                    miss_distance: row.miss_distance,
                    orbiting_body: row.orbiting_body,
                }
            })
            .collect();

        let package = PagePackageNeo { neos: neos };

        Ok(package)
    }

    pub async fn get_neo_by_date(
        &mut self,
        begin: String,
        end: String,
    ) -> Result<Vec<Neo>, AppError> {
        let begin_date = NaiveDate::parse_from_str(&begin, "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")?;
        let neo_rows = sqlx::query!(
            "SELECT * from neos WHERE close_approach_date >= $1 AND close_approach_date <= $2",
            begin_date,
            end_date,
        )
        .fetch_all(&self.conn_pool)
        .await?;

        if neo_rows.len() > 0 {
            let neos: Vec<_> = neo_rows
                .into_iter()
                .map(|row| {
                    Neo {
                        id: row.id.into(), // Assuming you have a From<u32> for NeoId
                        api_id: row.api_id,
                        designation: row.designation,
                        diameter_min: row.diameter_min,
                        diameter_max: row.diameter_max,
                        hazardous_asteroid: row.is_potentially_hazardous_asteroid,
                        approach_date: row.close_approach_date.to_string(),
                        velocity: row.relative_velocity,
                        miss_distance: row.miss_distance,
                        orbiting_body: row.orbiting_body,
                    }
                })
                .collect();

            Ok(neos)
        } else {
            let api_key = std::env::var("API_KEY").unwrap();
            let url = format!(
                "https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}",
                begin, end, api_key
            );
            let res = reqwest::get(url).await?.text().await?;
            let res_json: Value = serde_json::from_str(&res)?;

            let mut neos_list: Vec<Neo> = Vec::new();

            for neos in res_json["near_earth_objects"].as_object().unwrap() {
                for neo_date in neos.1.as_array().unwrap() {
                    for approach_data in neo_date["close_approach_data"].as_array().unwrap() {
                        let neo = Neo {
                            id: NeoId(0),
                            api_id: neo_date["id"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", "")
                                .parse()
                                .unwrap(),
                            designation: neo_date["name"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", ""),
                            diameter_min: neo_date["estimated_diameter"]["miles"]
                                ["estimated_diameter_min"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", "")
                                .parse()
                                .unwrap(),
                            diameter_max: neo_date["estimated_diameter"]["miles"]
                                ["estimated_diameter_max"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", "")
                                .parse()
                                .unwrap(),
                            hazardous_asteroid: neo_date["is_potentially_hazardous_asteroid"]
                                .as_bool()
                                .unwrap(),
                            approach_date: approach_data["close_approach_date"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", ""),
                            velocity: approach_data["relative_velocity"]["miles_per_hour"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", "")
                                .parse()
                                .unwrap(),
                            miss_distance: approach_data["miss_distance"]["miles"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", "")
                                .parse()
                                .unwrap(),
                            orbiting_body: approach_data["orbiting_body"]
                                .to_string()
                                .replace("\\", "")
                                .replace("\"", ""),
                        };

                        self.add_neo(
                            neo.api_id.clone(),
                            neo.designation.clone(),
                            neo.diameter_min.clone(),
                            neo.diameter_max.clone(),
                            neo.hazardous_asteroid.clone(),
                            neo.approach_date.clone(),
                            neo.velocity.clone(),
                            neo.miss_distance.clone(),
                            neo.orbiting_body.clone(),
                        )
                        .await?;

                        neos_list.push(neo);
                    }
                }
            }

            Ok(neos_list)
        }
    }

    pub async fn get_neo_by_id<T: IntoNeoId>(&mut self, id: T) -> Result<Vec<Neo>, AppError> {
        let id = id.into_neo_id();

        let neo_row = sqlx::query!(
            r#"
        SELECT * FROM neos WHERE api_id = $1
        "#,
            id.0,
        )
        .fetch_all(&self.conn_pool)
        .await?;

        if neo_row.len() > 0 {
            let neos: Vec<_> = neo_row
                .into_iter()
                .map(|row| {
                    Neo {
                        id: row.id.into(), // Assuming you have a From<u32> for NeoId
                        api_id: row.api_id,
                        designation: row.designation,
                        diameter_min: row.diameter_min,
                        diameter_max: row.diameter_max,
                        hazardous_asteroid: row.is_potentially_hazardous_asteroid,
                        approach_date: row.close_approach_date.to_string(),
                        velocity: row.relative_velocity,
                        miss_distance: row.miss_distance,
                        orbiting_body: row.orbiting_body,
                    }
                })
                .collect();

            Ok(neos)
        } else {
            let api_key = std::env::var("API_KEY").unwrap();
            let url = format!(
                "https://api.nasa.gov/neo/rest/v1/neo/{}?api_key={}",
                id.0, api_key
            );
            let res = reqwest::get(url).await?.text().await?;
            let res_json: NeoJson = serde_json::from_str(&res)?;

            let mut neos: Vec<Neo> = Vec::new();

            for approach_data in res_json.close_approach_data {
                let neo = Neo {
                    id: NeoId(0),
                    api_id: res_json.id.parse().unwrap(),
                    designation: res_json.designation.parse().unwrap(),
                    diameter_min: res_json.estimated_diameter.miles.estimated_diameter_min,
                    diameter_max: res_json.estimated_diameter.miles.estimated_diameter_max,
                    hazardous_asteroid: res_json.is_potentially_hazardous_asteroid,
                    approach_date: approach_data.close_approach_date.clone(),
                    velocity: approach_data
                        .relative_velocity
                        .miles_per_hour
                        .parse()
                        .unwrap(),
                    miss_distance: approach_data.miss_distance.miles.parse().unwrap(),
                    orbiting_body: approach_data.orbiting_body.clone(),
                };

                self.add_neo(
                    neo.api_id.clone(),
                    neo.designation.clone(),
                    neo.diameter_min.clone(),
                    neo.diameter_max.clone(),
                    neo.hazardous_asteroid.clone(),
                    neo.approach_date.clone(),
                    neo.velocity.clone(),
                    neo.miss_distance.clone(),
                    neo.orbiting_body.clone(),
                )
                .await?;

                neos.push(neo);
            }

            Ok(neos)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
