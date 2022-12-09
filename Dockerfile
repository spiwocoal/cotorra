# Build image
# Necessary dependencies to build Parrot
FROM rustlang/rust:nightly-bullseye-slim as build

RUN apt-get update && apt-get install -y \
    build-essential autoconf automake cmake libtool libssl-dev pkg-config

WORKDIR "/parrot"

# Cache cargo build dependencies by creating a dummy source
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml ./
RUN cargo +nightly build --release -Z sparse-registry

COPY . .
RUN cargo +nightly build --release -Z sparse-registry

# Release image
# Necessary dependencies to run Parrot
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3-pip ffmpeg
RUN pip install -U yt-dlp

COPY --from=build /parrot/target/release/parrot .

CMD ["./parrot"]
