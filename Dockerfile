FROM rust AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ \
  && touch src/lib.rs

RUN cargo fetch
RUN cargo build --release

RUN rm src/lib.rs \
  && rmdir src/

COPY src ./src/
RUN cargo install --path . --offline

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/hello-rocket-async /usr/local/bin/

ENV ROCKET_ADDRESS 0.0.0.0
COPY entrypoint.sh /usr/local/bin/
ENTRYPOINT ["entrypoint.sh"]
CMD ["hello-rocket-async"]

