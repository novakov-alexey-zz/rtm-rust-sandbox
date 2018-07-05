# Remember The Milk (play ground app)

Test application to play with Rust language

# 1. Switch to Rust nightly
```sh
rustup install nightly
rustup override add nightly
```

# 2. Create DB via Diesel library
## Create Postgres instance

Run shell script of this project `start-db.sh. It will create new Postgres Docker container

## Install diesel 
```sh
cargo install diesel_cli --no-default-features --features postgres
```

## Create database in Postgres
```sh
diesel setup
```

## Apply migration scripts to the database 
```sh
diesel migration run
```

# 3. Start RTM
```sh
cargo run 
```

#4. Run Integration Tests
```sh
cargo test 
```

#5. Call RTM app
```sh
curl localhost:8000/api/tasks/inbox/true 
```