FROM rust:1.57
WORKDIR /tonal-distancing

COPY . .
RUN cargo build --release -p server

CMD ./target/release/tonal-distancing
