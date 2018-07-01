# Remember The Milk (play ground app)

Test application to play with Rust language

# How to create DB via Diesel library
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
