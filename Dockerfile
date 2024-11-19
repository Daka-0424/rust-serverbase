FROM rust:1.82.0-bookworm as base

ENV TZ=Asia/Tokyo

FROM base as builder
RUN cargo build --release
RUN strip /app/target/release/server-controller -o /server-controller

EXPOSE 8080
CMD ["/server-controller"]