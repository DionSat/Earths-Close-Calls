DELETE FROM neos;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('neos', 'id'), 1, false);

INSERT INTO neos(api_id, designation, diameter_min, diameter_max, is_potentially_hazardous_asteroid, close_approach_date, relative_velocity, miss_distance, orbiting_body) VALUES (3542519, '2010 PK9', 0.0698081224, 0.156095707, true, DATE('1900-06-01'), 69201.9904887259, 4140648.4089846528, 'Merc');
