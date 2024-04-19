FROM docker.io/rust:bookworm AS builder
WORKDIR /usr/src/voter-fraud
RUN apt update && apt install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo install --path .

FROM debian:bookworm
RUN apt update && apt install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/voter-fraud /usr/local/bin/voter-fraud
CMD [ "voter-fraud" ]
