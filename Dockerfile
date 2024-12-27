FROM rust:1.83.0-alpine

RUN apk add --no-cache \
    openssl \
    gcc \
    musl-dev \
    pkgconfig \
    libressl-dev

RUN cargo install cargo-watch

WORKDIR /app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ]