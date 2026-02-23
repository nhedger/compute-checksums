FROM rust:1.87-bookworm AS build

WORKDIR /work

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release -p action

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /work/target/release/action /usr/local/bin/action

ENTRYPOINT ["/usr/local/bin/action"]
