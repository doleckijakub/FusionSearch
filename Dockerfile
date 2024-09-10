FROM rust:latest

WORKDIR /opt/fusion

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run"]