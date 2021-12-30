FROM rust:1.57
WORKDIR /tonal-distancing

COPY . .
RUN cargo build --release -p server

FROM debian:bullseye-slim AS runtime
WORKDIR /tonal-distancing
COPY --from=builder /tonal-distancing/target/release/tonal-distancing /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tonal-distancing"]