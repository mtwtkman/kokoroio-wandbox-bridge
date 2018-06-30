FROM liuchong/rustup:stable

RUN mkdir /app
WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

RUN rustup install nightly
RUN rustup default nightly