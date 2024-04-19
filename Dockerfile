FROM docker.io/rust:bullseye

WORKDIR /usr/src/voter-fraud
COPY . .
RUN cargo install --path .

CMD [ "voter-fraud" ]
