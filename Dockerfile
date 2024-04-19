FROM docker.io/rust:bookworm

WORKDIR /usr/src/voter-fraud
COPY . .
RUN cargo install --path .

CMD [ "voter-fraud" ]
