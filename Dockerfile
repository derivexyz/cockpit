####################################################################################################
## Planner Stage using cargo-chef
####################################################################################################

FROM arm64v8/rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Builder Stage using cargo-chef
####################################################################################################

FROM chef AS builder
RUN rustup target add aarch64-unknown-linux-gnu
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-gnu

# RUN apt-get update && apt-get install musl-tools -y && apt-get install musl-dev -y

# WORKDIR /app
# COPY ./ .
#
# RUN cargo build --target aarch64-unknown-linux-gnu --release

####################################################################################################
## Final image
####################################################################################################
FROM arm64v8/rust:slim as runtime
RUN apt-get update && apt-get install libclang-dev -y
WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/lyra-maker ./
COPY --from=builder /app/.env* ./

CMD ["/bin/bash"]

