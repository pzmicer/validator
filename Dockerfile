FROM rust:1.62-slim-bullseye as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install libpq-dev -y
RUN cargo build --release

FROM debian:bullseye-slim 
RUN apt-get update && apt-get install libpq5 -y
COPY --from=builder /app/target/release/xml-validator /xml-validator
CMD ["/xml-validator"]