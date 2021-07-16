FROM rust:1.49
WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]
