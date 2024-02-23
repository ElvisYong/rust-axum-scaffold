# Step 1: Setup the builder
FROM rust:latest as builder

# Create new cargo project and set it to the current directory
RUN USER=root cargo new --bin rust-axum-scaffold
WORKDIR /rust-axum-scaffold

# Build cargo crates
COPY Cargo.toml .
COPY Cargo.lock .

# Build cargo dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Build rust projects
COPY ./src ./src
RUN rm ./target/release/deps/rust_axum_scaffold*
RUN cargo build --release

# Step 2: Setup and run the runtime container
# Run time container
FROM photon:latest
ARG APP=/usr/src/app

RUN tdnf install -y ca-certificates shadow wget

# Set specific user for container security
ENV APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
  
COPY --from=builder /rust-axum-scaffold/target/release/rust-axum-scaffold ${APP}/rust-axum-scaffold

# Set the user to use when running this image
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER

# Open port 5000
EXPOSE 5000

WORKDIR ${APP}
CMD ["./rust-axum-scaffold"]