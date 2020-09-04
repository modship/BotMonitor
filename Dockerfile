# -----------------
# Cargo Build Stage
# -----------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/app
COPY Cargo.lock .
COPY Cargo.toml .
#RUN mkdir .cargo
#RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final Stage
# -----------------

FROM debian:stable-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
ADD cert.cer /etc/ssl/certs/

COPY --from=cargo-build /usr/local/cargo/bin/bot_monitor /bin

CMD ["bot_monitor"]
