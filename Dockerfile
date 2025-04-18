FROM rust:1.86 as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/list/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release -bin rust_blog_mvc

FROM debian:bullyseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1       \
    && rm -rf /var/lib/apt/list/*


COPY --from=builder /usr/src/app/target/release/rust_blog_mvc /usr/local/bin
COPY --from=builder /usr/src/app/migrations /migrations

ENV DATABASE_URL=mysql://admin:shalom***@mysql/blog_db
ENV RUST_LOG=info
ENV APP_HOST=0.0.0.0
ENV APP_PORT=8080

EXPOSE 8080

CMD ["rust_blog_mvc"]