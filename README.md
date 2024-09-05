# Shopping List App

## Brief

The customer has requested the following:

> As a healthcare company we have a keen eye on healthy eating and it’s been suggested by an employee that we have an easy way to keep track of what we need, what needs to be purchased and keep spending within the budget constraints. Below, are a number of stories which will achieve this objective.

They would like to see well structured, secure and clean code with adequate testing. From a development perspective I have the freedom to choose the tech stack.

The customer has provided 10 user stories to help guide the development process and fulfil the brief. The requirements are listed below 

### Story Requirements

1. ~~Create a shopping list that can contain a list of groceries~~
2. ~~Create a way for a user to add an item to the shopping list~~
3. Create a way for a user to remove an item from the shopping list
4. ~~Create a way for users to know what they have and haven't already picked up~~
5. ~~Persist shopping list state between page visits~~
6. Create a way for user to be able to change the order of items in their shopping list
7. Display the total cost for the whole shop
8. Put a spending limit in place alerting the user when they go over the limit
9. Add functionality to send the shopping list via email
10. Add a login system to persist shopping lists for different users


## Technologies 

### Backend

- Rust 
    - highly performant and safe
    - statically typed
    - personal programming language of choice
    - Tokio and Serde crates make async and managing data structures simple
    - Warp, fast & elegant web framework
- SQLX (PostgreSQL) 
    - supports both SQL and JSON querying
    - Opensource database server
    - first time use


### Frontend

- Native Web Components
    - Framework less
    - TS was suggested to me as it is statically typed and I have some familiarity with JS.
    - NativeDOM components
    - CSS&HTML

### Structure

Based on a todo application I recently built.

```
.
├── backend
│   ├── sql
│   └── src
│       ├── _tests
│       ├── model
│       ├── security
│       └── web
└── frontend
    ├── src
    │   ├── model
    │   └── ui
    └── web-folder
        └── css
```

---

## Setup

May need to install `docker`, `rustup` and `cargo watch`.  

### DB

```sh
# Start the database - from /backend
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14
```

### Dev Test 

```sh
# Test for db
cargo watch -q -c -w src/ -x 'test model_db -- --test-threads=1 --nocapture'
```

```sh
# Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'
```

```sh
# Test for web                      v-- this is a filter
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```

```sh
# All backend tests
cargo watch -q -c -w src/ -x 'test -- --test-threads=1 --nocapture'
```

### Dev Web

```sh
# Serve the front end
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```

### FRONTEND

Before proceeding with building the frontend make sure you are using an up-to-date version of npm: 

```sh
npm install
npm i -D tslib
npm run build -- -w
```

---

## Todo

- [ ] Add delete button to envoke DELETE method on grocery item
- [ ] Add index property to grocery items, look into click and drag items (on mouse up update db)
- [ ] Display cost per item, ensure user input for cost is stored in db, display total cost
- [ ] Allow user to configure spend limit and change total display to alert user to overspend
- [ ] Look into email client and sending shopping list
- [ ] User log in system, email address, cid, and spend limit can all be stored in user row