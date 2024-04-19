FROM docker.io/rust:alpine AS builder
WORKDIR /usr/src/voter-fraud
COPY . .
RUN cargo install --path .

FROM alpine:3.19
COPY --from=builder /usr/local/cargo/bin/voter-fraud /usr/local/bin/voter-fraud
CMD [ "voter-fraud" ]
