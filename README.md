# Earths-Close-Calls

 This is a Nasa API accesspoint for querying and storing Near Earth Objects from the Asteroids - NeoWs API.

 ## API NASA Asteroids - NeoWs
NeoWs (Near Earth Object Web Service) is a RESTful web service for near earth Asteroid information. With NeoWs a user can: search for Asteroids based on their closest approach date to Earth, lookup a specific Asteroid with its NASA JPL small body id, as well as browse the overall data-set.

Data-set: All the data is from the NASA JPL Asteroid team (http://neo.jpl.nasa.gov/).

## How to Run
Make sure you have docker installed

**Windows**

In the ```docker-compose.yml``` leave the 

```extra_hosts: - host.docker.internal:host-gateway```

**Linux/Mac**

In the ```docker-compose.yml``` remove the

```extra_hosts: - host.docker.internal:host-gateway```

Then run ```docker compose up -d``` to run docker compose to start up the postgres container

then ```cd``` into the backed and migrate and seed the database with
```sqlx migrate run```
```sqlx migrate run --source ./fixtures --ignore-missing```

then run ```cargo run``` to start the backend

Visiting ```localhost:3000``` on any browser should bring up the homepage.

## How to Use
On the homepage the user will be prompted for their login. 

Signing in will update the page to show the dashboard for accessing the API

The dashboard includes an option to **View NEO by Date Range** and **View NEO by ID**.

Upon selecting either one the user will be redirected to the results page. 

The database will be queried first for the corresponding result. If the rows are returned as empty then the API will be called and the results will stored in the database and returned to the user.

### If the User is Admin
If the user is admin their will be an extra option on the dashboard to go to the admin page.

Here there will be a list of users email and hashed passwords. 

On the bottom is an option to ban a user by their email

## Problems
One issues I ran into was using the html forms to post to the backend endpoint. Whenever a form was submitted the url with updates. So the issue was that the page page kept changing to a page that has no html rendering. I wasn't very sure if javascript was allowed on this project. I could of easily fixed this with javascript or jQuery.

Another issue banning a user. I couldn't figure out to get this to work without calling an endpoint to do it. The reason I call this a problem is because I dont believe that a function as important as ban should have an endpoint to be accessed. I just made it an endpoint because that was the only way I could of think of to get the html to speak to communicate to the backend.

One other issue I ran into was deserializing the NeoW API by date range. Mainly because one of the inner objects had fields by date and not name. So i couldn't create a struct to represent the structure of the json because that objects fields were different each time.

## What I Learned
I learned how to be flexible with tera templates to take into account whether a user is loggin, admin, or banned and which content to show them depending on that. At first I was confused on how to make templates more flexible but that states or context that they have make it as lot easier to change the page depending the state given. 

I learned how to use serde to serialize and deserialize json into objects and vice versa using structs. I found this confusing at first but once i figured out how to structures a struct to a json object it became easier to deserialize and access its members.

I learned how to use sqlx to add migrations to my database and how to use sqlx to also seed my data base as well.

I learned how to use axum extractors to help my extract fields such as dates and ids to service the api
