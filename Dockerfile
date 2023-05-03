FROM rust:1.69.0 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder ./target/release/micro_post_service ./app/micro_post_service
COPY --from=builder ./Rocket.toml ./app/Rocket.toml
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_DATABASES='{postservice={url="mongodb://host.docker.internal:27017"}}'
EXPOSE 8000
CMD ["./app/micro_post_service"]
