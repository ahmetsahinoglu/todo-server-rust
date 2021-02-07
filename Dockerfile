FROM rust:latest

WORKDIR /usr/src/todo-server-rust

COPY . .

ENV PORT 8080

RUN cargo install --path .

CMD ["todo-server-rust"]