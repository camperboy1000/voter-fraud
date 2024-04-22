FROM docker.io/library/rust:1.76-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates=20230311 libssl-dev=3.0.11-1~deb12u2 pkgconf=1.8.1-1 && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/voter-fraud
COPY . .
RUN cargo install --path .

FROM docker.io/library/debian:bookworm
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates=20230311 libssl3=3.0.11-1~deb12u2 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/voter-fraud /usr/local/bin/voter-fraud
CMD [ "voter-fraud" ]
