FROM rust:1.62-slim-bullseye as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim 
# FROM debian:buster-slim
RUN apt-get update && apt-get install libpq5 -y
COPY --from=builder /app/target/release/xml-validator /xml-validator
CMD ["/xml-validator"]