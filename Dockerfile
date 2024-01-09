FROM rust:1.70-buster as builder
RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 8081
COPY --from=builder /app/target/release/iela /usr/bin

ENTRYPOINT ["app"]