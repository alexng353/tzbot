FROM rust:latest

WORKDIR /usr/src/tzbot
COPY . .

RUN cargo install --path .

CMD ["tzbot"]

