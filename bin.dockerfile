FROM rust:1.66 as rs

RUN apt-get update && apt-get install -y build-essential \
  curl \
  openssl libssl-dev \
  pkg-config \
  python \
  valgrind \
  zlib1g-dev \
  cmake


FROM rs as planner
WORKDIR /usr/src/bin

RUN cargo install cargo-chef --locked

COPY . . 
RUN cargo chef prepare --recipe-path recipe.json


FROM rs as cacher
WORKDIR /usr/src/bin

RUN cargo install cargo-chef --locked

COPY --from=planner /usr/src/bin/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM rs as builder
WORKDIR /usr/src/bin

COPY --from=cacher /usr/src/bin/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

COPY . .
RUN cargo build --release


FROM rs as bin
COPY --from=builder /usr/src/bin/target/release/main /usr/local/bin/main

CMD main
