FROM rust:1.67 AS builder

WORKDIR /usr/src/publisher

COPY Cargo.toml .

RUN apt-get update \
    && apt-get install -y cmake build-essential libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY src /usr/src/publisher/src

RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    rm -rf /var/lib/apt/lists/* 

COPY --from=builder /usr/src/publisher/target/release/publisher /usr/local/bin/publisher

CMD ["publisher"]
