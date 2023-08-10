# rust-axum-scaffold

Rust implementation demo app showcasing the use of Axum web framework for scaffold usage of backend apis built with Axum.

This project is heavily inspired by current realworld axum projects made by
[launchbadge](https://github.com/launchbadge/realworld-axum-sqlx/tree/main) and [JoeyMckenzie](https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx).

## What is included in this project

1. Axum server setup
2. [AWS DynamoDB](https://crates.io/crates/aws-sdk-dynamodb)
3. Openapi json generation with [Utoipa](https://crates.io/crates/utoipa)
4. Swagger-ui documentation page generated with [Utoipa-swagger](https://crates.io/crates/utoipa-swagger-ui)
5. Github Actions to run tests using [Schemathesis](https://github.com/schemathesis/schemathesis)

## Project Structure

This project follows the controller service repository pattern approach to building backend apis

- `controllers`
  - API entrypoints with `server.rs` holding all the routers together
  - Controllers are to hold their individual routers that will then be nested together in `server.rs`
- `domain`
  - Place to hold structs relating to parsing data grabbed from our database(`models`) and responses/requests to/from client(`view_models`)
  - Models is where we hold database related structs
- `repositories`
  - Database adapters or external adapters such as DynamoDb or RabbitMq should lie here
  - Only focus on returning pure data from source
- `services`
  - Contains core business logics utilizing repositories
  - E.g mutating, sorting, pagination are done here before sending back to the controllers
  - Services will be injected into controller handlers with a [`/services/service_register.rs`](src/services/service_register.rs) file.

### Notes

This project doesn't make use of many useful trait usage for the sake of simplicity, which results in being unable to work out mocks easily.

## Local Setup

### Clone this Repository

```
git clone https://github.com/ElvisYong/rust-axum-scaffold.git
cd rust-axum-scaffold
```

### Installing Rust and Cargo

Install [Rust](https://www.rust-lang.org/tools/install)

### Setup the environment

Create an env file according to .env.example

```
mkdir .env
```

### Building the application

```
cargo build
```

### Starting the application

```
cargo run
```

### Testing the application

```
cargo test
```

## Docker Setup

Build the docker image

```
docker build -t rust-axum-scaffold .
```

Run the image according to the environment port that you've opened, and fill up your environments

```
docker run -p 5000:5000 rust-axum-scaffold . -e YOUR_ENVIRONMENTS
```
