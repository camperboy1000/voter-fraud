FROM docker.io/rust:alpine AS builder
WORKDIR /usr/src/voter-fraud
RUN apk add openssl-dev pkgconf
COPY . .
RUN cargo install --path .

FROM alpine:3.19
RUN apk add openssl
COPY --from=builder /usr/local/cargo/bin/voter-fraud /usr/local/bin/voter-fraud
CMD [ "voter-fraud" ]
