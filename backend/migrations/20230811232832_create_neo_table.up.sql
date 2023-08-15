-- Add up migration script here
CREATE TABLE IF NOT EXISTS neos
(
    id  serial PRIMARY KEY,
    api_id INTEGER NOT NULL,
    designation VARCHAR(255) NOT NULL,
    diameter_min FLOAT4 NOT NULL,
    diameter_max FLOAT4 NOT NULL,
    is_potentially_hazardous_asteroid BOOLEAN NOT NULL,
    close_approach_date DATE NOT NULL,
    relative_velocity FLOAT4 NOT NULL,
    miss_distance FLOAT4 NOT NULL,
    orbiting_body VARCHAR(255) NOT NULL
)
