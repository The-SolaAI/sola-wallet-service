FROM rust:latest
WORKDIR /app/sola-wallet-service
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo fetch

EXPOSE 8000

RUN cargo build --release

CMD ["cargo", "run", "--release"]