FROM docker.io/rust:bookworm AS builder
WORKDIR /usr/src/voter-fraud
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo install --path .

FROM docker.io/debian:bookworm
RUN apt-get update && apt-get install -y openssl pkg-config && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/voter-fraud /usr/local/bin/voter-fraud
CMD [ "voter-fraud" ]
