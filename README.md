# Remember The Milk (play ground app)

Test application to play with Rust language including database and web layers

### 1. Switch to Rust nightly
```sh
rustup install nightly
rustup override add nightly
```

### 2. Create DB via Diesel library
#### Create Postgres instance

Run shell script of this project *start-db.sh*. It will create new Postgres Docker container

#### Install diesel 
```sh
cargo install diesel_cli --no-default-features --features postgres
```

#### Create database in Postgres
```sh
diesel setup
```

#### Apply migration scripts to the database 
```sh
diesel migration run
```

### 3. Start RTM
```sh
cargo run 
```
To suppress the warnings due to Rust compiler change in macro resolution:
```bash
export RUSTFLAGS="-Aproc-macro-derive-resolution-fallback"
```
It won't be needed after diesel library adopt itself to this compiler marcos related change.

### 4. Run Integration Tests
```sh
cargo test 
```

### 5. Call RTM app
```sh
curl localhost:8000/api/tasks/inbox/true 
```

# Docker image build

Run the following to compile and build a Docker image:
```bash
sh docker-image.sh
```

# Docker compose

Run standard docker-compose to start and stop containers.

Start containers:
```bash
docker-compose up -d
```

Stop containers:
```bash
docker-compose down
```