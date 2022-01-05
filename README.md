Attendance System
=================

A system to track people's attendance at robotics.

## Backend
Backend is run using Rust with Actix Web. To run the backend run the command `MONGO_URI=<link to mongodb database> cargo run`.

## Frontend
The frontend uses React. To make changes run `npm run build` and reload the page served by the backend (defaults to `0.0.0.0:8080`).
To only test the frontend without the ability to make any `fetch` requests, run `npm run start`. This will hot reload the page.
