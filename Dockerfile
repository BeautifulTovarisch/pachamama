FROM rust:1.40.0 as development

ENV environment=development

WORKDIR /app/src/pachamama

COPY src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo install cargo-watch

RUN cargo build

CMD [ "cargo", "watch", "-c", "-x", "check", "-x", "test" ]

FROM rust:1.40.0 as build

ENV environment=production

COPY src ./src

COPY Cargo.toml Cargo.lock ./

RUN cargo test
RUN cargo build --release
