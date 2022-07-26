FROM rust:latest
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666

WORKDIR /app
COPY . .

RUN rustup default nightly
CMD ["cargo", "run"]