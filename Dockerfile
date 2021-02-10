FROM rust:latest

EXPOSE 80

WORKDIR /usr/src/myapp
COPY . .
RUN rm Cargo.lock
RUN cargo build

CMD ["cargo", "run"]