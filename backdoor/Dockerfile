FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/sami_rs ./target/release/sami_rs
CMD ["/target/release/sami_rs"]