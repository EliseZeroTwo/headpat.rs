FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .
RUN rm Cargo.lock
RUN cargo build

CMD ["cargo", "run"]